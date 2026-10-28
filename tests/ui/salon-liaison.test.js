import { test } from 'node:test';
import assert from 'node:assert/strict';
import { ouvrir } from '../../ui/js/salon/liaison.js';

class FauxSocket {
  constructor(url) {
    this.url = url;
    this.readyState = 0;
    this.envoyes = [];
    this.ferme = false;
    FauxSocket.dernier = this;
  }
  send(texte) {
    this.envoyes.push(texte);
  }
  close() {
    this.ferme = true;
  }
}

test('les messages envoyés avant l\'ouverture sont mis en file puis envoyés', () => {
  const lien = ouvrir('ws://x', FauxSocket, () => {}, () => {});
  const socket = FauxSocket.dernier;
  lien.envoyer({ t: 'creer' });
  assert.deepEqual(socket.envoyes, []);
  socket.readyState = 1;
  socket.onopen();
  assert.deepEqual(socket.envoyes, ['{"t":"creer"}']);
  lien.envoyer({ t: 'lancer' });
  assert.equal(socket.envoyes.length, 2);
});

test('messages reçus décodés, JSON invalide et fermeture signalés', () => {
  const recus = [];
  let fermetures = 0;
  const lien = ouvrir('ws://x', FauxSocket, (m) => recus.push(m), () => (fermetures += 1));
  const socket = FauxSocket.dernier;
  socket.onmessage({ data: '{"t":"salon"}' });
  assert.deepEqual(recus, [{ t: 'salon' }]);
  socket.onmessage({ data: 'pas du json' });
  socket.onclose();
  assert.equal(fermetures, 2);
  lien.fermer();
  assert.equal(socket.ferme, true);
});
