import { urlPartie, urlSocket } from './adresses.js';
import { appliquer, etatInitial } from './etat.js';
import { ouvrir } from './liaison.js';
import { chargerSets } from './sets.js';
import { afficher, afficherErreur, elements } from './vue.js';
import { creerHoteRobots } from './hote-robots.js';
import { sauverRobots } from './robots.js';
import { brancherEvenements, initialiserPseudo } from './interaction.js';

function memoriserPseudo(stockage, pseudo) {
  try {
    stockage?.setItem('pseudo', pseudo);
  } catch {
    return;
  }
}

function recevoir(session, message) {
  session.etat = appliquer(session.etat, message);
  if (session.etat.partieLancee) {
    session.hoteRobots.fermer();
    session.lien.fermer();
    session.deps.location.assign(urlPartie(session.etat.salon.code, session.pseudo));
    return;
  }
  const onRetirer = (nom) => session.hoteRobots.retirer(nom);
  afficher(session.vue, session.etat, session.pseudo, session.sets, session.robots, onRetirer);
}

function entrer(session, construireMessage) {
  const { vue, deps } = session;
  session.pseudo = vue.pseudo.value.trim();
  if (!session.pseudo) return afficherErreur(vue, 'Choisis un pseudo pour jouer.');
  memoriserPseudo(deps.stockage, session.pseudo);
  session.robots = [];
  sauverRobots(deps.stockage, session.robots);
  afficherErreur(vue, null);
  session.lien?.fermer();
  session.lien = ouvrir(urlSocket(deps.location), deps.SocketImpl, (m) => recevoir(session, m), () =>
    afficherErreur(vue, 'Connexion au serveur perdue.'),
  );
  session.lien.envoyer(construireMessage(session.pseudo));
}

export async function demarrer(deps) {
  const vue = elements(deps.document);
  const session = { vue, deps, etat: etatInitial(), lien: null, pseudo: '', sets: [], robots: [] };
  session.hoteRobots = creerHoteRobots(session);
  initialiserPseudo(vue, deps);
  brancherEvenements(session, entrer);
  const moi = await deps.appeler('GET', '/api/moi').catch(() => null);
  if (moi?.pseudo) {
    vue.pseudo.value = moi.pseudo;
    vue.pseudo.readOnly = true;
  }
  session.sets = await chargerSets(deps.appeler);
  return session;
}
