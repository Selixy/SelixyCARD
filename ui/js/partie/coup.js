import { besoinCible, besoinCouleur } from './etat/regles.js';
import { adversairesDe } from './etat/vers-table.js';
import { choisirCouleur, choisirJoueur } from './interface/selecteurs.js';

export async function preparerCoup(carte, partie, selecteur) {
  if (!carte) return null;
  const message = { t: 'jouer', carte: carte.id };
  if (besoinCouleur(carte)) message.couleur = await choisirCouleur(selecteur);
  if (besoinCible(carte, partie.salon?.options)) message.cible = await choisirJoueur(selecteur, adversairesDe(partie));
  return message.couleur === null || message.cible === null ? null : message;
}
