export function etatInitial(moi) {
  return { moi, salon: null, etat: null, main: [], jouables: null, piochee: null, eclair: null, fin: null, erreur: null };
}

export function appliquer(courant, message) {
  switch (message.t) {
    case 'salon': return { ...courant, salon: message, erreur: null };
    case 'etat': return { ...courant, etat: message, fin: null, erreur: null };
    case 'main': return { ...courant, main: message.cartes, jouables: message.jouables ?? null, piochee: message.piochee ?? null };
    case 'fin': return { ...courant, fin: message };
    case 'erreur': return { ...courant, erreur: message.message };
    default: return courant;
  }
}
