import { estCarte } from './objets.js';
import { tailleObjet } from './textures.js';

export function element([nom, x, z, lacet = 0, y = 0.05, flotte = 0, echelle = 1], rang) {
  const phase = rang * 1.7;
  if (estCarte(nom)) {
    return { cle: nom, x, y, z, lacet, l: 1.5 * echelle, h: 2.25 * echelle, flotte, phase, mobile: 1 };
  }
  const [l, h] = tailleObjet(nom);
  return { cle: nom, x, y, z, lacet, l: l * echelle, h: h * echelle, flotte: 0, phase, feuille: 1, mobile: 0 };
}

export function resoudre(definition, etroit, sansCartes = false) {
  const choix = etroit && definition.etroit ? { ...definition, ...definition.etroit } : definition;
  const bruts = sansCartes ? choix.elements.filter(([nom]) => !estCarte(nom)) : choix.elements;
  return {
    vise: choix.vise,
    distance: choix.distance,
    elevation: choix.elevation,
    elements: bruts.map(element),
  };
}

export const clesDe = (decor) => [...new Set(decor.elements.map((e) => e.cle))];
