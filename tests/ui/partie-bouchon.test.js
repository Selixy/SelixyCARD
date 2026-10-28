import { test } from 'node:test';
import assert from 'node:assert/strict';
import { genererPaquet } from '../../ui/js/partie/bouchon/paquet.js';
import { fabriqueBouchon } from '../../ui/js/partie/bouchon/simulateur.js';
import { creerConnexion } from '../../ui/js/partie/connexion.js';

function brancher() {
  const recus = [];
  const connexion = creerConnexion(fabriqueBouchon('moi'), { ouverte() {}, fermee() {}, message: m => recus.push(m) });
  return { recus, connexion };
}

const attendre = ms => new Promise(resolve => setTimeout(resolve, ms));

test('le paquet compte 108 cartes aux identifiants uniques', () => {
  const paquet = genererPaquet();
  assert.equal(paquet.length, 108);
  assert.equal(new Set(paquet.map(c => c.id)).size, 108);
});

test('le bouchon annonce salon, main et état comme le serveur', async () => {
  const { recus } = brancher();
  await attendre(120);
  assert.deepEqual(recus.map(m => m.t), ['salon', 'main', 'etat']);
  assert.equal(recus[1].cartes.length, 7);
  assert.equal(recus[2].joueurs.length, 4);
});

test('piocher ajoute une carte puis les bots jouent', async () => {
  const { recus, connexion } = brancher();
  await attendre(120);
  connexion.envoyer({ t: 'piocher' });
  const apres = recus.filter(m => m.t === 'main').at(-1);
  assert.equal(apres.cartes.length, 8);
  await attendre(2300);
  assert.equal(recus.filter(m => m.t === 'etat').at(-1).courant, 'moi');
});

test('une carte injouable renvoie une erreur', async () => {
  const { recus, connexion } = brancher();
  await attendre(120);
  connexion.envoyer({ t: 'jouer', carte: 'inconnue' });
  assert.equal(recus.at(-1).t, 'erreur');
});
