import { SCENES } from './scenes/index.js';
import { clesDe, resoudre } from './scene.js';
import { resoudreEtabli } from './etabli.js';
import { chargerCles } from './textures.js';

export async function peuplerDecor(moteur, nom, etroit, connues = new Set(), sansCartes = false) {
  const decor = nom === 'partie' ? resoudre(SCENES.partie, etroit, sansCartes) : resoudreEtabli(nom, etroit, sansCartes);
  moteur.definir_decor(JSON.stringify(decor));
  await chargerCles(moteur, clesDe(decor), connues);
}
