import { grainer, traceCraie } from './craie.js';

const CRAIE = '#ece9e1';

export function peindreGrille(ctx, l, h) {
  for (let i = 0; i <= 5; i++) {
    traceCraie(ctx, [[0.1 + (i * (l - 0.2)) / 5, 0.1], [0.1 + (i * (l - 0.2)) / 5, h - 0.1]], 0.035, 'rgba(236,233,225,.7)', 20 + i);
    traceCraie(ctx, [[0.1, 0.1 + (i * (h - 0.2)) / 5], [l - 0.1, 0.1 + (i * (h - 0.2)) / 5]], 0.035, 'rgba(236,233,225,.7)', 30 + i);
  }
  grainer(ctx, l, h, 12);
}

export function peindreTexte(ctx, l, h, texte, couleur = CRAIE) {
  ctx.fillStyle = couleur;
  ctx.font = `${h * 0.8}px 'Caveat Brush', 'Caveat', cursive`;
  ctx.textBaseline = 'middle';
  ctx.textAlign = 'center';
  ctx.globalAlpha = 0.9;
  ctx.fillText(texte, l / 2, h * 0.5);
  ctx.globalAlpha = 1;
  grainer(ctx, l, h, 14, 350);
}

export function peindreScore(ctx, l, h) {
  [0, 1, 2, 3].forEach((i) => traceCraie(ctx, [[0.3 + i * 0.28, 0.2], [0.3 + i * 0.28, h - 0.2]], 0.06, CRAIE, 40 + i));
  traceCraie(ctx, [[0.05, h * 0.75], [1.3, h * 0.25]], 0.06, CRAIE, 45);
  [0, 1].forEach((i) => traceCraie(ctx, [[1.7 + i * 0.28, 0.2], [1.7 + i * 0.28, h - 0.2]], 0.06, CRAIE, 50 + i));
  grainer(ctx, l, h, 15);
}
