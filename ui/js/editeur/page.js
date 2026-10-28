import { libelle } from '../cartes/dessin/palette.js';
import { lirePseudo } from '../cartes/session.js';
import { chargerSet } from '../cartes/service-sets.js';
import { monter } from './montage.js';
import { urlEditeur, voisins } from './navigation.js';

const DOS = { id: 'dos', type: 'dos', couleur: null, valeur: null, image: null };

function remplirEntete(doc, etat) {
  const { set, carte } = etat;
  const ids = voisins(carte.id);
  doc.getElementById('titre').textContent = carte.id === 'dos' ? 'Dos des cartes' : libelle(carte);
  doc.getElementById('sous-titre').textContent = `${set.nom} · ${carte.id}`;
  doc.getElementById('retour').href = `index.html#set=${encodeURIComponent(set.id)}`;
  for (const [cle, id] of Object.entries(ids)) {
    const lien = doc.getElementById(cle);
    lien.hidden = !id;
    if (id) lien.href = urlEditeur(set.id, id);
  }
  doc.getElementById('lecture-seule').hidden = etat.peutEditer;
}

export async function demarrerEditeur(appeler, doc) {
  const pseudo = await lirePseudo(appeler);
  if (!pseudo) {
    const invite = doc.getElementById('invite-connexion');
    if (invite) invite.hidden = false;
    const editeur = doc.querySelector('.editeur');
    if (editeur) editeur.hidden = true;
    if (typeof location !== 'undefined') location.href = '/compte/';
    return;
  }
  try {
    const params = new URLSearchParams(location.search);
    const set = await chargerSet(appeler, params.get('set'));
    const id = params.get('carte');
    const carte = id === 'dos' ? DOS : set.cartes.find((c) => c.id === id);
    if (!carte) throw new Error('Cette carte n’existe pas dans le set.');
    const etat = { appeler, set, carte, peutEditer: pseudo === set.auteur };
    remplirEntete(doc, etat);
    monter(doc, etat);
  } catch (e) {
    doc.getElementById('erreur').textContent = e.message;
  }
}
