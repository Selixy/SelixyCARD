export function suivreTaille(canvas, moteur) {
  const ajuster = () => {
    const ratio = Math.min(devicePixelRatio, 2);
    canvas.width = Math.round(canvas.clientWidth * ratio);
    canvas.height = Math.round(canvas.clientHeight * ratio);
    moteur.redimensionner(canvas.width, canvas.height);
  };
  new ResizeObserver(ajuster).observe(canvas);
  ajuster();
}
