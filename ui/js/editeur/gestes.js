import { echelleAffichage } from './vue-recadrage.js';

const distance = (a, b) => Math.hypot(a.x - b.x, a.y - b.y);

export function brancherGestes(canvas, etat) {
  const doigts = new Map();
  const pixelsParCanvas = () => canvas.width / canvas.getBoundingClientRect().width;
  canvas.style.touchAction = 'none';
  canvas.addEventListener('pointerdown', (e) => {
    canvas.setPointerCapture(e.pointerId);
    doigts.set(e.pointerId, { x: e.clientX, y: e.clientY });
  });
  canvas.addEventListener('pointermove', (e) => {
    const image = etat.image();
    if (!image || !doigts.has(e.pointerId)) return;
    const avant = [...doigts.values()];
    doigts.set(e.pointerId, { x: e.clientX, y: e.clientY });
    const apres = [...doigts.values()];
    if (apres.length === 2) {
      etat.zoomer(distance(apres[0], apres[1]) / distance(avant[0], avant[1]));
      return;
    }
    const echelle = echelleAffichage(image, canvas) / pixelsParCanvas();
    etat.deplacer((e.movementX || 0) / echelle, (e.movementY || 0) / echelle);
  });
  const fin = (e) => doigts.delete(e.pointerId);
  canvas.addEventListener('pointerup', fin);
  canvas.addEventListener('pointercancel', fin);
  canvas.addEventListener('wheel', (e) => {
    e.preventDefault();
    etat.zoomer(Math.exp(-e.deltaY * 0.0015));
  }, { passive: false });
}
