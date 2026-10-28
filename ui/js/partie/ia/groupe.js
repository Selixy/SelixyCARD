import { creerAgentRobot } from './agent.js';
import { creerConnexion, fabriqueWebSocket } from '../connexion.js';

export function lancerRobots({ code, robots, urlWs, delaiMs = 600 }) {
  const instances = [];
  for (const pseudo of robots) {
    let connexion = null;
    let agent = null;
    const fabrique = () => fabriqueWebSocket(urlWs)();
    connexion = creerConnexion(fabrique, {
      ouverte: () => {
        connexion.envoyer({ t: 'rejoindre', code, pseudo });
      },
      message: (m) => agent?.recevoir(m),
    });
    agent = creerAgentRobot({ pseudo, connexion, delaiMs });
    instances.push({ pseudo, agent, connexion });
  }

  return {
    fermer() {
      for (const inst of instances) {
        inst.agent.arreter();
        inst.connexion.fermer();
      }
    },
  };
}
