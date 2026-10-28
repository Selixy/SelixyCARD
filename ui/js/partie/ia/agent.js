import { evaluerDecision } from './decision.js';

export function creerAgentRobot({ pseudo, connexion, delaiMs = 600 }) {
  const memoire = { pseudo, main: [], jouables: [], piochee: null, etat: null, options: null };
  let minuterie = null;

  function agir() {
    if (!memoire.etat) return;
    const coup = evaluerDecision(memoire);
    if (!coup) return;
    connexion.envoyer(coup);
  }

  function planifier() {
    if (minuterie) clearTimeout(minuterie);
    minuterie = setTimeout(agir, delaiMs);
  }

  return {
    recevoir(message) {
      if (message.t === 'salon') memoire.options = message.options;
      else if (message.t === 'etat') {
        memoire.etat = message;
        planifier();
      } else if (message.t === 'main') {
        memoire.main = message.cartes ?? [];
        memoire.jouables = message.jouables ?? [];
        memoire.piochee = message.piochee ?? null;
        planifier();
      }
    },
    arreter() {
      if (minuterie) clearTimeout(minuterie);
    },
  };
}
