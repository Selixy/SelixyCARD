import { test } from 'node:test';
import assert from 'node:assert/strict';
import { normaliserCode, urlPartie, urlSocket } from '../../ui/js/salon/adresses.js';
import { appliquer, estHote, etatInitial, peutLancer } from '../../ui/js/salon/etat.js';
import { choisirSet, creer, lancer, rejoindre, reglages } from '../../ui/js/salon/messages.js';

test('adresses : code normalisé, WebSocket selon le protocole, page de partie', () => {
  assert.equal(normaliserCode('  ab3k9 '), 'AB3K9');
  assert.equal(normaliserCode(null), '');
  assert.equal(urlSocket({ protocol: 'https:', host: 'card.selixy.dev' }), 'wss://card.selixy.dev/ws');
  assert.equal(urlSocket({ protocol: 'http:', host: 'localhost:8080' }), 'ws://localhost:8080/ws');
  assert.equal(urlPartie('AB3K9'), '/partie/?code=AB3K9');
  assert.equal(urlPartie('AB3K9', 'alice'), '/partie/?code=AB3K9&pseudo=alice');
});

test('messages envoyés conformes au contrat', () => {
  assert.deepEqual(creer('alice', ''), { t: 'creer', pseudo: 'alice', set: null });
  assert.deepEqual(rejoindre('bob', ' xy12z'), { t: 'rejoindre', pseudo: 'bob', code: 'XY12Z' });
  assert.deepEqual(reglages({ jump_in: true }), { t: 'options', options: { jump_in: true } });
  assert.deepEqual(choisirSet('abc'), { t: 'choisir_set', set: 'abc' });
  assert.deepEqual(lancer(), { t: 'lancer' });
});

test('état du salon mis à jour par les messages du serveur', () => {
  const salon = { t: 'salon', code: 'AAAAA', hote: 'alice', joueurs: ['alice'], options: {}, set: null };
  let etat = appliquer(etatInitial(), salon);
  assert.equal(etat.salon.code, 'AAAAA');
  assert.equal(estHote(etat, 'alice'), true);
  assert.equal(estHote(etat, 'bob'), false);
  assert.equal(peutLancer(etat, 'alice'), false);
  etat = appliquer(etat, { ...salon, joueurs: ['alice', 'bob'] });
  assert.equal(peutLancer(etat, 'alice'), true);
  assert.equal(peutLancer(etat, 'bob'), false);
});

test('erreur affichée puis effacée, état de partie lance la redirection', () => {
  let etat = appliquer(etatInitial(), { t: 'erreur', message: 'Salon introuvable.' });
  assert.equal(etat.erreur, 'Salon introuvable.');
  etat = appliquer(etat, { t: 'salon', code: 'B', hote: 'a', joueurs: ['a'], options: {}, set: null });
  assert.equal(etat.erreur, null);
  assert.equal(etat.partieLancee, false);
  assert.equal(appliquer(etat, { t: 'etat' }).partieLancee, true);
  assert.equal(appliquer(etat, { t: 'main', cartes: [] }), etat);
});
