import { chargerPolices } from '../polices.js';
import { appeler } from '../api.js';
import { demarrerBibliotheque } from './bibliotheque.js';

export function demarrer() {
  return chargerPolices().then(() => demarrerBibliotheque(appeler, document));
}

if (!window.__navigation_active) demarrer();
