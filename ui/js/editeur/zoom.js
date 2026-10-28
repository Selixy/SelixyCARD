import { RATIO, largeurMax, limiter } from './recadrage.js';

export function zoomer(cadre, facteur, iw, ih) {
  const l = cadre.l / facteur;
  const cx = cadre.x + cadre.l / 2;
  const cy = cadre.y + (cadre.l * RATIO) / 2;
  return limiter({ x: cx - l / 2, y: cy - (l * RATIO) / 2, l }, iw, ih);
}

export function niveauDeZoom(cadre, iw, ih) {
  return largeurMax(iw, ih) / cadre.l;
}
