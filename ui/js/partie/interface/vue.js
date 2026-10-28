import { adversairesDe } from '../etat/vers-table.js';
import { peutContester, peutDireUno } from '../etat/regles.js';
import { afficherBandeau, afficherJoueurs } from './bandeau.js';
import { afficherFin } from './fin.js';

export function trouverZones(document) {
  const par = id => document.getElementById(id);
  return {
    bandeau: par('bandeau'), joueurs: par('joueurs'), uno: par('btn-uno'), contester: par('btn-contester'),
    message: par('message'), selecteur: par('selecteur'), fin: par('fin'),
    jouerPiochee: par('btn-jouer-piochee'), garder: par('btn-garder'),
  };
}

export function afficher(zones, partie, actions) {
  afficherBandeau(zones.bandeau, partie);
  afficherJoueurs(zones.joueurs, partie, adversairesDe(partie), actions.signaler);
  zones.uno.hidden = !peutDireUno(partie);
  zones.contester.hidden = !peutContester(partie);
  zones.jouerPiochee.hidden = !partie.piochee;
  zones.garder.hidden = !partie.piochee;
  zones.message.textContent = partie.erreur ?? '';
  zones.message.hidden = !partie.erreur;
  afficherFin(zones.fin, partie);
}
