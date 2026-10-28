import { CLE_DOS, cleCarte } from './cles.js';

export const LARGEUR_TEXTURE = 256;
export const HAUTEUR_TEXTURE = 384;

export function creerFabrique(moteur, dessin, chargerImage, creerToile = () => document.createElement('canvas')) {
  const demandees = new Set();

  async function produire(cle, sha, dessiner) {
    const image = sha ? await chargerImage(sha).catch(() => null) : null;
    const toile = creerToile();
    toile.width = LARGEUR_TEXTURE;
    toile.height = HAUTEUR_TEXTURE;
    dessiner(toile.getContext('2d'), image);
    moteur.charger_texture(cle, toile);
  }

  function demander(cle, sha, dessiner, marque = cle) {
    if (demandees.has(marque)) return Promise.resolve();
    demandees.add(marque);
    return produire(cle, sha, dessiner).catch(() => demandees.delete(marque));
  }

  return {
    carte(carte) {
      return demander(cleCarte(carte), carte.image, (ctx, image) => dessin.dessinerCarte(ctx, carte, image, LARGEUR_TEXTURE, HAUTEUR_TEXTURE));
    },
    dos(sha) {
      return demander(CLE_DOS, sha, (ctx, image) => dessin.dessinerDos(ctx, image, LARGEUR_TEXTURE, HAUTEUR_TEXTURE), `${CLE_DOS}:${sha ?? ''}`);
    },
  };
}
