import { COULEURS } from './emplacements.js';
import { el, vider } from './dom.js';
import { creerVignette } from './vignette.js';

const href = (set, id) => `editeur.html?set=${encodeURIComponent(set.id)}&carte=${encodeURIComponent(id)}`;

export const nbIllustrees = (cartes) => cartes.filter((c) => c.image).length;

function groupe(titre, cartes, set) {
  const section = el('section', 'groupe');
  const entete = el('h2', 'groupe-titre', titre);
  entete.append(el('span', 'hint', ` ${nbIllustrees(cartes)} / ${cartes.length} illustrées`));
  const grille = el('div', 'grille grille-cartes');
  for (const carte of cartes) grille.append(creerVignette(carte, href(set, carte.id)));
  section.append(entete, grille);
  return section;
}

export function afficherSet(conteneur, set) {
  vider(conteneur);
  const dos = { id: 'dos', type: 'dos', couleur: null, valeur: null, image: set.dos };
  conteneur.append(groupe('Dos des cartes', [dos], set));
  for (const couleur of COULEURS) {
    const cartes = set.cartes.filter((c) => c.couleur === couleur);
    conteneur.append(groupe(couleur[0].toUpperCase() + couleur.slice(1), cartes, set));
  }
  conteneur.append(groupe('Jokers', set.cartes.filter((c) => !c.couleur), set));
}
