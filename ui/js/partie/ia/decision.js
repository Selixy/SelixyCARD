import { choisirCibleRobot, choisirCouleurRobot } from './tactique.js';

function moiRobot(etat, pseudo) {
  return (etat?.joueurs ?? []).find((j) => j.pseudo === pseudo);
}

export function evaluerDecision({ pseudo, main, jouables, piochee, etat, options }) {
  if (etat.signalable && etat.signalable !== pseudo) {
    return { t: 'signaler', pseudo: etat.signalable };
  }
  if (etat.contestable && etat.courant === pseudo) {
    return { t: 'contester' };
  }
  if (etat.courant !== pseudo) return null;
  const robot = moiRobot(etat, pseudo);
  if (main.length === 2 && !robot?.uno && jouables?.length > 0) {
    return { t: 'uno' };
  }
  const carteId = piochee || (jouables && jouables.length > 0 ? jouables[0] : null);
  if (!carteId) {
    return { t: 'piocher' };
  }
  const carte = main.find((c) => c.id === carteId);
  const coup = { t: 'jouer', carte: carteId };
  if (carte && (carte.type === 'joker' || carte.type === 'joker_plus4')) {
    coup.couleur = choisirCouleurRobot(main.filter((c) => c.id !== carteId));
  }
  if (carte && carte.type === 'nombre' && carte.valeur === 7 && options?.sept_zero) {
    coup.cible = choisirCibleRobot(etat.joueurs ?? [], pseudo);
  }
  return coup;
}
