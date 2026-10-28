import test from 'node:test';
import assert from 'node:assert/strict';
import { cadreInitial, deplacer, limiter } from '../../ui/js/editeur/recadrage.js';
import { niveauDeZoom, zoomer } from '../../ui/js/editeur/zoom.js';
import { appliquerImage } from '../../ui/js/cartes/images.js';
import { emplacements } from '../../ui/js/cartes/emplacements.js';
import { voisins } from '../../ui/js/editeur/navigation.js';

test('cadre initial centré au ratio 2:3', () => {
  const c = cadreInitial(1000, 500);
  assert.equal(c.l, 500 / 1.5);
  assert.equal(c.y, 0);
  assert.ok(Math.abs(c.x - (1000 - c.l) / 2) < 1e-9);
  const p = cadreInitial(400, 1200);
  assert.equal(p.l, 400);
  assert.equal(p.y, 300);
});

test('déplacement borné à l’image', () => {
  const c = deplacer(cadreInitial(1000, 500), 9999, -9999, 1000, 500);
  assert.equal(c.y, 0);
  assert.ok(Math.abs(c.x + c.l - 1000) < 1e-9);
});

test('zoom centré et borné', () => {
  const base = cadreInitial(800, 1200);
  const z = zoomer(base, 2, 800, 1200);
  assert.equal(z.l, 400);
  assert.equal(z.x + z.l / 2, base.x + base.l / 2);
  assert.equal(niveauDeZoom(z, 800, 1200), 2);
  assert.equal(zoomer(base, 1000, 800, 1200).l, 100);
  assert.equal(zoomer(base, 0.1, 800, 1200).l, 800);
  assert.deepEqual(limiter({ x: -5, y: -5, l: 10 }, 800, 1200), { x: 0, y: 0, l: 100 });
});

test('appliquer une image à plusieurs emplacements et au dos', () => {
  const set = { id: 'x', dos: null, cartes: emplacements() };
  const s = appliquerImage(set, ['rouge-7-a', 'rouge-7-b', 'dos'], 'abc');
  assert.equal(s.cartes.filter((c) => c.image === 'abc').length, 2);
  assert.equal(s.dos, 'abc');
  assert.equal(set.cartes.filter((c) => c.image).length, 0);
  assert.equal(appliquerImage(s, ['rouge-7-a'], null).cartes.find((c) => c.id === 'rouge-7-a').image, null);
});

test('navigation entre emplacements', () => {
  assert.deepEqual(voisins('dos'), { precedent: null, suivant: 'rouge-0' });
  assert.equal(voisins('joker_plus4-4').suivant, null);
  assert.equal(voisins('rouge-1-a').precedent, 'rouge-0');
});
