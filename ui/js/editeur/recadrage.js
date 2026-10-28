export const RATIO = 1.5;
export const ZOOM_MAX = 8;

export const largeurMax = (iw, ih) => Math.min(iw, ih / RATIO);

export function limiter(cadre, iw, ih) {
  const max = largeurMax(iw, ih);
  const l = Math.min(max, Math.max(max / ZOOM_MAX, cadre.l));
  const x = Math.min(iw - l, Math.max(0, cadre.x));
  const y = Math.min(ih - l * RATIO, Math.max(0, cadre.y));
  return { x, y, l };
}

export function cadreInitial(iw, ih) {
  const l = largeurMax(iw, ih);
  return limiter({ x: (iw - l) / 2, y: (ih - l * RATIO) / 2, l }, iw, ih);
}

export function deplacer(cadre, dx, dy, iw, ih) {
  return limiter({ ...cadre, x: cadre.x + dx, y: cadre.y + dy }, iw, ih);
}
