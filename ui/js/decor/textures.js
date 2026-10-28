import { OBJETS, estCarte, peindreCarteDecor, peindreObjet } from './objets.js';
import { chargerPolices } from '../polices.js';

const cacheToiles = new Map();

function fabriquer(cle) {
  let toile = cacheToiles.get(cle);
  if (!toile) {
    toile = document.createElement('canvas');
    toile.width = 256;
    toile.height = 384;
    if (estCarte(cle)) peindreCarteDecor(cle, toile);
    else peindreObjet(cle, toile);
    cacheToiles.set(cle, toile);
  }
  return toile;
}

function rafraichirCartes(moteur, cles) {
  for (const cle of cles) {
    if (!estCarte(cle)) continue;
    const toile = cacheToiles.get(cle);
    if (toile) {
      peindreCarteDecor(cle, toile);
      moteur.charger_texture(cle, toile);
    }
  }
}

export async function chargerCles(moteur, cles, connues) {
  for (const cle of cles) {
    if (connues.has(cle)) continue;
    connues.add(cle);
    moteur.charger_texture(cle, fabriquer(cle));
  }
  chargerPolices().then(() => rafraichirCartes(moteur, cles)).catch(() => {});
}

export const tailleObjet = (nom) => [OBJETS[nom].l, OBJETS[nom].h];
