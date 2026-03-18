import { initPlayground } from './playground.js';

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

// Init WASM + Playground
async function init() {
  const statusEl = document.getElementById('status');
  if (statusEl) {
    statusEl.textContent = 'Loading WASM...';
  }

  try {
    const wasm = await import('../pkg/archflow_wasm.js');
    await wasm.default();
    initPlayground(wasm.render_svg);
  } catch (err) {
    console.error('WASM load failed:', err);
    if (statusEl) {
      statusEl.textContent = 'Failed to load WASM engine: ' + err.message;
      statusEl.className = 'status error';
    }
  }
}

init();
