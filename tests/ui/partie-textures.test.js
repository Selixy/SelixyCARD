import { test } from 'node:test';
import assert from 'node:assert/strict';
import { creerFabrique } from '../../ui/js/partie/textures/fabrique.js';

function atelier() {
  const chargees = [];
  const moteur = { charger_texture: (cle, bitmap) => chargees.push([cle, bitmap]) };
  const appels = [];
  const dessin = {
    dessinerCarte: (ctx, carte, image, l, h) => appels.push(['carte', carte.id, image, l, h]),
    dessinerDos: (ctx, image, l, h) => appels.push(['dos', image, l, h]),
  };
  const toile = () => ({ getContext: () => ({}) });
  globalThis.createImageBitmap = async source => ({ source });
  return { chargees, appels, fabrique: creerFabrique(moteur, dessin, async sha => `image-${sha}`, toile) };
}

test('une carte illustrée charge son image puis sa texture', async () => {
  const { chargees, appels, fabrique } = atelier();
  await fabrique.carte({ id: 'rouge-3-a', image: 'abc' });
  assert.deepEqual(chargees.map(c => c[0]), ['abc:rouge-3-a']);
  assert.deepEqual(appels[0], ['carte', 'rouge-3-a', 'image-abc', 256, 384]);
});

test('une carte sans image utilise la clé par défaut et n’est produite qu’une fois', async () => {
  const { chargees, fabrique } = atelier();
  await Promise.all([fabrique.carte({ id: 'bleu-0', image: null }), fabrique.carte({ id: 'bleu-0', image: null })]);
  assert.deepEqual(chargees.map(c => c[0]), ['defaut:bleu-0']);
});

test('le dos a la clé « dos »', async () => {
  const { chargees, fabrique } = atelier();
  await fabrique.dos(null);
  assert.equal(chargees[0][0], 'dos');
});
