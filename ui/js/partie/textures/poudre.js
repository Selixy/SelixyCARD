import { hasard } from '../../decor/papiers/outils.js';
import { HAUTEUR_TEXTURE, LARGEUR_TEXTURE } from './fabrique.js';

const VARIANTES = 4;
const SYMBOLES = ['★', '+4', '✦', '⊘', '⇄', 'J'];

function hachuresCraie(ctx, alea) {
  const nbLignes = 28;
  for (let i = 0; i < nbLignes; i++) {
    const y = 35 + i * (314 / nbLignes) + (alea() - 0.5) * 8;
    const x1 = 28 + alea() * 18;
    const x2 = LARGEUR_TEXTURE - 28 - alea() * 18;
    ctx.save();
    ctx.strokeStyle = '#ffffff';
    ctx.lineWidth = 10 + alea() * 12;
    ctx.lineCap = 'round';
    ctx.globalAlpha = 0.35 + 0.45 * alea();
    ctx.beginPath();
    ctx.moveTo(x1, y);
    ctx.lineTo(x2, y + (alea() - 0.5) * 10);
    ctx.stroke();
    ctx.restore();
  }
}

function frottisGrain(ctx, alea) {
  const n = 2800;
  for (let i = 0; i < n; i++) {
    const x = 20 + alea() * (LARGEUR_TEXTURE - 40);
    const y = 25 + alea() * (HAUTEUR_TEXTURE - 50);
    ctx.fillStyle = '#ffffff';
    ctx.globalAlpha = 0.12 + 0.5 * alea();
    const t = 1.0 + 2.8 * alea();
    ctx.fillRect(x, y, t, t);
  }
}

function dessinerSymboles(ctx, alea) {
  const nb = 4 + Math.floor(alea() * 3);
  for (let i = 0; i < nb; i++) {
    const sym = SYMBOLES[Math.floor(alea() * SYMBOLES.length)];
    const x = 45 + alea() * (LARGEUR_TEXTURE - 90);
    const y = 55 + alea() * (HAUTEUR_TEXTURE - 110);
    const taille = 24 + alea() * 22;
    ctx.save();
    ctx.translate(x, y);
    ctx.rotate((alea() - 0.5) * 0.7);
    ctx.font = `900 ${taille}px sans-serif`;
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    ctx.fillStyle = '#ffffff';
    ctx.globalAlpha = 0.5 + 0.45 * alea();
    ctx.fillText(sym, 0, 0);
    ctx.restore();
  }
}

function peindreTextureCraie(variante) {
  const toile = document.createElement('canvas');
  toile.width = LARGEUR_TEXTURE;
  toile.height = HAUTEUR_TEXTURE;
  const ctx = toile.getContext('2d');
  const alea = hasard(42 + variante * 107);
  hachuresCraie(ctx, alea);
  dessinerSymboles(ctx, alea);
  frottisGrain(ctx, alea);
  return toile;
}

export async function chargerPoudres(moteur) {
  for (let v = 0; v < VARIANTES; v++) {
    moteur.charger_texture(`poudre:${v}`, peindreTextureCraie(v));
  }
}
