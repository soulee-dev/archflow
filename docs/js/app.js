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

// If URL has #playground/... hash, switch to playground section
if (location.hash.startsWith('#playground/')) {
  document.querySelectorAll('.section').forEach(s => s.classList.remove('active'));
  const pg = document.getElementById('playground');
  if (pg) pg.classList.add('active');
  document.querySelectorAll('[data-nav]').forEach(l => {
    l.classList.toggle('active', l.getAttribute('data-nav') === 'playground');
  });
}

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
