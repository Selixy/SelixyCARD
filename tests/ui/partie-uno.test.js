import { test } from 'node:test';
import assert from 'node:assert/strict';
import { peutDireUno } from '../../ui/js/partie/etat/regles.js';

const partie = (courant, cartes, jouables) => ({ moi: 'moi', etat: { courant }, main: cartes.map(id => ({ id })), jouables });

test('UNO seulement à mon tour, avec 2 cartes dont une jouable', () => {
  assert.equal(peutDireUno(partie('moi', ['a', 'b'], ['a'])), true);
  assert.equal(peutDireUno(partie('autre', ['a', 'b'], ['a'])), false);
  assert.equal(peutDireUno(partie('moi', ['a', 'b'], [])), false);
  assert.equal(peutDireUno(partie('moi', ['a', 'b'], null)), false);
  assert.equal(peutDireUno(partie('moi', ['a'], ['a'])), false);
  assert.equal(peutDireUno(partie('moi', ['a', 'b', 'c'], ['a'])), false);
  assert.equal(peutDireUno({ moi: 'moi', etat: null, main: [], jouables: null }), false);
});
