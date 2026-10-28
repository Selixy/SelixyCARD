import { TEINTES, teinte } from './palette.js';

export function tracerRectArrondi(ctx, x, y, l, h, rayon) {
  ctx.beginPath();
  ctx.roundRect(x, y, l, h, rayon);
}

export function remplirQuadrants(ctx, x, y, l, h) {
  Object.values(TEINTES).forEach((couleur, i) => {
    ctx.fillStyle = couleur;
    ctx.fillRect(x + (i % 2) * (l / 2), y + Math.floor(i / 2) * (h / 2), l / 2 + 1, h / 2 + 1);
  });
}

export function remplirTeinte(ctx, carte, rect) {
  if (carte.couleur) {
    ctx.fillStyle = teinte(carte);
    ctx.fillRect(rect.x, rect.y, rect.l, rect.h);
  } else {
    remplirQuadrants(ctx, rect.x, rect.y, rect.l, rect.h);
  }
}
