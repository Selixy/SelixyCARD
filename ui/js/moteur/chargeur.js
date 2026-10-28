import init, { Moteur } from '../../wasm/moteur.js';

function cloner(canvas) {
  const propre = canvas.cloneNode(false);
  canvas.replaceWith(propre);
  return propre;
}

async function construire(canvas) {
  await init();
  try {
    return await Moteur.creer(canvas);
  } catch {
    return await Moteur.creer(cloner(canvas));
  }
}

export async function creerMoteur(canvas, _rendu) {
  const moteur = await construire(canvas);
  moteur.definir_theme(document.body?.dataset.scene ?? 'ardoise');
  return moteur;
}
