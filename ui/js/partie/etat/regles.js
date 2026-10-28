export function besoinCouleur(carte) {
  return carte.type === 'joker' || carte.type === 'joker_plus4';
}

export function besoinCible(carte, options) {
  return carte.type === 'nombre' && carte.valeur === 7 && options?.sept_zero === true;
}

export function peutContester(partie) {
  const { etat, moi } = partie;
  return Boolean(etat?.contestable) && etat.courant === moi;
}

export function peutDireUno(partie) {
  const { etat, moi, main, jouables } = partie;
  return etat?.courant === moi && main.length === 2 && (jouables ?? []).length > 0;
}
