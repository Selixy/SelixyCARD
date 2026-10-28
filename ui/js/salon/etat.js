export function etatInitial() {
  return { salon: null, erreur: null, partieLancee: false };
}

export function appliquer(etat, message) {
  switch (message.t) {
    case 'salon':
      return { ...etat, salon: message, erreur: null };
    case 'erreur':
      return { ...etat, erreur: message.message };
    case 'etat':
      return { ...etat, partieLancee: true };
    default:
      return etat;
  }
}

export function estHote(etat, pseudo) {
  return Boolean(etat.salon) && etat.salon.hote === pseudo;
}

export function peutLancer(etat, pseudo) {
  return estHote(etat, pseudo) && etat.salon.joueurs.length >= 2;
}
