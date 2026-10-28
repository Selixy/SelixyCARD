export const ENCRE = '#2b2a3a';
export const CREME = '#f7f0df';
export const AQUARELLES = ['#d9482f', '#e8b730', '#5f9c6a', '#3a63b8', '#b0578d', '#e98a4a'];

export function hasard(graine) {
  let etat = graine * 9301 + 49297;
  return () => {
    etat = (etat * 9301 + 49297) % 233280;
    return etat / 233280;
  };
}

export function tache(ctx, x, y, rayon, couleur, alea) {
  ctx.fillStyle = couleur;
  for (let i = 0; i < 7; i++) {
    ctx.globalAlpha = 0.16;
    ctx.beginPath();
    ctx.ellipse(x + (alea() - 0.5) * rayon * 0.5, y + (alea() - 0.5) * rayon * 0.5, rayon * (0.6 + alea() * 0.4), rayon * (0.5 + alea() * 0.4), alea() * 3, 0, Math.PI * 2);
    ctx.fill();
  }
  ctx.globalAlpha = 1;
}

export function trait(ctx, points, epaisseur, couleur) {
  ctx.strokeStyle = couleur;
  ctx.lineWidth = epaisseur;
  ctx.lineCap = 'round';
  ctx.lineJoin = 'round';
  ctx.beginPath();
  points.forEach(([x, y], i) => (i ? ctx.lineTo(x, y) : ctx.moveTo(x, y)));
  ctx.stroke();
}
