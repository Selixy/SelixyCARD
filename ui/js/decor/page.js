import { creerMoteur } from '../moteur/chargeur.js';
import { lancerBoucle } from '../moteur/boucle.js';
import { choisirRendu } from '../moteur/choix-rendu.js';
import { suivreTaille } from '../moteur/taille.js';
import { attendrePolices, chargerPolices } from '../polices.js';
import { peuplerDecor } from './index.js';
import { brancherNavigation } from './navigation.js';

function programmerIndicateur() {
  return setTimeout(() => {
    if (document.body.classList.contains('decor-actif') || document.body.classList.contains('decor-absent')) return;
    let el = document.getElementById('decor-chargement');
    if (!el) {
      el = document.createElement('div');
      el.id = 'decor-chargement';
      el.className = 'decor-chargement';
      el.setAttribute('aria-live', 'polite');
      el.innerHTML = '<span class="decor-chargement-craie"></span><span>Préparation de l’atelier…</span>';
      document.body.appendChild(el);
    }
  }, 250);
}

async function ouvrir() {
  const timer = programmerIndicateur();
  chargerPolices().catch(() => {});
  attendrePolices();
  const canvas = document.getElementById('decor');
  const parametres = new URLSearchParams(location.search);
  const webgpu = 'gpu' in navigator && !/iPhone|iPad|iPod/.test(navigator.userAgent);
  const moteur = await creerMoteur(canvas, choisirRendu(parametres, webgpu));
  suivreTaille(document.getElementById('decor') ?? canvas, moteur);
  const connues = new Set();
  const etroit = matchMedia('(max-width: 700px)');
  const sansCartes = parametres.has('sans_cartes');
  const decorDefaut = parametres.get('scene') ?? document.body.dataset.decor;
  const peupler = (nom = decorDefaut) => peuplerDecor(moteur, nom, etroit.matches, connues, sansCartes);
  etroit.addEventListener('change', () => peupler());
  addEventListener('pointermove', (e) => {
    if (document.body.classList.contains('decor-actif')) {
      moteur.pointeur_decor((e.clientX / innerWidth) * 2 - 1, (e.clientY / innerHeight) * 2 - 1);
    }
  });
  await peupler();
  moteur.tick(0);
  brancherNavigation((nom) => peupler(nom));
  lancerBoucle(moteur);
  clearTimeout(timer);
  const indicateur = document.getElementById('decor-chargement');
  if (indicateur) indicateur.remove();
  document.body.classList.add('decor-actif');
}

attendrePolices();

ouvrir().catch((err) => {
  console.error('Erreur decor:', err);
  document.body.classList.add('decor-absent', 'polices-pretes');
});
