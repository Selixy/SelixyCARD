import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { brancherNavigation } from '../../ui/js/decor/navigation.js';
import { attendrePolices } from '../../ui/js/polices.js';

test('transitions CSS : définitions des animations et repli sans mouvement', () => {
  const css = readFileSync('ui/css/atelier/transitions.css', 'utf-8');
  assert.ok(css.includes('@keyframes ui-apparition'));
  assert.ok(css.includes('@keyframes ui-sortie'));
  assert.ok(css.includes('@keyframes ui-entree'));
  assert.ok(css.includes('.ui-sortie'));
  assert.ok(css.includes('.ui-entree'));
  assert.ok(css.includes('translate: 0 54px'));
  assert.ok(css.includes('scale: .94'));
  assert.ok(css.includes('body:not(.polices-pretes) main'));
  assert.ok(css.includes('body.polices-pretes main'));
  assert.ok(css.includes('prefers-reduced-motion: reduce'));
  assert.ok(css.includes('translate: none !important'));
});

test('transitions CSS : échelonnement des fiches d’accueil', () => {
  const css = readFileSync('ui/css/atelier/transitions.css', 'utf-8');
  assert.ok(css.includes('.accueil-entrees .accueil-entree'));
  assert.ok(css.includes('animation-delay: .18s'));
  assert.ok(css.includes('animation-delay: .26s'));
  assert.ok(css.includes('animation-delay: .34s'));
  assert.ok(css.includes('animation-delay: .42s'));
});

test('brancherNavigation intercepte un clic interne et ignore les liens externes', () => {
  let ecouteurClic;
  const fauxDoc = {
    addEventListener: (type, cb) => {
      if (type === 'click') ecouteurClic = cb;
    }
  };
  const fauxWin = { addEventListener: () => {} };
  globalThis.document = fauxDoc;
  globalThis.window = fauxWin;
  globalThis.location = { origin: 'http://localhost:3000', href: 'http://localhost:3000/' };

  brancherNavigation(() => Promise.resolve());
  assert.equal(fauxWin.__navigation_active, true);
  assert.equal(typeof ecouteurClic, 'function');

  let empeche = false;
  ecouteurClic({
    target: {
      closest: () => ({ origin: 'http://autre.com', href: 'http://autre.com/page', pathname: '/page' })
    },
    preventDefault: () => { empeche = true; }
  });
  assert.equal(empeche, false);
});

test('attendrePolices ajoute polices-pretes des resolution', async () => {
  const classes = new Set();
  const fauxDoc = {
    body: {
      classList: {
        contains: (c) => classes.has(c),
        add: (...cs) => cs.forEach((c) => classes.add(c))
      }
    },
    fonts: {
      ready: Promise.resolve()
    }
  };
  globalThis.document = fauxDoc;
  await attendrePolices(50);
  assert.equal(classes.has('polices-pretes'), true);
});

test('attendrePolices utilise le repli timeout', async () => {
  const classes = new Set();
  const fauxDoc = {
    body: {
      classList: {
        contains: (c) => classes.has(c),
        add: (...cs) => cs.forEach((c) => classes.add(c))
      }
    },
    fonts: {
      ready: new Promise(() => {})
    }
  };
  globalThis.document = fauxDoc;
  await attendrePolices(15);
  assert.equal(classes.has('polices-pretes'), true);
});
