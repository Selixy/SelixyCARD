import { estHote, peutLancer } from './etat.js';
import { lireRegles, rendreRegles } from './regles.js';
import { remplirSets } from './sets.js';
import { creerLigneJoueur } from './vue-joueur.js';

const IDENTIFIANTS = [
  'erreur', 'accueil', 'salon', 'pseudo', 'creer', 'code',
  'rejoindre', 'code-salon', 'joueurs', 'set', 'regles',
  'attente', 'lancer', 'ajouter-robot',
];

export function elements(document) {
  const vue = { document };
  IDENTIFIANTS.forEach((id) => {
    vue[id] = document.getElementById(id);
  });
  return vue;
}

export function afficherErreur(vue, message) {
  vue.erreur.textContent = message ?? '';
  vue.erreur.hidden = !message;
}

function rendreJoueurs(vue, salon, pseudo, robots, onRetirer) {
  const doc = vue.document;
  const lignes = salon.joueurs.map((nom) => creerLigneJoueur(doc, nom, salon, pseudo, robots, onRetirer));
  vue.joueurs.replaceChildren(...lignes);
}

export function afficher(vue, etat, pseudo, sets, robots = [], onRetirer = null) {
  afficherErreur(vue, etat.erreur);
  if (!etat.salon) return;
  const hote = estHote(etat, pseudo);
  vue.accueil.hidden = true;
  vue.salon.hidden = false;
  vue['code-salon'].textContent = etat.salon.code;
  rendreJoueurs(vue, etat.salon, pseudo, robots, onRetirer);
  remplirSets(vue.set, sets, etat.salon.set, hote);
  rendreRegles(vue.regles, etat.salon.options, hote);
  vue.attente.hidden = hote;
  vue.lancer.hidden = !hote;
  vue.lancer.disabled = !peutLancer(etat, pseudo);
  if (vue['ajouter-robot']) {
    vue['ajouter-robot'].hidden = !hote;
    vue['ajouter-robot'].disabled = !hote || etat.salon.joueurs.length >= 10;
  }
}

export function reglesAffichees(vue) {
  return lireRegles(vue.regles);
}
