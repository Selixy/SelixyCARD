import { test } from 'node:test';
import assert from 'node:assert/strict';
import { avatar } from '../../ui/js/compte/badge.js';

test("l'avatar dérive du pseudo, stable et en majuscule", () => {
  assert.deepEqual(avatar('selixy'), avatar('selixy'));
  assert.equal(avatar('selixy').initiale, 'S');
  assert.equal(avatar('  éloïse').initiale, 'É');
  assert.match(avatar('bob').teinte, /^#[0-9a-f]{6}$/);
  assert.notDeepEqual(avatar('bob'), avatar('alice'));
});
