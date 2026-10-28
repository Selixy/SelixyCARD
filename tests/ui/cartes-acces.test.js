import { test } from 'node:test';
import assert from 'node:assert/strict';
import { demarrerBibliotheque } from '../../ui/js/cartes/bibliotheque.js';
import { demarrerEditeur } from '../../ui/js/editeur/page.js';

function creerDoc(elements) {
  return {
    getElementById: (id) => elements[id] || { hidden: false, textContent: '', append: () => {}, replaceChildren: () => {} },
    querySelector: (sel) => elements[sel] || { hidden: false },
  };
}

test('bibliothèque sans session invite à se connecter et ne liste rien', async () => {
  globalThis.addEventListener = () => {};
  globalThis.removeEventListener = () => {};
  globalThis.location = { hash: '' };
  const elements = {
    'creer-set': { hidden: false, addEventListener: () => {} },
    'invite-connexion': { hidden: true },
    'vue-set': { hidden: false },
    'vue-liste': { hidden: true },
    'erreur': { textContent: '' },
    'sets': { vide: false, replaceChildren() { this.vide = true; } },
  };
  const appels = [];
  const appeler = async (methode, chemin) => {
    appels.push({ methode, chemin });
    if (chemin === '/api/moi') throw new Error('Non connecté');
    return [];
  };
  await demarrerBibliotheque(appeler, creerDoc(elements));
  assert.equal(elements['creer-set'].hidden, true);
  assert.equal(elements['invite-connexion'].hidden, false);
  assert.equal(elements['sets'].vide, true);
  assert.deepEqual(appels, [{ methode: 'GET', chemin: '/api/moi' }]);
});

test('bibliothèque avec session affiche la création et charge les sets', async () => {
  globalThis.addEventListener = () => {};
  globalThis.removeEventListener = () => {};
  globalThis.location = { hash: '' };
  const setsEnfants = [];
  const elements = {
    'creer-set': { hidden: true, addEventListener: () => {} },
    'invite-connexion': { hidden: false },
    'vue-set': { hidden: false },
    'vue-liste': { hidden: true },
    'erreur': { textContent: '' },
    'sets': {
      replaceChildren() { setsEnfants.length = 0; },
      append(noeud) { setsEnfants.push(noeud); },
    },
  };
  globalThis.document = {
    createElement: () => ({ append: () => {}, addEventListener: () => {} }),
  };
  const appeler = async (methode, chemin) => {
    if (chemin === '/api/moi') return { pseudo: 'selixy' };
    if (chemin === '/api/sets') return [{ id: '1', nom: 'Set test', auteur: 'selixy', nb_illustrees: 0 }];
    return [];
  };
  await demarrerBibliotheque(appeler, creerDoc(elements));
  assert.equal(elements['creer-set'].hidden, false);
  assert.equal(elements['invite-connexion'].hidden, true);
  assert.equal(setsEnfants.length, 1);
});

test('éditeur sans session invite et redirige vers le compte', async () => {
  const elements = {
    'invite-connexion': { hidden: true },
    '.editeur': { hidden: false },
  };
  globalThis.location = { href: '/cartes/editeur.html' };
  const appeler = async () => { throw new Error('Non connecté'); };
  await demarrerEditeur(appeler, creerDoc(elements));
  assert.equal(elements['invite-connexion'].hidden, false);
  assert.equal(elements['.editeur'].hidden, true);
  assert.equal(globalThis.location.href, '/compte/');
});
