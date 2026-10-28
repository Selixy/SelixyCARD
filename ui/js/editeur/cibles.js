import { el, vider } from '../cartes/dom.js';
import { libelle } from '../cartes/dessin/palette.js';
import { jumeaux } from '../cartes/emplacements.js';

export function afficherCibles(conteneur, set, carte) {
  vider(conteneur);
  const ids = carte.id === 'dos' ? [] : jumeaux(carte.id);
  conteneur.parentElement.hidden = !ids.length;
  if (!ids.length) return;
  conteneur.append(el('p', 'hint', 'Appliquer aussi cette illustration à :'));
  const liste = el('div', 'cibles-liste');
  for (const id of ids) {
    const cible = set.cartes.find((c) => c.id === id);
    const etiquette = el('label', 'cible');
    const case_ = el('input');
    case_.type = 'checkbox';
    case_.value = id;
    etiquette.append(case_, el('span', '', ` ${libelle(cible)} (${id.split('-').pop()})`));
    liste.append(etiquette);
  }
  conteneur.append(liste);
}

export function basculerCibles(conteneur, coche) {
  conteneur.querySelectorAll('input').forEach((c) => { c.checked = coche; });
}

export function lireCibles(conteneur, carte) {
  const choisies = [...conteneur.querySelectorAll('input:checked')].map((c) => c.value);
  return [carte.id, ...choisies];
}
