import { afficherSet, nbIllustrees } from './grille-set.js';
import { afficherListe } from './liste-sets.js';
import { lireId, lirePseudo } from './session.js';
import { chargerSet, creerSet, listerSets, supprimerSet } from './service-sets.js';
import { vider } from './dom.js';

async function montrerListe(ctx) {
  const { appeler, doc, pseudo } = ctx;
  doc.getElementById('vue-set').hidden = true;
  doc.getElementById('vue-liste').hidden = false;
  if (!pseudo) {
    vider(doc.getElementById('sets'));
    return;
  }
  const actions = {
    ouvrir: (id) => { location.hash = `set=${id}`; },
    supprimer: async (set) => {
      if (!confirm(`Supprimer le set « ${set.nom} » ?`)) return;
      await supprimerSet(appeler, set.id);
      montrerListe(ctx);
    },
  };
  afficherListe(doc.getElementById('sets'), await listerSets(appeler), pseudo, actions);
}

async function montrerSet(ctx, id) {
  const { appeler, doc } = ctx;
  const set = await chargerSet(appeler, id);
  doc.getElementById('vue-liste').hidden = true;
  doc.getElementById('vue-set').hidden = false;
  doc.getElementById('set-titre').textContent = set.nom;
  doc.getElementById('set-info').textContent = `par ${set.auteur} · ${nbIllustrees(set.cartes)} / 108 illustrées`;
  afficherSet(doc.getElementById('cartes'), set);
}

function brancherCreation(ctx) {
  const { appeler, doc } = ctx;
  doc.getElementById('creer-set').addEventListener('submit', async (evenement) => {
    evenement.preventDefault();
    const champ = doc.getElementById('nom-set');
    const nom = champ.value.trim();
    if (!nom) return;
    try {
      const set = await creerSet(appeler, nom);
      champ.value = '';
      location.hash = `set=${set.id}`;
    } catch (e) {
      doc.getElementById('erreur').textContent = e.message;
    }
  });
}

async function router(ctx) {
  const erreur = ctx.doc.getElementById('erreur');
  erreur.textContent = '';
  try {
    const id = lireId();
    if (id) await montrerSet(ctx, id);
    else await montrerListe(ctx);
  } catch (e) {
    erreur.textContent = e.message;
  }
}

let routeurActif = null;

export async function demarrerBibliotheque(appeler, doc) {
  const ctx = { appeler, doc, pseudo: await lirePseudo(appeler) };
  doc.getElementById('creer-set').hidden = !ctx.pseudo;
  doc.getElementById('invite-connexion').hidden = Boolean(ctx.pseudo);
  brancherCreation(ctx);
  if (routeurActif) removeEventListener('hashchange', routeurActif);
  routeurActif = () => router(ctx);
  addEventListener('hashchange', routeurActif);
  router(ctx);
}
