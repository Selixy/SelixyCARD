const COULEURS = ['rouge', 'jaune', 'vert', 'bleu'];

export function melanger(cartes) {
  for (let i = cartes.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [cartes[i], cartes[j]] = [cartes[j], cartes[i]];
  }
  return cartes;
}

export function genererPaquet() {
  const cartes = [];
  for (const couleur of COULEURS) {
    cartes.push({ id: `${couleur}-0`, type: 'nombre', couleur, valeur: 0, image: null });
    for (const lettre of 'ab') {
      for (let valeur = 1; valeur <= 9; valeur++) cartes.push({ id: `${couleur}-${valeur}-${lettre}`, type: 'nombre', couleur, valeur, image: null });
      for (const type of ['passer', 'inverser', 'plus2']) cartes.push({ id: `${couleur}-${type}-${lettre}`, type, couleur, valeur: null, image: null });
    }
  }
  for (let i = 1; i <= 4; i++) {
    cartes.push({ id: `joker-${i}`, type: 'joker', couleur: null, valeur: null, image: null });
    cartes.push({ id: `joker_plus4-${i}`, type: 'joker_plus4', couleur: null, valeur: null, image: null });
  }
  return melanger(cartes);
}

export function jouable(carte, defausse, couleur) {
  if (carte.type === 'joker' || carte.type === 'joker_plus4') return true;
  if (carte.couleur === couleur) return true;
  return carte.type === defausse.type && carte.valeur === defausse.valeur;
}
