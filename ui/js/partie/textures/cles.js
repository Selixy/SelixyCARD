export const CLE_DOS = 'dos';

export function cleCarte(carte) {
  return carte.image ? `${carte.image}:${carte.id}` : `defaut:${carte.id}`;
}
