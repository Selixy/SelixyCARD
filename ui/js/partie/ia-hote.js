import { chargerRobots } from '../salon/robots.js';
import { lancerRobots } from './ia/groupe.js';

export function demarrerRobotsPartie(parametres, stockage) {
  if (parametres.has('bouchon') || !parametres.has('code')) return null;
  const code = parametres.get('code');
  const robots = chargerRobots(stockage);
  if (!robots.length) return null;
  const protocole = location.protocol === 'https:' ? 'wss' : 'ws';
  const urlWs = `${protocole}://${location.host}/ws`;
  return lancerRobots({ code, robots, urlWs });
}
