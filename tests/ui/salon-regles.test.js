import { test } from 'node:test';
import assert from 'node:assert/strict';
import { REGLES } from '../../ui/js/salon/regles.js';
import { SET_PAR_DEFAUT, chargerSets, libelleSet } from '../../ui/js/salon/sets.js';

test('les six règles du contrat ont un libellé et une explication', () => {
  const cles = REGLES.map((r) => r.cle).sort();
  assert.deepEqual(cles, ['cumul_plus', 'contester_plus4', 'jump_in', 'penalite_uno', 'piocher_jusqua_jouable', 'sept_zero'].sort());
  for (const regle of REGLES) {
    assert.ok(regle.libelle.length > 5, regle.cle);
    assert.ok(regle.aide.length > 30, regle.cle);
  }
});

test('liste des sets via la fonction appeler injectée', async () => {
  const appels = [];
  const sets = await chargerSets(async (methode, chemin) => {
    appels.push([methode, chemin]);
    return [{ id: 'u1', nom: 'Chats', auteur: 'alice', nb_illustrees: 12 }];
  });
  assert.deepEqual(appels, [['GET', '/api/sets']]);
  assert.equal(sets.length, 1);
});

test('échec ou réponse inattendue : liste vide, pas d\'exception', async () => {
  assert.deepEqual(await chargerSets(async () => { throw new Error('Session expirée'); }), []);
  assert.deepEqual(await chargerSets(async () => ({ erreur: 'x' })), []);
});

test('libellé des sets', () => {
  assert.equal(libelleSet({ nom: 'Chats', auteur: 'alice', nb_illustrees: 1 }), 'Chats — alice (1 illustrée)');
  assert.equal(libelleSet({ nom: 'Chats', auteur: 'alice', nb_illustrees: 108 }), 'Chats — alice (108 illustrées)');
  assert.equal(libelleSet(SET_PAR_DEFAUT), 'Set par défaut (sans illustrations)');
});
