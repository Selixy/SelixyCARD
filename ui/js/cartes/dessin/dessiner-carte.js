import { miseEnPageCadre } from './cadre.js';
import { dessinerCentre, dessinerCoins } from './coins.js';
import { remplirTeinte, tracerRectArrondi } from './forme.js';
import { BORD } from './palette.js';

export function dessinerCarte(ctx, carte, image, largeur, hauteur) {
  const m = miseEnPageCadre(largeur, hauteur);
  const dedans = m.interieur;
  ctx.save();
  ctx.clearRect(0, 0, largeur, hauteur);
  tracerRectArrondi(ctx, 0, 0, largeur, hauteur, m.rayon);
  ctx.clip();
  ctx.fillStyle = BORD;
  ctx.fillRect(0, 0, largeur, hauteur);
  tracerRectArrondi(ctx, dedans.x, dedans.y, dedans.l, dedans.h, dedans.rayon);
  ctx.clip();
  if (image) {
    ctx.drawImage(image, dedans.x, dedans.y, dedans.l, dedans.h);
  } else {
    remplirTeinte(ctx, carte, dedans);
    dessinerCentre(ctx, carte, m);
  }
  ctx.restore();
  dessinerCoins(ctx, carte, m);
}
