import { hasard, trait } from './outils.js';
import { grainer } from './craie.js';

export function peindrePoussiere(ctx, l, h) {
  const alea = hasard(21);
  for (let i = 0; i < 40; i++) {
    ctx.fillStyle = `rgba(236,233,225,${0.05 + alea() * 0.1})`;
    ctx.beginPath();
    ctx.ellipse(l * (0.2 + alea() * 0.6), h * (0.25 + alea() * 0.5), l * (0.05 + alea() * 0.18), h * (0.05 + alea() * 0.2), alea() * 3, 0, Math.PI * 2);
    ctx.fill();
  }
}

export function peindreEponge(ctx, l, h) {
  ctx.fillStyle = '#7a5a3a';
  ctx.beginPath();
  ctx.roundRect(0, 0, l, h * 0.55, 0.12);
  ctx.fill();
  ctx.fillStyle = '#3d4348';
  ctx.beginPath();
  ctx.roundRect(0.05, h * 0.5, l - 0.1, h * 0.5, 0.1);
  ctx.fill();
  ctx.fillStyle = 'rgba(236,233,225,.5)';
  for (let x = 0.2; x < l - 0.2; x += 0.16) ctx.fillRect(x, h * 0.58, 0.05, h * 0.34);
  grainer(ctx, l, h, 13, 120);
}

export function peindreCadre(ctx, l, h) {
  const alea = hasard(31);
  ctx.fillStyle = '#6b4a2e';
  ctx.fillRect(0, 0, l, h);
  ctx.fillStyle = '#4e3520';
  ctx.fillRect(0, h * 0.82, l, h * 0.18);
  for (let i = 0; i < 14; i++) {
    const y = alea() * h * 0.8;
    trait(ctx, [[alea() * l * 0.4, y], [l * (0.5 + alea() * 0.5), y + (alea() - 0.5) * 0.1]], 0.015, 'rgba(40,25,12,.45)');
  }
}
