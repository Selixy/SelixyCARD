import { test } from 'node:test';
import assert from 'node:assert/strict';
import { choisirRendu } from '../../ui/js/moteur/choix-rendu.js';

test('WebGPU par défaut quand il est disponible', () => {
  assert.equal(choisirRendu(new URLSearchParams(''), true), 'webgpu');
});

test('repli WebGL2 sans WebGPU', () => {
  assert.equal(choisirRendu(new URLSearchParams(''), false), 'webgl2');
});

test('le paramètre ?rendu= force le choix', () => {
  assert.equal(choisirRendu(new URLSearchParams('rendu=webgl2'), true), 'webgl2');
});
