import { emplacements } from '../cartes/emplacements.js';

export const urlEditeur = (setId, carteId) => `editeur.html?set=${encodeURIComponent(setId)}&carte=${encodeURIComponent(carteId)}`;

export function voisins(id) {
  const ordre = ['dos', ...emplacements().map((c) => c.id)];
  const i = ordre.indexOf(id);
  return { precedent: i > 0 ? ordre[i - 1] : null, suivant: i >= 0 && i < ordre.length - 1 ? ordre[i + 1] : null };
}
