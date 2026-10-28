import { connecterLienRobot, fermerLiens } from './liens-robots.js';
import { ajouterRobot, retirerRobot } from './gestion-robots.js';
import { sauverRobots } from './robots.js';

export function creerHoteRobots(session) {
  const liens = new Map();

  return {
    ajouter() {
      const code = session.etat.salon?.code;
      if (!code) return;
      const res = ajouterRobot(session.robots, session.etat.salon.joueurs);
      if (!res) return;
      session.robots = res.robots;
      sauverRobots(session.deps.stockage, session.robots);
      liens.set(res.nom, connecterLienRobot(session.deps, code, res.nom));
    },
    retirer(nom) {
      liens.get(nom)?.fermer();
      liens.delete(nom);
      session.robots = retirerRobot(session.robots, nom);
      sauverRobots(session.deps.stockage, session.robots);
    },
    fermer() {
      fermerLiens(liens);
    },
  };
}
