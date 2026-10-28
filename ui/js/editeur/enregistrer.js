import { appliquerImage, envoyerImage } from '../cartes/images.js';
import { sauverSet } from '../cartes/service-sets.js';

export async function enregistrer(appeler, set, cibles, blob) {
  const sha = await envoyerImage(appeler, blob);
  return sauverSet(appeler, appliquerImage(set, cibles, sha));
}

export function retirer(appeler, set, cibles) {
  return sauverSet(appeler, appliquerImage(set, cibles, null));
}
