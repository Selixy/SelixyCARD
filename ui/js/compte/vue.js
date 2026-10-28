const el = (id) => document.getElementById(id);

export function montrerConnecte(pseudo) {
  el('pseudo').textContent = pseudo;
  el('connecte').hidden = false;
  el('anonyme').hidden = true;
}

export function montrerAnonyme() {
  el('connecte').hidden = true;
  el('anonyme').hidden = false;
}

export function montrerErreur(message) {
  el('erreur').textContent = message;
  el('erreur').hidden = !message;
}

export function configurerMode(inscription) {
  el('titre').textContent = inscription ? 'Inscription' : 'Connexion';
  el('valider').textContent = inscription ? 'Créer le compte' : 'Se connecter';
  el('invite').textContent = inscription ? 'Déjà un compte ?' : 'Pas de compte ?';
  el('basculer').textContent = inscription ? 'Se connecter' : 'Créer un compte';
  el('mdp').autocomplete = inscription ? 'new-password' : 'current-password';
}
