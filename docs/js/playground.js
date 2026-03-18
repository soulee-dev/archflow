import { examples } from './examples.js';
import { registerArchflowLanguage, setupDiagnostics } from './monaco-archflow.js';

let editor = null;
let renderSvgFn = null;   // wasm.render_svg (JSON → SVG)
let renderDslFn = null;   // wasm.render_dsl (DSL → SVG)
let parseDslFn = null;    // wasm.parse_dsl (DSL → JSON)
let renderTimeout = null;
let mode = 'dsl'; // 'dsl' or 'json'

// Pan & Zoom state
let scale = 1;
let panX = 0;
let panY = 0;
let isPanning = false;
let startX = 0;
let startY = 0;

// Icon cache (in-memory) — acts as the "disk cache" equivalent for the browser
const iconCache = new Map();

// Central registry — always tried as fallback (same as Python resolver)
const DEFAULT_REGISTRY = (location.hostname === 'localhost' || location.hostname === '127.0.0.1')
  ? location.origin
  : 'https://raw.githubusercontent.com/soulee-dev/archflow-icons/main';

export function initPlayground(wasmModule) {
  renderSvgFn = wasmModule.render_svg;
  renderDslFn = wasmModule.render_dsl;
  parseDslFn = wasmModule.parse_dsl;

  // Register archflow language for Monaco
  registerArchflowLanguage(monaco);

  // Determine initial content
  const shared = loadFromURL();
  const initialValue = shared || examples[0].dsl;

  // Init Monaco Editor
  editor = monaco.editor.create(document.getElementById('editor-container'), {
    value: initialValue,
    language: 'archflow',
    theme: 'vs-dark',
    fontSize: 13,
    fontFamily: "'JetBrains Mono', 'Fira Code', monospace",
    minimap: { enabled: false },
    lineNumbers: 'on',
    scrollBeyondLastLine: false,
    automaticLayout: true,
    tabSize: 2,
    wordWrap: 'on',
    bracketPairColorization: { enabled: true },
    padding: { top: 8, bottom: 8 },
  });

  // Setup diagnostics (error markers from WASM parse_dsl)
  setupDiagnostics(monaco, editor, parseDslFn);

  // Auto-render on change
  editor.onDidChangeModelContent(() => {
    clearTimeout(renderTimeout);
    renderTimeout = setTimeout(render, 300);
  });

  // Example selector
  const select = document.getElementById('example-select');
  examples.forEach((ex, i) => {
    const opt = document.createElement('option');
    opt.value = i;
    opt.textContent = ex.name;
    select.appendChild(opt);
  });

  select.addEventListener('change', () => {
    const ex = examples[select.value];
    if (ex) {
      if (mode === 'dsl') {
        editor.setValue(ex.dsl);
      } else {
        const json = parseDslFn(ex.dsl);
        const ir = JSON.parse(json);
        editor.setValue(JSON.stringify(ir, null, 2));
      }
    }
  });

  // Theme selector
  document.getElementById('theme-select').addEventListener('change', (e) => {
    applyTheme(e.target.value);
  });

  // Mode toggle
  document.getElementById('mode-btn').addEventListener('click', toggleMode);

  // Render button
  document.getElementById('render-btn').addEventListener('click', render);

  // Download button
  document.getElementById('download-btn').addEventListener('click', downloadSvg);

  // Share button
  const shareBtn = document.getElementById('share-btn');
  if (shareBtn) shareBtn.addEventListener('click', shareDiagram);

  // Zoom controls
  document.getElementById('zoom-in').addEventListener('click', () => zoom(1.25));
  document.getElementById('zoom-out').addEventListener('click', () => zoom(0.8));
  document.getElementById('zoom-fit').addEventListener('click', fitToView);
  document.getElementById('zoom-reset').addEventListener('click', resetView);

  // Pan & Zoom on preview panel
  const panel = document.querySelector('.preview-panel');
  panel.addEventListener('wheel', onWheel, { passive: false });
  panel.addEventListener('mousedown', onMouseDown);
  window.addEventListener('mousemove', onMouseMove);
  window.addEventListener('mouseup', onMouseUp);
}

// ─── Icon & Style Resolution ───

const manifestCache = new Map();

function resolveSourceBase(source) {
  if (!source) return DEFAULT_REGISTRY;
  const ghMatch = source.match(/^github:(.+\/.+)$/);
  if (ghMatch) {
    return `https://raw.githubusercontent.com/${ghMatch[1]}/main`;
  }
  if (source.startsWith('https://') || source.startsWith('http://')) {
    return source.replace(/\/$/, '');
  }
  return DEFAULT_REGISTRY;
}

async function fetchManifest(provider, baseUrl) {
  const key = `${baseUrl}/${provider}/manifest`;
  if (manifestCache.has(key)) return manifestCache.get(key);

  const url = `${baseUrl}/${provider}/manifest.json`;
  try {
    const resp = await fetch(url);
    if (!resp.ok) { manifestCache.set(key, null); return null; }
    const manifest = await resp.json();
    manifestCache.set(key, manifest);
    return manifest;
  } catch {
    manifestCache.set(key, null);
    return null;
  }
}

