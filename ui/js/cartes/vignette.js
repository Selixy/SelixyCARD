import { el } from './dom.js';
import { dessinerCarte } from './dessin/dessiner-carte.js';
import { dessinerDos } from './dessin/dos.js';
import { libelle } from './dessin/palette.js';
import { chargerImage } from './images.js';

export const LARGEUR = 160;
export const HAUTEUR = 240;

let observateur = null;

function obtenirObservateur() {
  if (typeof IntersectionObserver === 'undefined') return null;
  if (!observateur) {
    observateur = new IntersectionObserver((entrees) => {
      for (const entree of entrees) {
        if (!entree.isIntersecting) continue;
        const canvas = entree.target;
        observateur.unobserve(canvas);
        if (canvas._peindre) {
          canvas._peindre();
          delete canvas._peindre;
        }
      }
    }, { rootMargin: '200px' });
  }
  return observateur;
}

export function peindre(canvas, carte, image) {
  const ctx = canvas.getContext('2d');
  if (carte.id === 'dos') dessinerDos(ctx, image, canvas.width, canvas.height);
  else dessinerCarte(ctx, carte, image, canvas.width, canvas.height);
}

export function creerVignette(carte, href) {
  const lien = el('a', 'vignette surface--cliquable');
  const canvas = el('canvas', 'vignette-canvas');
  canvas.width = LARGEUR;
  canvas.height = HAUTEUR;
  lien.href = href;
  lien.title = carte.id === 'dos' ? 'Dos des cartes' : libelle(carte);
  lien.setAttribute('aria-label', `${lien.title}${carte.image ? ', illustrée' : ', sans illustration'}`);
  lien.append(canvas);
  if (carte.image) lien.append(el('span', 'pastille vignette-badge', 'illustrée'));
  const executer = () => {
    peindre(canvas, carte, null);
    if (carte.image) chargerImage(carte.image).then((image) => image && peindre(canvas, carte, image));
  };
  const obs = obtenirObservateur();
  if (obs) {
    canvas._peindre = executer;
    obs.observe(canvas);
  } else {
    executer();
  }
  return lien;
}
