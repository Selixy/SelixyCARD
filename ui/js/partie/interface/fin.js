import { element } from './dom.js';

export function afficherFin(racine, partie) {
  racine.hidden = !partie.fin;
  racine.replaceChildren();
  if (!partie.fin) return;
  const gagne = partie.fin.gagnant === partie.moi;
  const boite = element('div', 'surface partie-selecteur');
  boite.append(element('h2', 'partie-selecteur-titre', gagne ? 'Victoire !' : 'Partie terminée'));
  boite.append(element('p', '', gagne ? 'Vous avez posé votre dernière carte.' : `${partie.fin.gagnant} remporte la partie.`));
  const retour = element('a', 'button button--primary', 'Retour au salon');
  retour.href = '/salon/';
  boite.append(retour);
  racine.append(boite);
}
