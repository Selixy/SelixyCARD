import { normaliserCode } from './adresses.js';

export function creer(pseudo, set) {
  return { t: 'creer', pseudo, set: set || null };
}

export function rejoindre(pseudo, code) {
  return { t: 'rejoindre', pseudo, code: normaliserCode(code) };
}

export function reglages(options) {
  return { t: 'options', options };
}

export function choisirSet(set) {
  return { t: 'choisir_set', set };
}

export function lancer() {
  return { t: 'lancer' };
}
