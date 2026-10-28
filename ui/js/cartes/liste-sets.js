import { el, vider } from './dom.js';

function carteSet(set, pseudo, actions) {
  const bloc = el('article', 'surface pile set-carte');
  const lignes = el('div', 'rangee');
  const ouvrir = el('button', 'button button--primary', 'Ouvrir');
  ouvrir.type = 'button';
  ouvrir.addEventListener('click', () => actions.ouvrir(set.id));
  lignes.append(ouvrir);
  if (set.auteur === pseudo) {
    const retirer = el('button', 'button button--discret button--danger', 'Supprimer');
    retirer.type = 'button';
    retirer.addEventListener('click', () => actions.supprimer(set));
    lignes.append(retirer);
  }
  bloc.append(el('h3', 'set-nom', set.nom), el('p', 'hint', `par ${set.auteur}`));
  bloc.append(el('span', 'pastille', `${set.nb_illustrees} / 108 illustrées`), lignes);
  return bloc;
}

export function afficherListe(conteneur, sets, pseudo, actions) {
  vider(conteneur);
  if (!sets.length) {
    conteneur.append(el('p', 'vide', 'Aucun set pour le moment. Créez le premier ci-dessus.'));
    return;
  }
  for (const set of sets) conteneur.append(carteSet(set, pseudo, actions));
}
