export const COULEURS = ['rouge', 'jaune', 'vert', 'bleu'];
const ACTIONS = ['passer', 'inverser', 'plus2'];
const SUFFIXES = ['a', 'b'];

function carte(id, type, couleur, valeur = null) {
  return { id, type, couleur, valeur, image: null };
}

export function emplacements() {
  const liste = [];
  for (const c of COULEURS) {
    liste.push(carte(`${c}-0`, 'nombre', c, 0));
    for (let v = 1; v <= 9; v++) {
      for (const s of SUFFIXES) liste.push(carte(`${c}-${v}-${s}`, 'nombre', c, v));
    }
    for (const t of ACTIONS) {
      for (const s of SUFFIXES) liste.push(carte(`${c}-${t}-${s}`, t, c));
    }
  }
  for (let n = 1; n <= 4; n++) {
    liste.push(carte(`joker-${n}`, 'joker', null));
    liste.push(carte(`joker_plus4-${n}`, 'joker_plus4', null));
  }
  return liste;
}

function famille(id) {
  const parties = id.split('-');
  return id.startsWith('joker') ? parties[0] : parties[1];
}

export function jumeaux(id) {
  const reference = famille(id);
  return emplacements().map((c) => c.id).filter((autre) => autre !== id && famille(autre) === reference);
}
