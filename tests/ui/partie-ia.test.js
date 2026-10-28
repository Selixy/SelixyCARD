import { test } from 'node:test';
import assert from 'node:assert/strict';
import { estNomRobot, prochainNomRobot, chargerRobots, sauverRobots } from '../../ui/js/salon/robots.js';
import { choisirCouleurRobot, choisirCibleRobot } from '../../ui/js/partie/ia/tactique.js';
import { evaluerDecision } from '../../ui/js/partie/ia/decision.js';

test('identification et génération de nom de robot', () => {
  assert.equal(estNomRobot('Robot 1'), true);
  assert.equal(estNomRobot('Alice'), false);
  assert.equal(prochainNomRobot(['Alice', 'Robot 1']), 'Robot 2');
  assert.equal(prochainNomRobot(['Robot 1', 'Robot 2']), 'Robot 3');
});

test('persistance des robots en stockage', () => {
  const memoire = {};
  const stockage = {
    getItem: (k) => memoire[k] ?? null,
    setItem: (k, v) => { memoire[k] = v; },
  };
  assert.deepEqual(chargerRobots(stockage), []);
  sauverRobots(stockage, ['Robot 1', 'Robot 2']);
  assert.deepEqual(chargerRobots(stockage), ['Robot 1', 'Robot 2']);
});

test('tactique : choix couleur et cible', () => {
  const cartes = [
    { type: 'nombre', couleur: 'rouge' },
    { type: 'nombre', couleur: 'bleu' },
    { type: 'nombre', couleur: 'bleu' },
  ];
  assert.equal(choisirCouleurRobot(cartes), 'bleu');

  const joueurs = [
    { pseudo: 'Robot 1', nb_cartes: 5 },
    { pseudo: 'Alice', nb_cartes: 2 },
    { pseudo: 'Bob', nb_cartes: 4 },
  ];
  assert.equal(choisirCibleRobot(joueurs, 'Robot 1'), 'Alice');
});

test('décision : uno, pioche, coup, contestation et signalement', () => {
  const robot = 'Robot 1';
  const signal = evaluerDecision({
    pseudo: robot,
    main: [],
    jouables: [],
    etat: { signalable: 'Bob', courant: 'Alice' },
  });
  assert.deepEqual(signal, { t: 'signaler', pseudo: 'Bob' });

  const contestation = evaluerDecision({
    pseudo: robot,
    main: [],
    jouables: [],
    etat: { contestable: true, courant: robot },
  });
  assert.deepEqual(contestation, { t: 'contester' });

  const coupUno = evaluerDecision({
    pseudo: robot,
    main: [{ id: 'bleu-1-a' }, { id: 'rouge-1-a' }],
    jouables: ['bleu-1-a'],
    etat: { courant: robot, joueurs: [{ pseudo: robot, uno: false }] },
  });
  assert.deepEqual(coupUno, { t: 'uno' });

  const coupApresUno = evaluerDecision({
    pseudo: robot,
    main: [{ id: 'bleu-1-a' }, { id: 'rouge-1-a' }],
    jouables: ['bleu-1-a'],
    etat: { courant: robot, joueurs: [{ pseudo: robot, uno: true }] },
  });
  assert.deepEqual(coupApresUno, { t: 'jouer', carte: 'bleu-1-a' });

  const coupJouer = evaluerDecision({
    pseudo: robot,
    main: [{ id: 'bleu-1-a', type: 'nombre', couleur: 'bleu' }],
    jouables: ['bleu-1-a'],
    etat: { courant: robot },
  });
  assert.deepEqual(coupJouer, { t: 'jouer', carte: 'bleu-1-a' });
});
