// Navigation
document.querySelectorAll('[data-nav]').forEach(link => {
  link.addEventListener('click', (e) => {
    e.preventDefault();
    const target = link.getAttribute('data-nav');
    document.querySelectorAll('.section').forEach(s => s.classList.remove('active'));
    document.getElementById(target).classList.add('active');
    document.querySelectorAll('[data-nav]').forEach(l => l.classList.remove('active'));
    link.classList.add('active');
  });
});

// If URL has #playground/... hash, switch to playground section
if (location.hash.startsWith('#playground/')) {
  document.querySelectorAll('.section').forEach(s => s.classList.remove('active'));
  const pg = document.getElementById('playground');
  if (pg) pg.classList.add('active');
  document.querySelectorAll('[data-nav]').forEach(l => {
    l.classList.toggle('active', l.getAttribute('data-nav') === 'playground');
  });
}

// Load Monaco Editor via AMD loader, then init WASM + Playground
function loadMonaco() {
  return new Promise((resolve, reject) => {
    const script = document.createElement('script');
    script.src = 'https://cdn.jsdelivr.net/npm/monaco-editor@0.52.2/min/vs/loader.js';
    script.onload = () => {
      // Monaco's AMD require — save reference before it can conflict
      const monacoRequire = window.require;
      monacoRequire.config({
        paths: { vs: 'https://cdn.jsdelivr.net/npm/monaco-editor@0.52.2/min/vs' },
      });
      monacoRequire(['vs/editor/editor.main'], () => resolve());
    };
    script.onerror = reject;
    document.head.appendChild(script);
  });
}

async function init() {
  const statusEl = document.getElementById('status');
  if (statusEl) {
    statusEl.textContent = 'Loading editor...';
  }

  try {
    // Load Monaco first
    await loadMonaco();

    if (statusEl) {
      statusEl.textContent = 'Loading WASM...';
    }

    // Load WASM
    const wasm = await import('../pkg/archflow_wasm.js');
    await wasm.default();

    // Init playground (dynamic import to ensure Monaco globals are ready)
    const { initPlayground } = await import('./playground.js');
    initPlayground(wasm);
  } catch (err) {
    console.error('Init failed:', err);
    if (statusEl) {
      statusEl.textContent = 'Failed to load: ' + err.message;
      statusEl.className = 'status error';
    }
  }
}

init();
