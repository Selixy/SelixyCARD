import { test } from 'node:test';
import assert from 'node:assert/strict';
import { appliquer, etatInitial } from '../../ui/js/partie/etat/reducteur.js';
import { adversairesDe, versTable } from '../../ui/js/partie/etat/vers-table.js';
import { besoinCible, besoinCouleur, peutContester } from '../../ui/js/partie/etat/regles.js';

const carte = (id, extra = {}) => ({ id, type: 'nombre', couleur: 'rouge', valeur: 3, image: null, ...extra });
const joueurs = ['a', 'moi', 'b', 'c'].map(pseudo => ({ pseudo, nb_cartes: 5, uno: false }));
const etat = { t: 'etat', defausse: carte('rouge-7-a', { valeur: 7 }), couleur_courante: 'rouge', sens: 1, courant: 'moi', joueurs, pioche: 40, penalite: 0 };

test('le réducteur range chaque message à sa place', () => {
  let p = etatInitial('moi');
  p = appliquer(p, etat);
  p = appliquer(p, { t: 'main', cartes: [carte('x')] });
  assert.equal(p.main.length, 1);
  assert.equal(p.etat.pioche, 40);
  p = appliquer(p, { t: 'erreur', message: 'non' });
  assert.equal(p.erreur, 'non');
  p = appliquer(p, { t: 'fin', gagnant: 'b' });
  assert.equal(p.fin.gagnant, 'b');
  assert.equal(appliquer(p, { t: 'inconnu' }), p);
});

test('adversaires dans l’ordre de la table à partir du joueur suivant', () => {
  const p = appliquer(etatInitial('moi'), etat);
  assert.deepEqual(adversairesDe(p).map(j => j.pseudo), ['b', 'c', 'a']);
});

test('le JSON de table respecte le contrat du moteur', () => {
  let p = appliquer(etatInitial('moi'), etat);
  p = appliquer(p, { t: 'main', cartes: [carte('rouge-3-a'), carte('rouge-4-a', { image: 'abc' })] });
  const table = JSON.parse(versTable(p));
  assert.deepEqual(table.main, ['defaut:rouge-3-a', 'abc:rouge-4-a']);
  assert.equal(table.defausse, 'defaut:rouge-7-a');
  assert.deepEqual(table.adversaires, [5, 5, 5]);
  assert.equal(table.pioche, 40);
});

test('jokers, 7 et contestation', () => {
  assert.equal(besoinCouleur({ type: 'joker_plus4' }), true);
  assert.equal(besoinCouleur(carte('x')), false);
  assert.equal(besoinCible(carte('s', { valeur: 7 }), { sept_zero: true }), true);
  assert.equal(besoinCible(carte('s', { valeur: 7 }), { sept_zero: false }), false);
  const plus4 = { ...etat, defausse: { type: 'joker_plus4' }, penalite: 4, contestable: true };
  assert.equal(peutContester(appliquer(etatInitial('moi'), { ...plus4, contestable: false })), false);
  assert.equal(peutContester(appliquer(etatInitial('moi'), plus4)), true);
  assert.equal(peutContester(appliquer(etatInitial('autre'), plus4)), false);
});
