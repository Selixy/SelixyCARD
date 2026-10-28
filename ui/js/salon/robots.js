const PREFIXE = 'Robot ';

export function estNomRobot(pseudo) {
  return typeof pseudo === 'string' && pseudo.startsWith(PREFIXE);
}

export function prochainNomRobot(joueurs) {
  let numero = 1;
  while (joueurs.includes(`${PREFIXE}${numero}`)) {
    numero += 1;
  }
  return `${PREFIXE}${numero}`;
}

export function chargerRobots(stockage) {
  try {
    const brut = stockage?.getItem('robots_salon');
    const liste = JSON.parse(brut);
    return Array.isArray(liste) ? liste : [];
  } catch {
    return [];
  }
}

export function sauverRobots(stockage, liste) {
  try {
    stockage?.setItem('robots_salon', JSON.stringify(liste));
  } catch {
    return;
  }
}
