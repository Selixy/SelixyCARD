import { appeler } from '../api.js';

const TEINTES = ['#d9482f', '#e8b730', '#4f9a62', '#3a63b8', '#b0578d', '#e98a4a'];

export function avatar(pseudo) {
  const empreinte = [...pseudo].reduce((h, c) => (h * 31 + c.codePointAt(0)) >>> 0, 7);
  return { initiale: (pseudo.trim()[0] ?? '?').toUpperCase(), teinte: TEINTES[empreinte % TEINTES.length], angle: (empreinte % 360) };
}

export async function afficherCompte(racine) {
  const lien = racine.querySelector('.tab');
  const pendus = racine.querySelectorAll('.compte-cordon, .compte-medaille');
  try {
    const { pseudo } = await appeler('GET', '/api/moi');
    const a = avatar(pseudo);
    lien.textContent = pseudo;
    const medaille = racine.querySelector('.compte-medaille');
    medaille.textContent = a.initiale;
    medaille.style.setProperty('--teinte', a.teinte);
    medaille.style.setProperty('--motif', `${a.angle}deg`);
    pendus.forEach(e => { e.hidden = false; });
  } catch {
    lien.textContent = 'Connexion';
    pendus.forEach(e => { e.hidden = true; });
  }
}

export function demarrer() {
  const racine = typeof document !== 'undefined' && document.getElementById('compte-badge');
  if (racine) afficherCompte(racine);
}

if (typeof window !== 'undefined' && !window.__navigation_active) demarrer();
