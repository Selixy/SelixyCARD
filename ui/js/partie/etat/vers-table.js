import { cleCarte } from '../textures/cles.js';

export function adversairesDe(partie) {
  const joueurs = partie.etat?.joueurs ?? [];
  const rang = joueurs.findIndex(j => j.pseudo === partie.moi);
  if (rang < 0) return joueurs;
  return [...joueurs.slice(rang + 1), ...joueurs.slice(0, rang)];
}

function indiceCourant(partie) {
  const courant = partie.etat?.courant;
  if (!courant) return -1;
  if (courant === partie.moi) return 0;
  return adversairesDe(partie).findIndex(j => j.pseudo === courant) + 1 || -1;
}

export function versTable(partie) {
  const enAvant = partie.piochee ?? partie.eclair;
  return JSON.stringify({
    main: partie.main.map(cleCarte),
    defausse: partie.etat?.defausse ? cleCarte(partie.etat.defausse) : null,
    adversaires: adversairesDe(partie).map(j => j.nb_cartes),
    pioche: partie.etat?.pioche ?? 0,
    jouables: partie.main.flatMap((c, i) => (partie.jouables ?? []).includes(c.id) ? [i] : []),
    premier_plan: partie.main.findIndex(c => c.id === enAvant),
    courant: indiceCourant(partie),
    couleur: partie.etat?.couleur_courante ?? null,
  });
}
