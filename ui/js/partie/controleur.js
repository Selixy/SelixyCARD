import { appliquer, etatInitial } from './etat/reducteur.js';
import { estCliquable, nouvelleCarte, peutPiocher } from './etat/possibles.js';
import { versTable } from './etat/vers-table.js';
import { preparerCoup } from './coup.js';
import { afficher } from './interface/vue.js';

const DUREE_ECLAIR = 1400;

export function creerControleur({ moi, connexion, moteur, fabrique, zones }) {
  let partie = etatInitial(moi);
  let attentePioche = null;
  const envoyer = message => connexion().envoyer(message);
  const actions = { signaler: pseudo => envoyer({ t: 'signaler', pseudo }) };

  function redessiner() {
    moteur.definir_table(versTable(partie));
    afficher(zones, partie, actions);
  }

  zones.uno.addEventListener('click', () => envoyer({ t: 'uno' }));
  zones.contester.addEventListener('click', () => envoyer({ t: 'contester' }));
  zones.jouerPiochee.addEventListener('click', () => preparerCoup(partie.main.find(c => c.id === partie.piochee), partie, zones.selecteur).then(m => m && envoyer(m)));
  zones.garder.addEventListener('click', () => envoyer({ t: 'piocher' }));

  return {
    surMessage(message) {
      const avant = partie.main;
      partie = appliquer(partie, message);
      if (message.t === 'main' && attentePioche) {
        attentePioche = null;
        const tiree = partie.piochee ? null : nouvelleCarte(avant, partie.main);
        if (tiree) {
          partie = { ...partie, eclair: tiree.id };
          setTimeout(() => { partie = { ...partie, eclair: null }; redessiner(); }, DUREE_ECLAIR);
        }
      }
      partie.main.forEach(carte => fabrique.carte(carte));
      if (partie.etat?.defausse) fabrique.carte(partie.etat.defausse);
      if (partie.salon || partie.etat) fabrique.dos(partie.salon?.set?.dos ?? null);
      redessiner();
    },
    surClic(indice) {
      if (indice === -2 && peutPiocher(partie)) {
        attentePioche = true;
        envoyer({ t: 'piocher' });
      } else if (indice >= 0 && estCliquable(partie, partie.main[indice])) preparerCoup(partie.main[indice], partie, zones.selecteur).then(m => m && envoyer(m));
    },
  };
}
