export const TEINTES = { rouge: '#d9482f', jaune: '#e8b730', vert: '#4f9a62', bleu: '#3a63b8' };
export const POLICE = '"Caveat Brush", "Caveat", cursive';
export const CREME = '#fbf5e6';
export const NEUTRE = '#2b2a3a';
export const BORD = '#fbf8f1';
const SYMBOLES = { passer: '⊘', inverser: '⇄', plus2: '+2', joker: '★', joker_plus4: '+4' };
const NOMS = { passer: 'Passer', inverser: 'Inverser', plus2: '+2', joker: 'Joker', joker_plus4: 'Joker +4' };

export function symbole(carte) {
  return carte.type === 'nombre' ? String(carte.valeur) : SYMBOLES[carte.type];
}

export function teinte(carte) {
  return carte.couleur ? TEINTES[carte.couleur] : NEUTRE;
}

export function libelle(carte) {
  const base = carte.type === 'nombre' ? String(carte.valeur) : NOMS[carte.type];
  return carte.couleur ? `${base} ${carte.couleur}` : base;
}
