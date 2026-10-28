export function lancerBoucle(moteur) {
  let avant = performance.now();
  let id = null;
  const image = (maintenant) => {
    moteur.tick((maintenant - avant) / 1000);
    avant = maintenant;
    id = requestAnimationFrame(image);
  };
  const basculer = () => {
    if (document.hidden) {
      if (id !== null) cancelAnimationFrame(id);
      id = null;
    } else if (id === null) {
      avant = performance.now();
      id = requestAnimationFrame(image);
    }
  };
  if (typeof document !== 'undefined') {
    document.addEventListener('visibilitychange', basculer);
  }
  id = requestAnimationFrame(image);
}
