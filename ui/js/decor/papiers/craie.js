import { hasard, trait } from './outils.js';

export function grainer(ctx, l, h, graine, quantite = 500) {
  const alea = hasard(graine);
  ctx.save();
  ctx.globalCompositeOperation = 'destination-out';
  for (let i = 0; i < quantite; i++) {
    ctx.globalAlpha = 0.25 + alea() * 0.5;
    ctx.fillRect(alea() * l, alea() * h, 0.02 + alea() * 0.05, 0.02 + alea() * 0.04);
  }
  ctx.restore();
}

export function traceCraie(ctx, points, epaisseur, couleur, graine = 1) {
  const alea = hasard(graine);
  [1, 0.55, 0.4].forEach((f, i) => {
    ctx.globalAlpha = i ? 0.45 : 0.85;
    const decale = points.map(([x, y]) => [x + (alea() - 0.5) * 0.04 * i, y + (alea() - 0.5) * 0.04 * i]);
    trait(ctx, decale, epaisseur * f, couleur);
  });
  ctx.globalAlpha = 1;
}

export function peindreBaton(ctx, l, h, couleur) {
  ctx.fillStyle = couleur;
  ctx.beginPath();
  ctx.roundRect(0, 0, l, h, h * 0.3);
  ctx.fill();
  ctx.fillStyle = 'rgba(255,255,255,.35)';
  ctx.fillRect(h * 0.2, h * 0.12, l - h * 0.4, h * 0.14);
  ctx.fillStyle = 'rgba(0,0,0,.18)';
  ctx.fillRect(h * 0.2, h * 0.74, l - h * 0.4, h * 0.14);
  grainer(ctx, l, h, 5, 90);
}

export function peindreBatonCasse(ctx, l, h, couleur) {
  ctx.fillStyle = couleur;
  ctx.beginPath();
  ctx.moveTo(0, h * 0.1);
  ctx.lineTo(l, 0);
  ctx.lineTo(l - 0.06, h * 0.45);
  ctx.lineTo(l, h);
  ctx.lineTo(0.05, h * 0.9);
  ctx.closePath();
  ctx.fill();
  ctx.fillStyle = 'rgba(255,255,255,.3)';
  ctx.fillRect(0.1, h * 0.15, l - 0.2, h * 0.14);
  grainer(ctx, l, h, 8, 50);
}
