import { estNomRobot, prochainNomRobot } from './robots.js';

export function ajouterRobot(robots, joueurs, limite = 10) {
  if (joueurs.length >= limite) return null;
  const nom = prochainNomRobot(joueurs);
  return { nom, robots: [...robots, nom] };
}

export function retirerRobot(robots, nom) {
  return robots.filter((r) => r !== nom);
}

export function estRetirable(pseudo, robots, estHote) {
  return estHote && estNomRobot(pseudo) && robots.includes(pseudo);
}
