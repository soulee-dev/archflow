import { examples } from './examples.js';
import { parseDSL } from './parser.js';

let editor = null;
let renderFn = null;
let renderTimeout = null;
let mode = 'dsl'; // 'dsl' or 'json'

// Pan & Zoom state
let scale = 1;
let panX = 0;
let panY = 0;
let isPanning = false;
let startX = 0;
let startY = 0;

export function initPlayground(wasmRenderSvg) {
  renderFn = wasmRenderSvg;

  // Init CodeMirror
  const textarea = document.getElementById('editor-textarea');
  editor = CodeMirror.fromTextArea(textarea, {
    mode: 'archflow',
    theme: 'material-darker',
    lineNumbers: true,
    matchBrackets: true,
    autoCloseBrackets: true,
    tabSize: 2,
    lineWrapping: true,
  });

  editor.setSize('100%', '100%');

  // Auto-render on change
  editor.on('change', () => {
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
        const ir = parseDSL(ex.dsl);
        editor.setValue(JSON.stringify(ir, null, 2));
      }
    }
  });

  // Mode toggle
  document.getElementById('mode-btn').addEventListener('click', toggleMode);

  // Render button
  document.getElementById('render-btn').addEventListener('click', render);

  // Download button
  document.getElementById('download-btn').addEventListener('click', downloadSvg);

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

  // Load first example in DSL mode
  editor.setValue(examples[0].dsl);
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

  // Zoom toward cursor position
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

  const pw = panel.clientWidth - 48;
  const ph = panel.clientHeight - 48;

  scale = Math.min(pw / vw, ph / vh, 1.5);
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
  // Middle-click or left-click on panel background
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

// ─── Mode Toggle ───

function toggleMode() {
  const modeBtn = document.getElementById('mode-btn');
  const content = editor.getValue();

  if (mode === 'dsl') {
    try {
      const ir = parseDSL(content);
      mode = 'json';
      editor.setOption('mode', 'application/json');
      editor.setValue(JSON.stringify(ir, null, 2));
      modeBtn.textContent = 'JSON';
      modeBtn.title = 'Switch to DSL mode';
    } catch (e) {
      setStatus('Parse error: ' + e.message, true);
    }
  } else {
    mode = 'dsl';
    editor.setOption('mode', 'archflow');
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

function render() {
  if (!renderFn || !editor) return;

  const content = editor.getValue();
  let jsonStr;

  try {
    if (mode === 'dsl') {
      const ir = parseDSL(content);
      jsonStr = JSON.stringify(ir);
    } else {
      JSON.parse(content);
      jsonStr = content;
    }
  } catch (e) {
    setStatus('Parse error: ' + e.message, true);
    return;
  }

  try {
    const svg = renderFn(jsonStr);
    const preview = document.getElementById('svg-preview');
    preview.innerHTML = svg;
    setStatus('Ready', false);

    // Keep SVG at native size for proper pan/zoom
    const svgEl = preview.querySelector('svg');
    if (svgEl) {
      svgEl.style.display = 'block';
    }

    // Fit on first render
    setTimeout(fitToView, 10);
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
