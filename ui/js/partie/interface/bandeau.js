import { bouton, element } from './dom.js';

export function afficherBandeau(racine, partie) {
  const { etat, moi } = partie;
  racine.replaceChildren();
  if (!etat) {
    racine.append(element('span', 'partie-attente', 'En attente du lancement de la partie…'));
    return;
  }
  const tour = etat.courant === moi ? 'À vous de jouer' : `Au tour de ${etat.courant}`;
  racine.append(element('span', 'lecteur-ecran', `${tour}, couleur ${etat.couleur_courante ?? 'libre'}`));
  racine.classList.toggle('partie-bandeau--muet', !(etat.penalite > 0));
  if (etat.penalite > 0) racine.append(element('span', 'pastille partie-penalite', `+${etat.penalite} à piocher`));
}

export function afficherJoueurs(racine, partie, adversaires, signaler) {
  racine.replaceChildren();
  for (const joueur of adversaires) {
    const puce = element('li', 'partie-joueur');
    puce.classList.toggle('partie-joueur--courant', joueur.pseudo === partie.etat?.courant);
    puce.append(element('span', 'partie-joueur-nom', joueur.pseudo));
    puce.append(element('span', 'pastille', `${joueur.nb_cartes} carte${joueur.nb_cartes > 1 ? 's' : ''}`));
    if (joueur.uno) puce.append(element('span', 'partie-uno', 'UNO'));
    if (joueur.pseudo === partie.etat?.signalable) puce.append(bouton('Signaler', 'partie-signaler', () => signaler(joueur.pseudo)));
    racine.append(puce);
  }
}
