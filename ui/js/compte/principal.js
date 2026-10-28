import { appeler } from '../api.js';
import { configurerMode, montrerAnonyme, montrerConnecte, montrerErreur } from './vue.js';

let inscription = false;

async function envoyer(evenement) {
  evenement.preventDefault();
  const champs = new FormData(evenement.target);
  const chemin = inscription ? '/api/comptes' : '/api/session';
  try {
    const moi = await appeler('POST', chemin, Object.fromEntries(champs));
    montrerErreur('');
    montrerConnecte(moi.pseudo);
  } catch (erreur) {
    montrerErreur(erreur.message);
  }
}

async function deconnecter() {
  await appeler('DELETE', '/api/session');
  montrerAnonyme();
}

function basculer(evenement) {
  evenement.preventDefault();
  inscription = !inscription;
  montrerErreur('');
  configurerMode(inscription);
}

export async function demarrer() {
  document.getElementById('formulaire').addEventListener('submit', envoyer);
  document.getElementById('deconnexion').addEventListener('click', deconnecter);
  document.getElementById('basculer').addEventListener('click', basculer);
  try {
    montrerConnecte((await appeler('GET', '/api/moi')).pseudo);
  } catch {
    montrerAnonyme();
  }
}

if (!window.__navigation_active) demarrer();
