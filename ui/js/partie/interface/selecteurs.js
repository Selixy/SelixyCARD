import { bouton, element } from './dom.js';

const COULEURS = ['rouge', 'jaune', 'vert', 'bleu'];

function demander(racine, titre, choix) {
  return new Promise(resolve => {
    const boite = element('div', 'surface partie-selecteur');
    const fermer = valeur => {
      racine.hidden = true;
      racine.replaceChildren();
      resolve(valeur);
    };
    boite.append(element('h2', 'partie-selecteur-titre', titre));
    const liste = element('div', 'rangee partie-selecteur-choix');
    for (const [valeur, libelle, classe] of choix) liste.append(bouton(libelle, classe, () => fermer(valeur)));
    boite.append(liste, bouton('Annuler', 'button--discret', () => fermer(null)));
    racine.replaceChildren(boite);
    racine.hidden = false;
  });
}

export function choisirCouleur(racine) {
  const choix = COULEURS.map(c => [c, c, `partie-choix-couleur partie-couleur--${c}`]);
  return demander(racine, 'Quelle couleur ?', choix);
}

export function choisirJoueur(racine, joueurs) {
  return demander(racine, 'Échanger sa main avec…', joueurs.map(j => [j.pseudo, j.pseudo, 'button--primary']));
}
