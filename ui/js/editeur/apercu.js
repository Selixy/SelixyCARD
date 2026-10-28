import { dessinerCarte } from '../cartes/dessin/dessiner-carte.js';
import { dessinerDos } from '../cartes/dessin/dos.js';
import { rogner } from './exporter.js';

export function majApercu(canvas, carte, source, imageDuSet) {
  const ctx = canvas.getContext('2d');
  const image = source ? rogner(source.image, source.cadre, canvas.width, canvas.height) : imageDuSet;
  if (carte.id === 'dos') dessinerDos(ctx, image, canvas.width, canvas.height);
  else dessinerCarte(ctx, carte, image, canvas.width, canvas.height);
}
