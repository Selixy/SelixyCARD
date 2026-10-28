import { grainer, traceCraie } from './craie.js';

const CRAIE = '#ece9e1';

export function peindreEtoile(ctx, l, h, couleur = '#f5d76e') {
  const pts = [];
  for (let i = 0; i < 6; i++) {
    const a = -Math.PI / 2 + (i * 4 * Math.PI) / 5;
    pts.push([l / 2 + Math.cos(a) * l * 0.42, h / 2 + Math.sin(a) * h * 0.42]);
  }
  traceCraie(ctx, [...pts, pts[0]], 0.07, couleur, 2);
  grainer(ctx, l, h, 3);
}

export function peindreFleche(ctx, l, h, couleur = CRAIE) {
  traceCraie(ctx, [[0.1, h * 0.7], [l * 0.35, h * 0.3], [l * 0.7, h * 0.55], [l - 0.2, h * 0.5]], 0.07, couleur, 4);
  traceCraie(ctx, [[l - 0.55, h * 0.2], [l - 0.15, h * 0.5], [l - 0.5, h * 0.85]], 0.07, couleur, 5);
  grainer(ctx, l, h, 6);
}

export function peindreCoeur(ctx, l, h, couleur = '#ff8f9a') {
  const pts = [];
  for (let i = 0; i <= 40; i++) {
    const t = (i / 40) * Math.PI * 2;
    pts.push([l / 2 + (16 * Math.sin(t) ** 3 * l) / 36, h * 0.45 - ((13 * Math.cos(t) - 5 * Math.cos(2 * t) - 2 * Math.cos(3 * t) - Math.cos(4 * t)) * h) / 36]);
  }
  traceCraie(ctx, pts, 0.07, couleur, 7);
  grainer(ctx, l, h, 9);
}

export function peindreGribouillis(ctx, l, h, couleur = '#9fc4ff') {
  const pts = [];
  for (let i = 0; i < 26; i++) pts.push([0.2 + (i / 25) * (l - 0.4), h / 2 + Math.sin(i * 1.9) * h * 0.38]);
  traceCraie(ctx, pts, 0.06, couleur, 10);
  grainer(ctx, l, h, 11);
}
