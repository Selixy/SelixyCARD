import { test } from 'node:test';
import assert from 'node:assert/strict';
import { appeler } from '../../ui/js/api.js';

function bouchonner(statut, donnees) {
  const appels = [];
  globalThis.fetch = async (chemin, options) => {
    appels.push({ chemin, options });
    return { status: statut, ok: statut < 400, json: async () => donnees };
  };
  return appels;
}

test('envoie le corps en JSON et renvoie la réponse', async () => {
  const appels = bouchonner(200, { pseudo: 'ana' });
  const resultat = await appeler('POST', '/api/session', { pseudo: 'ana' });
  assert.deepEqual(resultat, { pseudo: 'ana' });
  assert.equal(appels[0].options.body, '{"pseudo":"ana"}');
  assert.equal(appels[0].options.headers['Content-Type'], 'application/json');
});

test('un Blob part brut avec son type', async () => {
  const appels = bouchonner(201, { image: 'abc' });
  const blob = new Blob(['RIFF'], { type: 'image/webp' });
  await appeler('POST', '/api/images', blob);
  assert.equal(appels[0].options.body, blob);
  assert.equal(appels[0].options.headers['Content-Type'], 'image/webp');
});

test('204 donne null et sans corps pas d\'en-tête', async () => {
  const appels = bouchonner(204, null);
  assert.equal(await appeler('DELETE', '/api/session'), null);
  assert.equal(appels[0].options.body, undefined);
});

test('une erreur lève le message français du serveur', async () => {
  bouchonner(409, { erreur: 'Ce pseudo est déjà pris.' });
  await assert.rejects(appeler('POST', '/api/comptes', {}), { message: 'Ce pseudo est déjà pris.' });
});

test('réseau coupé : message français', async () => {
  globalThis.fetch = async () => { throw new TypeError('fail'); };
  await assert.rejects(appeler('GET', '/api/moi'), { message: 'Le serveur est injoignable.' });
});
