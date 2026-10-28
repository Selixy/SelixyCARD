import { test } from 'node:test';
import assert from 'node:assert/strict';
import { messageRejoindre } from '../../ui/js/partie/rejoindre.js';

test('un invité renvoie son pseudo en rejoignant la partie', () => {
  assert.deepEqual(messageRejoindre(new URLSearchParams('code=ZFGAH'), 'Lou'), { t: 'rejoindre', code: 'ZFGAH', pseudo: 'Lou' });
});

test('sans code, on ne rejoint rien', () => {
  assert.equal(messageRejoindre(new URLSearchParams(''), 'Lou'), null);
});
