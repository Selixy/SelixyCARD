import { miseEnPageCadre } from './cadre.js';
import { remplirQuadrants, tracerRectArrondi } from './forme.js';
import { BORD, NEUTRE } from './palette.js';

export function dessinerDos(ctx, image, largeur, hauteur) {
  const m = miseEnPageCadre(largeur, hauteur);
  const d = m.interieur;
  ctx.save();
  ctx.clearRect(0, 0, largeur, hauteur);
  tracerRectArrondi(ctx, 0, 0, largeur, hauteur, m.rayon);
  ctx.clip();
  ctx.fillStyle = BORD;
  ctx.fillRect(0, 0, largeur, hauteur);
  tracerRectArrondi(ctx, d.x, d.y, d.l, d.h, d.rayon);
  ctx.clip();
  if (image) {
    ctx.drawImage(image, d.x, d.y, d.l, d.h);
  } else {
    remplirQuadrants(ctx, d.x, d.y, d.l, d.h);
    ctx.fillStyle = NEUTRE;
    ctx.beginPath();
    ctx.ellipse(largeur / 2, hauteur / 2, largeur * 0.32, largeur * 0.42, -0.26, 0, Math.PI * 2);
    ctx.fill();
  }
  ctx.restore();
}
