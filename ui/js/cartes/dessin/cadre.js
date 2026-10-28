export function miseEnPageCadre(largeur, hauteur) {
  const bord = Math.max(2, Math.round(largeur * 0.06));
  return {
    largeur,
    hauteur,
    rayon: largeur * 0.09,
    bord,
    interieur: { x: bord, y: bord, l: largeur - 2 * bord, h: hauteur - 2 * bord, rayon: largeur * 0.05 },
    coin: { x: bord + largeur * 0.1, y: bord + largeur * 0.13, taille: largeur * 0.17 },
    centre: { x: largeur / 2, y: hauteur / 2, rx: largeur * 0.3, ry: largeur * 0.4, taille: largeur * 0.42 },
  };
}
