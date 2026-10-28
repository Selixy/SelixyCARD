import { AQUARELLES, CREME, ENCRE, hasard, tache, trait } from './outils.js';

function etoile(ctx, x, y, r) {
  const points = [];
  for (let i = 0; i <= 10; i++) {
    const rayon = i % 2 ? r * 0.45 : r;
    const a = (i * Math.PI) / 5 - Math.PI / 2;
    points.push([x + Math.cos(a) * rayon, y + Math.sin(a) * rayon]);
  }
  trait(ctx, points, 4, ENCRE);
}

function lune(ctx, x, y, r) {
  ctx.strokeStyle = ENCRE;
  ctx.lineWidth = 4;
  ctx.beginPath();
  ctx.arc(x, y, r, 0.6, Math.PI * 2 - 0.6);
  ctx.arc(x + r * 0.45, y, r * 0.8, Math.PI * 2 - 0.9, 0.9, true);
  ctx.stroke();
}

function feuille(ctx, x, y, r) {
  trait(ctx, [[x, y + r], [x - r * 0.5, y], [x, y - r], [x + r * 0.5, y], [x, y + r]], 4, ENCRE);
  trait(ctx, [[x, y + r * 1.2], [x, y - r * 0.6]], 3, ENCRE);
}

const MOTIFS = [etoile, lune, feuille];

export function creerIllustration(variante, largeur = 256, hauteur = 384) {
  const toile = document.createElement('canvas');
  toile.width = largeur;
  toile.height = hauteur;
  const ctx = toile.getContext('2d');
  const alea = hasard(21 + variante);
  ctx.fillStyle = CREME;
  ctx.fillRect(0, 0, largeur, hauteur);
  tache(ctx, largeur * 0.5, hauteur * 0.38, largeur * 0.55, AQUARELLES[(variante + 3) % 6], alea);
  tache(ctx, largeur * 0.3, hauteur * 0.7, largeur * 0.4, AQUARELLES[(variante + 1) % 6], alea);
  MOTIFS[variante % MOTIFS.length](ctx, largeur * 0.5, hauteur * 0.45, largeur * 0.28);
  for (let i = 0; i < 3; i++) trait(ctx, [[largeur * 0.2, hauteur * (0.78 + i * 0.04)], [largeur * (0.5 + alea() * 0.3), hauteur * (0.78 + i * 0.04)]], 2, 'rgba(43,42,58,.6)');
  return toile;
}
