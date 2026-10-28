import test from 'node:test';
import assert from 'node:assert/strict';
import { COULEURS, emplacements, jumeaux } from '../../ui/js/cartes/emplacements.js';

test('108 emplacements aux identifiants uniques', () => {
  const liste = emplacements();
  assert.equal(liste.length, 108);
  assert.equal(new Set(liste.map((c) => c.id)).size, 108);
});

test('25 cartes par couleur et 8 jokers', () => {
  const liste = emplacements();
  for (const c of COULEURS) assert.equal(liste.filter((x) => x.couleur === c).length, 25);
  assert.equal(liste.filter((x) => x.type === 'joker').length, 4);
  assert.equal(liste.filter((x) => x.type === 'joker_plus4').length, 4);
});

test('identifiants et champs du contrat', () => {
  const par = Object.fromEntries(emplacements().map((c) => [c.id, c]));
  assert.deepEqual(par['rouge-7-a'], { id: 'rouge-7-a', type: 'nombre', couleur: 'rouge', valeur: 7, image: null });
  assert.equal(par['bleu-0'].valeur, 0);
  assert.equal(par['vert-plus2-b'].type, 'plus2');
  assert.equal(par['jaune-inverser-a'].valeur, null);
  assert.equal(par['joker-3'].couleur, null);
  assert.equal(par['joker_plus4-4'].type, 'joker_plus4');
  assert.ok(!par['rouge-0-a'] && !par['rouge-10-a']);
});

test('jumeaux : même valeur dans les autres couleurs et emplacements', () => {
  const j = jumeaux('rouge-7-a');
  assert.equal(j.length, 7);
  assert.ok(j.includes('rouge-7-b') && j.includes('bleu-7-a') && !j.includes('rouge-7-a'));
  assert.equal(jumeaux('rouge-0').length, 3);
  assert.deepEqual(jumeaux('joker-1').sort(), ['joker-2', 'joker-3', 'joker-4']);
  assert.equal(jumeaux('rouge-passer-a').length, 7);
});
