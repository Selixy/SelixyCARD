import { test } from 'node:test';
import assert from 'node:assert/strict';
import { estCliquable, nouvelleCarte, peutPiocher } from '../../ui/js/partie/etat/possibles.js';
import { versTable } from '../../ui/js/partie/etat/vers-table.js';

const carte = id => ({ id, type: 'nombre', couleur: 'rouge', valeur: 1, image: null });
const partie = (champs = {}) => ({
  moi: 'lou', salon: null, fin: null, eclair: null, piochee: null, jouables: ['b'],
  main: [carte('a'), carte('b')],
  etat: { courant: 'lou', joueurs: [{ pseudo: 'lou', nb_cartes: 2 }, { pseudo: 'max', nb_cartes: 5 }], pioche: 40, defausse: null },
  ...champs,
});

test('seules les cartes jouables sont cliquables', () => {
  assert.equal(estCliquable(partie(), carte('a')), false);
  assert.equal(estCliquable(partie(), carte('b')), true);
});

test('hors de mon tour on ne pioche pas', () => {
  assert.equal(peutPiocher(partie()), true);
  assert.equal(peutPiocher(partie({ etat: { ...partie().etat, courant: 'max' } })), false);
  assert.equal(peutPiocher(partie({ piochee: 'b' })), false);
});

test('la carte piochée passe au premier plan et le tour est indiqué', () => {
  const table = JSON.parse(versTable(partie({ piochee: 'b' })));
  assert.equal(table.premier_plan, 1);
  assert.deepEqual(table.jouables, [1]);
  assert.equal(table.courant, 0);
  assert.equal(JSON.parse(versTable(partie({ etat: { ...partie().etat, courant: 'max' } }))).courant, 1);
});

test('la nouvelle carte est repérée après une pioche', () => {
  assert.equal(nouvelleCarte([carte('a')], [carte('a'), carte('c')]).id, 'c');
});
