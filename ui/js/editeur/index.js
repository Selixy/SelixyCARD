import { chargerPolices } from '../polices.js';
import { appeler } from '../api.js';
import { demarrerEditeur } from './page.js';

export function demarrer() {
  return chargerPolices().then(() => demarrerEditeur(appeler, document));
}

if (!window.__navigation_active) demarrer();
