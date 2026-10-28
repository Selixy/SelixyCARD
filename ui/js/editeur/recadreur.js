import { brancherGestes } from './gestes.js';
import { cadreInitial, deplacer } from './recadrage.js';
import { niveauDeZoom, zoomer } from './zoom.js';
import { dessinerRecadrage } from './vue-recadrage.js';

export function creerRecadreur(zone, auChangement) {
  let image = null;
  let cadre = null;
  const courant = () => (image ? { image, cadre } : null);
  const appliquer = (nouveau) => {
    cadre = nouveau;
    dessinerRecadrage(zone.canvas, image, cadre);
    zone.zoom.value = niveauDeZoom(cadre, image.width, image.height);
    auChangement(courant());
  };
  brancherGestes(zone.canvas, {
    image: () => image,
    deplacer: (dx, dy) => appliquer(deplacer(cadre, dx, dy, image.width, image.height)),
    zoomer: (facteur) => appliquer(zoomer(cadre, facteur, image.width, image.height)),
  });
  zone.zoom.addEventListener('input', () => {
    if (!image) return;
    const facteur = Number(zone.zoom.value) / niveauDeZoom(cadre, image.width, image.height);
    appliquer(zoomer(cadre, facteur, image.width, image.height));
  });
  return {
    courant,
    charger(nouvelle) {
      image = nouvelle;
      appliquer(cadreInitial(image.width, image.height));
    },
  };
}