async function resolveIcons(ir) {
  const providerSources = (ir.metadata && ir.metadata.provider_sources) || {};
  const declaredProviders = new Set(Object.keys(providerSources));

  if (declaredProviders.size === 0) return ir;

  const providerBaseUrls = {};
  const manifests = {};
  await Promise.all([...declaredProviders].map(async p => {
    const base = resolveSourceBase(providerSources[p]);
    providerBaseUrls[p] = base;
    manifests[p] = await fetchManifest(p, base);
  }));

  // Apply cluster_styles from manifests
  for (const cluster of (ir.clusters || [])) {
    if (!cluster.provider || !cluster.cluster_type) continue;
    if (!declaredProviders.has(cluster.provider)) continue;
    if (cluster.style) continue;
    const mf = manifests[cluster.provider];
    const preset = mf && mf.cluster_styles && mf.cluster_styles[cluster.cluster_type];
    if (preset) {
      cluster.style = { ...preset };
    }
  }

  // Apply node_render_modes from manifests
  const renderModes = {};
  for (const [p, mf] of Object.entries(manifests)) {
    if (mf && mf.node_render_mode) renderModes[p] = mf.node_render_mode;
  }
  if (Object.keys(renderModes).length > 0) {
    if (!ir.metadata) ir.metadata = {};
    ir.metadata.node_render_modes = renderModes;
  }

  // Resolve node icons
  const resolveQueue = [];
  for (const node of (ir.nodes || [])) {
    const provider = node.provider;
    const icon = node.icon;
    if (!provider || !icon) continue;
    if (!declaredProviders.has(provider)) continue;

    const base = providerBaseUrls[provider];
    resolveQueue.push({ node, urls: [`${base}/${provider}/nodes/${icon}.svg`] });
  }

  // Resolve cluster icons
  for (const cluster of (ir.clusters || [])) {
    const provider = cluster.provider;
    const clusterType = cluster.cluster_type;
    if (!provider || !clusterType) continue;
    if (!declaredProviders.has(provider)) continue;

    const base = providerBaseUrls[provider];
    resolveQueue.push({ node: cluster, urls: [`${base}/${provider}/clusters/${clusterType}.svg`] });
  }

  // Fetch all icons in parallel
  await Promise.allSettled(
    resolveQueue.map(({ node, urls }) => resolveNodeIcon(node, urls))
  );

  return ir;
}

async function resolveNodeIcon(node, urls) {
  for (const url of urls) {
    const svg = await fetchIcon(url);
    if (svg) {
      node.icon_svg = svg;
      return;
    }
  }
}

async function fetchIcon(url) {
  if (iconCache.has(url)) {
    return iconCache.get(url);
  }
  try {
    const resp = await fetch(url);
    if (!resp.ok) {
      iconCache.set(url, null);
      return null;
    }
    let svg = await resp.text();
    svg = svg.replace(/<script[\s\S]*?<\/script>/gi, '');
    svg = svg.replace(/\bon\w+\s*=\s*["'][^"']*["']/gi, '');
    iconCache.set(url, svg);
    return svg;
  } catch {
    iconCache.set(url, null);
    return null;
  }
}

// ─── Pan & Zoom ───

function applyTransform() {
  const preview = document.getElementById('svg-preview');
  preview.style.transform = `translate(${panX}px, ${panY}px) scale(${scale})`;
  updateZoomLabel();
}

function updateZoomLabel() {
  const label = document.getElementById('zoom-level');
  if (label) label.textContent = Math.round(scale * 100) + '%';
}

function zoom(factor, cx, cy) {
  const panel = document.querySelector('.preview-panel');
  if (cx === undefined) {
    cx = panel.clientWidth / 2;
    cy = panel.clientHeight / 2;
  }

  const newScale = Math.min(Math.max(scale * factor, 0.1), 5);
  const ratio = newScale / scale;

  panX = cx - ratio * (cx - panX);
  panY = cy - ratio * (cy - panY);
  scale = newScale;

  applyTransform();
}

function fitToView() {
  const panel = document.querySelector('.preview-panel');
  const preview = document.getElementById('svg-preview');
  const svgEl = preview.querySelector('svg');
  if (!svgEl) return;

  const vb = svgEl.getAttribute('viewBox');
  if (!vb) return;
  const [, , vw, vh] = vb.split(/\s+/).map(Number);
  if (!vw || !vh) return;

  const pw = panel.clientWidth - 48;
  const ph = panel.clientHeight - 48;

  scale = Math.min(pw / vw, ph / vh, 1.5);
  scale = Math.max(scale, 0.1);
  panX = (panel.clientWidth - vw * scale) / 2;
  panY = (panel.clientHeight - vh * scale) / 2;

  applyTransform();
}

function resetView() {
  scale = 1;
  panX = 0;
  panY = 0;
  applyTransform();
}

