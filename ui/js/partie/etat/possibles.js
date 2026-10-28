export function estCliquable(partie, carte) {
  if (!partie.etat || partie.fin || !carte) return false;
  if (partie.jouables) return partie.jouables.includes(carte.id);
  return partie.etat.courant === partie.moi;
}

export function peutPiocher(partie) {
  return Boolean(partie.etat) && !partie.fin && !partie.piochee && partie.etat.courant === partie.moi;
}

export function nouvelleCarte(avant, apres) {
  const connues = new Set(avant.map(c => c.id));
  return apres.find(c => !connues.has(c.id)) ?? null;
}
