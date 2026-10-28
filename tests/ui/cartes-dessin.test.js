import test from 'node:test';
import assert from 'node:assert/strict';
import { miseEnPageCadre } from '../../ui/js/cartes/dessin/cadre.js';
import { dessinerCarte } from '../../ui/js/cartes/dessin/dessiner-carte.js';
import { libelle, symbole } from '../../ui/js/cartes/dessin/palette.js';

function faux() {
  const appels = [];
  return new Proxy({}, {
    get: (_, nom) => (nom === 'appels' ? appels : (...args) => appels.push([nom, ...args])),
    set: () => true,
  });
}

test('mise en page : intérieur contenu dans la carte, coins dedans', () => {
  const m = miseEnPageCadre(320, 480);
  assert.ok(m.interieur.x > 0 && m.interieur.x + m.interieur.l < 320);
  assert.ok(m.interieur.y + m.interieur.h < 480);
  assert.ok(m.coin.x < 160 && m.coin.y < 240);
});

test('symboles et libellés', () => {
  assert.equal(symbole({ type: 'nombre', valeur: 0 }), '0');
  assert.equal(symbole({ type: 'joker_plus4' }), '+4');
  assert.equal(libelle({ type: 'nombre', valeur: 7, couleur: 'rouge' }), '7 rouge');
});

test('dessinerCarte : sans image, valeur au centre ; avec image, illustration', () => {
  const carte = { id: 'rouge-7-a', type: 'nombre', couleur: 'rouge', valeur: 7 };
  const sans = faux();
  dessinerCarte(sans, carte, null, 320, 480);
  assert.ok(sans.appels.some((a) => a[0] === 'fillText' && a[1] === '7'));
  assert.ok(!sans.appels.some((a) => a[0] === 'drawImage'));
  const avec = faux();
  const image = {};
  dessinerCarte(avec, carte, image, 320, 480);
  assert.ok(avec.appels.some((a) => a[0] === 'drawImage' && a[1] === image));
  assert.equal(avec.appels.filter((a) => a[0] === 'fillText').length, 2);
});

test('joker : quatre quadrants de couleur', () => {
  const ctx = faux();
  dessinerCarte(ctx, { id: 'joker-1', type: 'joker', couleur: null, valeur: null }, null, 320, 480);
  assert.ok(ctx.appels.filter((a) => a[0] === 'fillRect').length >= 5);
});