function onWheel(e) {
  e.preventDefault();
  const rect = e.currentTarget.getBoundingClientRect();
  const cx = e.clientX - rect.left;
  const cy = e.clientY - rect.top;
  const factor = e.deltaY < 0 ? 1.1 : 0.9;
  zoom(factor, cx, cy);
}

function onMouseDown(e) {
  if (e.button === 1 || (e.button === 0 && (e.target.closest('.preview-panel') && !e.target.closest('.zoom-controls')))) {
    isPanning = true;
    startX = e.clientX - panX;
    startY = e.clientY - panY;
    e.currentTarget.style.cursor = 'grabbing';
    e.preventDefault();
  }
}

function onMouseMove(e) {
  if (!isPanning) return;
  panX = e.clientX - startX;
  panY = e.clientY - startY;
  applyTransform();
}

function onMouseUp() {
  if (isPanning) {
    isPanning = false;
    const panel = document.querySelector('.preview-panel');
    if (panel) panel.style.cursor = '';
  }
}

// ─── Theme ───

function applyTheme(themeName) {
  render();
}

// ─── Mode Toggle ───

function toggleMode() {
  const modeBtn = document.getElementById('mode-btn');
  const content = editor.getValue();

  if (mode === 'dsl') {
    try {
      const json = parseDslFn(content);
      const ir = JSON.parse(json);
      mode = 'json';
      const model = editor.getModel();
      monaco.editor.setModelLanguage(model, 'json');
      editor.setValue(JSON.stringify(ir, null, 2));
      modeBtn.textContent = 'JSON';
      modeBtn.title = 'Switch to DSL mode';
    } catch (e) {
      setStatus('Parse error: ' + e.message, true);
    }
  } else {
    mode = 'dsl';
    const model = editor.getModel();
    monaco.editor.setModelLanguage(model, 'archflow');
    const select = document.getElementById('example-select');
    const ex = examples[select.value];
    if (ex) {
      editor.setValue(ex.dsl);
    }
    modeBtn.textContent = 'DSL';
    modeBtn.title = 'Switch to JSON mode';
  }
}

// ─── Render ───

async function render() {
  if (!renderSvgFn || !editor) return;

  const content = editor.getValue();

  try {
    let svg;
    if (mode === 'dsl') {
      const irJson = parseDslFn(content);
      const ir = JSON.parse(irJson);

      const selectedTheme = document.getElementById('theme-select').value;
      if (!ir.metadata) ir.metadata = {};
      ir.metadata.theme = selectedTheme;

      await resolveIcons(ir);

      svg = renderSvgFn(JSON.stringify(ir));
    } else {
      const ir = JSON.parse(content);
      const selectedTheme = document.getElementById('theme-select').value;
      if (!ir.metadata) ir.metadata = {};
      ir.metadata.theme = selectedTheme;

      await resolveIcons(ir);
      svg = renderSvgFn(JSON.stringify(ir));
    }

    const preview = document.getElementById('svg-preview');
    preview.innerHTML = svg;
    setStatus('Ready', false);

    const svgEl = preview.querySelector('svg');
    if (svgEl) {
      svgEl.style.display = 'block';
    }

    requestAnimationFrame(() => requestAnimationFrame(fitToView));
  } catch (e) {
    setStatus(e.toString(), true);
  }
}

function setStatus(text, isError) {
  const status = document.getElementById('status');
  status.textContent = text;
  status.className = isError ? 'status error' : 'status success';
}

function downloadSvg() {
  const preview = document.getElementById('svg-preview');
  const svg = preview.innerHTML;
  if (!svg) return;

  const blob = new Blob([svg], { type: 'image/svg+xml' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = 'archflow-diagram.svg';
  a.click();
  URL.revokeObjectURL(url);
}

// ─── Share ───

function encodeContent(str) {
  return btoa(unescape(encodeURIComponent(str)));
}

function decodeContent(encoded) {
  return decodeURIComponent(escape(atob(encoded)));
}

function shareDiagram() {
  if (!editor) return;
  const content = editor.getValue();
  const encoded = encodeContent(content);
  const url = `${location.origin}${location.pathname}#playground/${mode}/${encoded}`;

  navigator.clipboard.writeText(url).then(() => {
    setStatus('Share URL copied to clipboard!', false);
  }).catch(() => {
    prompt('Share URL:', url);
  });

  history.replaceState(null, '', `#playground/${mode}/${encoded}`);
}

function loadFromURL() {
  const hash = location.hash;
  if (!hash.startsWith('#playground/')) return null;

  const parts = hash.substring('#playground/'.length);
  const slashIdx = parts.indexOf('/');
  if (slashIdx < 0) return null;

  const urlMode = parts.substring(0, slashIdx);
  const encoded = parts.substring(slashIdx + 1);

  try {
    const content = decodeContent(encoded);

    if (urlMode === 'json' && mode === 'dsl') {
      mode = 'json';
      const modeBtn = document.getElementById('mode-btn');
      if (modeBtn) {
        modeBtn.textContent = 'JSON';
        modeBtn.title = 'Switch to DSL mode';
      }
    }

    return content;
  } catch {
    return null;
  }
}
