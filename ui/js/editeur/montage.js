import { chargerImage } from '../cartes/images.js';
import { majApercu } from './apercu.js';
import { brancherBoutons } from './boutons.js';
import { afficherCibles } from './cibles.js';
import { brancherFichier, lireImage } from './fichier.js';
import { creerRecadreur } from './recadreur.js';

export function monter(doc, etat) {
  const apercu = doc.getElementById('apercu');
  const sha = etat.carte.id === 'dos' ? etat.set.dos : etat.carte.image;
  let imageDuSet = null;
  let recadreur = null;
  const redessiner = (source) => majApercu(apercu, etat.carte, source, imageDuSet);
  recadreur = creerRecadreur({ canvas: doc.getElementById('recadrage'), zoom: doc.getElementById('zoom') }, (source) => {
    redessiner(source);
    doc.getElementById('zone').classList.add('zone--chargee');
    doc.getElementById('enregistrer').disabled = !etat.peutEditer;
    doc.getElementById('zoom').disabled = false;
  });
  brancherFichier(doc, async (fichier) => recadreur.charger(await lireImage(fichier)));
  afficherCibles(doc.getElementById('cibles'), etat.set, etat.carte);
  brancherBoutons(doc, etat, recadreur);
  redessiner(null);
  if (sha) {
    chargerImage(sha).then((image) => {
      imageDuSet = image;
      if (!recadreur.courant()) redessiner(null);
    });
  }
}
