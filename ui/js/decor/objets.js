import { dessinerCarte } from '../cartes/dessin/dessiner-carte.js';
import { emplacements } from '../cartes/emplacements.js';
import { creerIllustration } from './papiers/illustration.js';
import { peindreBaton, peindreBatonCasse } from './papiers/craie.js';
import { peindreGrille, peindreScore, peindreTexte } from './papiers/ecritures.js';
import { peindreCadre, peindreEponge, peindrePoussiere } from './papiers/materiaux.js';
import { peindreCoeur, peindreEtoile, peindreFleche, peindreGribouillis } from './papiers/traces.js';

export const OBJETS = {
  craie_blanche: { l: 1.7, h: 0.34, peindre: (c, l, h) => peindreBaton(c, l, h, '#ece9e1') },
  craie_jaune: { l: 1.7, h: 0.34, peindre: (c, l, h) => peindreBaton(c, l, h, '#f5d76e') },
  craie_rose: { l: 1.7, h: 0.34, peindre: (c, l, h) => peindreBaton(c, l, h, '#f0a0b4') },
  craie_bleue: { l: 1.7, h: 0.34, peindre: (c, l, h) => peindreBaton(c, l, h, '#8fb4ff') },
  craie_cassee: { l: 0.85, h: 0.34, peindre: (c, l, h) => peindreBatonCasse(c, l, h, '#ece9e1') },
  craie_cassee_jaune: { l: 0.7, h: 0.34, peindre: (c, l, h) => peindreBatonCasse(c, l, h, '#f5d76e') },
  poussiere: { l: 3.4, h: 1.8, peindre: peindrePoussiere },
  eponge: { l: 2.8, h: 1.2, peindre: peindreEponge },
  cadre: { l: 9, h: 0.7, peindre: peindreCadre },
  trace_etoile: { l: 1.8, h: 1.8, peindre: peindreEtoile },
  trace_etoile_rose: { l: 1.2, h: 1.2, peindre: (c, l, h) => peindreEtoile(c, l, h, '#ff9fb0') },
  trace_fleche: { l: 3.4, h: 1.6, peindre: peindreFleche },
  trace_coeur: { l: 1.6, h: 1.5, peindre: peindreCoeur },
  trace_uno: { l: 3.4, h: 1.5, peindre: (c, l, h) => peindreTexte(c, l, h, 'UNO', '#f5d76e') },
  trace_etoile_bleue: { l: 1.4, h: 1.4, peindre: (c, l, h) => peindreEtoile(c, l, h, '#8fb4ff') },
  trace_etoile_verte: { l: 1.1, h: 1.1, peindre: (c, l, h) => peindreEtoile(c, l, h, '#9fe0a8') },
  trace_fleche_jaune: { l: 3, h: 1.4, peindre: (c, l, h) => peindreFleche(c, l, h, '#f5d76e') },
  trace_fleche_rose: { l: 2.6, h: 1.2, peindre: (c, l, h) => peindreFleche(c, l, h, '#ff9fb0') },
  trace_coeur_jaune: { l: 1.2, h: 1.1, peindre: (c, l, h) => peindreCoeur(c, l, h, '#f5d76e') },
  trace_coeur_bleu: { l: 1.3, h: 1.2, peindre: (c, l, h) => peindreCoeur(c, l, h, '#8fb4ff') },
  trace_gribouillis_rose: { l: 3, h: 1, peindre: (c, l, h) => peindreGribouillis(c, l, h, '#ff9fb0') },
  trace_gribouillis_vert: { l: 2.6, h: 0.9, peindre: (c, l, h) => peindreGribouillis(c, l, h, '#9fe0a8') },
  trace_plus4: { l: 2.2, h: 1.3, peindre: (c, l, h) => peindreTexte(c, l, h, '+4', '#ff9fb0') },
  trace_bravo: { l: 3.6, h: 1.3, peindre: (c, l, h) => peindreTexte(c, l, h, 'bravo !', '#9fe0a8') },
  trace_score: { l: 2.6, h: 1.2, peindre: peindreScore },
  trace_grille: { l: 3, h: 3, peindre: peindreGrille },
  trace_gribouillis: { l: 3.4, h: 1.2, peindre: peindreGribouillis },
};

export function estCarte(cle) {
  return cle.startsWith('carte:');
}

export function peindreObjet(nom, toile) {
  const objet = OBJETS[nom];
  const ctx = toile.getContext('2d');
  ctx.scale(toile.width / objet.l, toile.height / objet.h);
  objet.peindre(ctx, objet.l, objet.h);
}

export function peindreCarteDecor(cle, toile) {
  const [, variante, id] = cle.split(':');
  const carte = emplacements().find((c) => c.id === id);
  const image = variante === 'defaut' ? null : creerIllustration(Number(variante));
  dessinerCarte(toile.getContext('2d'), carte, image, toile.width, toile.height);
}
