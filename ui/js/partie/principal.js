import { creerMoteur } from '../moteur/chargeur.js';
import { lancerBoucle } from '../moteur/boucle.js';
import { choisirRendu } from '../moteur/choix-rendu.js';
import { suivreTaille } from '../moteur/taille.js';
import { fabriqueBouchon } from './bouchon/simulateur.js';
import { creerConnexion, fabriqueWebSocket } from './connexion.js';
import { creerControleur } from './controleur.js';
import { brancherPointeur } from './interface/pointeur.js';
import { trouverZones } from './interface/vue.js';
import { dessinerCarte } from '../cartes/dessin/dessiner-carte.js';
import { dessinerDos } from '../cartes/dessin/dos.js';
import { creerFabrique } from './textures/fabrique.js';
import { chargerPoudres } from './textures/poudre.js';
import { chargerImage } from './textures/image.js';
import { peuplerDecor } from '../decor/index.js';
import { attendrePolices, chargerPolices } from '../polices.js';
import { enrichirSalon } from './salon-set.js';
import { messageRejoindre } from './rejoindre.js';
import { demarrerRobotsPartie } from './ia-hote.js';
import { appeler } from '../api.js';

async function pseudoJoueur(parametres) {
  if (parametres.has('pseudo')) return parametres.get('pseudo');
  try {
    const { appeler } = await import('../api.js');
    return (await appeler('GET', '/api/moi')).pseudo;
  } catch {
    try {
      return sessionStorage.getItem('pseudo') || 'moi';
    } catch {
      return 'moi';
    }
  }
}

function fabriqueSocket(parametres, moi) {
  if (parametres.has('bouchon')) return fabriqueBouchon(moi);
  return fabriqueWebSocket(`${location.protocol === 'https:' ? 'wss' : 'ws'}://${location.host}/ws`);
}

async function demarrer() {
  const parametres = new URLSearchParams(location.search);
  const canvas = document.getElementById('scene');
  const sansPartie = !parametres.has('code') && !parametres.has('bouchon');
  document.getElementById('sans-partie').hidden = !sansPartie;
  const moi = await pseudoJoueur(parametres);
  const webgpu = 'gpu' in navigator && !/iPhone|iPad|iPod/.test(navigator.userAgent);
  const moteur = await creerMoteur(canvas, choisirRendu(parametres, webgpu));
  const toile = document.getElementById('scene') ?? canvas;
  suivreTaille(toile, moteur);
  await chargerPolices();
  attendrePolices();
  peuplerDecor(moteur, 'partie', matchMedia('(max-width: 700px)').matches);
  if (sansPartie) {
    document.querySelector('.partie-hud').hidden = true;
    lancerBoucle(moteur);
    return;
  }
  moteur.definir_table('{"main":[],"defausse":null,"adversaires":[],"pioche":0}');
  const fabrique = creerFabrique(moteur, { dessinerCarte, dessinerDos }, chargerImage);
  chargerPoudres(moteur).catch(() => {});
  let connexion = null;
  const controleur = creerControleur({ moi, connexion: () => connexion, moteur, fabrique, zones: trouverZones(document) });
  brancherPointeur(toile, moteur, controleur.surClic);
  connexion = creerConnexion(fabriqueSocket(parametres, moi), {
    ouverte: () => {
      const message = messageRejoindre(parametres, moi);
      if (message) connexion.envoyer(message);
    },
    message: m => enrichirSalon(m, appeler).then(controleur.surMessage),
    fermee: () => controleur.surMessage({ t: 'erreur', message: 'Connexion interrompue : reconnexion en cours...' }),
  });
  const bots = demarrerRobotsPartie(parametres, sessionStorage);
  window.addEventListener('beforeunload', () => bots?.fermer());
  lancerBoucle(moteur);
}

demarrer().catch(e => {
  const zone = document.getElementById('message');
  zone.textContent = `Erreur : ${e?.message ?? e}`;
  zone.hidden = false;
});
