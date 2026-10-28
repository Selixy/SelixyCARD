import { exporterWebp } from './exporter.js';
import { basculerCibles, lireCibles } from './cibles.js';
import { enregistrer, retirer } from './enregistrer.js';

async function executer(doc, statut, travail) {
  const erreur = doc.getElementById('erreur');
  erreur.textContent = '';
  doc.getElementById('statut').textContent = statut;
  try {
    await travail();
    doc.getElementById('statut').textContent = 'Enregistré.';
    const suivant = doc.getElementById('suivant');
    if (!suivant.hidden) {
      suivant.classList.replace('button--discret', 'button--primary');
      suivant.focus();
    }
  } catch (e) {
    doc.getElementById('statut').textContent = '';
    erreur.textContent = e.message;
  }
}

export function brancherBoutons(doc, etat, recadreur) {
  const cibles = doc.getElementById('cibles');
  const garder = (set) => {
    etat.set = set;
    etat.carte = set.cartes.find((c) => c.id === etat.carte.id) || etat.carte;
  };
  doc.getElementById('enregistrer').addEventListener('click', () => executer(doc, 'Envoi…', async () => {
    const blob = await exporterWebp(recadreur.courant().image, recadreur.courant().cadre);
    garder(await enregistrer(etat.appeler, etat.set, lireCibles(cibles, etat.carte), blob));
  }));
  doc.getElementById('retirer').addEventListener('click', () => executer(doc, 'Retrait…', async () => {
    garder(await retirer(etat.appeler, etat.set, lireCibles(cibles, etat.carte)));
  }));
  doc.getElementById('tout-cocher').addEventListener('click', () => basculerCibles(cibles, true));
  doc.getElementById('tout-decocher').addEventListener('click', () => basculerCibles(cibles, false));
}
