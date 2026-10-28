function versCanvas(canvas, evenement) {
  const zone = canvas.getBoundingClientRect();
  return [(evenement.clientX - zone.left) * canvas.width / zone.width, (evenement.clientY - zone.top) * canvas.height / zone.height];
}

export function brancherPointeur(canvas, moteur, surClic) {
  canvas.addEventListener('pointermove', e => moteur.carte_sous_pointeur(...versCanvas(canvas, e)));
  canvas.addEventListener('pointerleave', () => moteur.carte_sous_pointeur(-1e4, -1e4));
  canvas.addEventListener('pointerdown', e => surClic(moteur.carte_sous_pointeur(...versCanvas(canvas, e))));
}
