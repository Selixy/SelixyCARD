const COULEURS = ['rouge', 'jaune', 'vert', 'bleu'];

export function choisirCouleurRobot(cartes) {
  const comptes = { rouge: 0, jaune: 0, vert: 0, bleu: 0 };
  for (const c of cartes) {
    if (c.couleur && comptes[c.couleur] !== undefined) comptes[c.couleur] += 1;
  }
  let max = -1;
  let meilleure = COULEURS[0];
  for (const c of COULEURS) {
    if (comptes[c] > max) {
      max = comptes[c];
      meilleure = c;
    }
  }
  return meilleure;
}

export function choisirCibleRobot(joueurs, pseudoBot) {
  const adversaires = joueurs.filter((j) => j.pseudo !== pseudoBot);
  if (!adversaires.length) return null;
  let minCartes = Infinity;
  let cible = adversaires[0].pseudo;
  for (const j of adversaires) {
    if (j.nb_cartes < minCartes) {
      minCartes = j.nb_cartes;
      cible = j.pseudo;
    }
  }
  return cible;
}
