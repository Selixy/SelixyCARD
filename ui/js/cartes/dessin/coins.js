import { CREME, POLICE, symbole, teinte } from './palette.js';

function ecrire(ctx, texte, x, y, taille) {
  ctx.font = `700 ${taille * 1.15}px ${POLICE}`;
  ctx.textAlign = 'center';
  ctx.textBaseline = 'middle';
  ctx.lineJoin = 'round';
  ctx.lineWidth = taille * 0.2;
  ctx.strokeStyle = 'rgba(43,42,58,.85)';
  ctx.strokeText(texte, x, y);
  ctx.fillStyle = CREME;
  ctx.fillText(texte, x, y);
}

export function dessinerCoins(ctx, carte, m) {
  const texte = symbole(carte);
  const taille = m.coin.taille * (texte.length > 1 ? 0.8 : 1);
  ctx.save();
  ecrire(ctx, texte, m.coin.x, m.coin.y, taille);
  ctx.translate(m.largeur, m.hauteur);
  ctx.rotate(Math.PI);
  ecrire(ctx, texte, m.coin.x, m.coin.y, taille);
  ctx.restore();
}

export function dessinerCentre(ctx, carte, m) {
  const c = m.centre;
  const texte = symbole(carte);
  ctx.save();
  ctx.translate(c.x, c.y);
  ctx.rotate(-Math.PI / 12);
  ctx.beginPath();
  ctx.ellipse(0, 0, c.rx, c.ry, 0, 0, Math.PI * 2);
  ctx.fillStyle = 'rgba(251,245,230,.94)';
  ctx.fill();
  ctx.strokeStyle = 'rgba(43,42,58,.7)';
  ctx.lineWidth = c.taille * 0.05;
  ctx.stroke();
  ctx.rotate(Math.PI / 12);
  ctx.font = `700 ${c.taille * 1.15 * (texte.length > 1 ? 0.8 : 1)}px ${POLICE}`;
  ctx.textAlign = 'center';
  ctx.textBaseline = 'middle';
  ctx.fillStyle = teinte(carte);
  ctx.fillText(texte, 0, 0);
  ctx.restore();
}
