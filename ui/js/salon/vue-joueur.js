import { estNomRobot } from './robots.js';
import { estRetirable } from './gestion-robots.js';

export function creerLigneJoueur(document, nom, salon, pseudo, robots, onRetirer) {
  const ligne = document.createElement('li');
  ligne.textContent = nom;
  if (nom === salon.hote) {
    const pastille = document.createElement('span');
    pastille.className = 'pastille';
    pastille.textContent = 'hôte';
    ligne.append(pastille);
  } else if (estNomRobot(nom)) {
    const pastille = document.createElement('span');
    pastille.className = 'pastille pastille--robot';
    pastille.textContent = 'robot';
    ligne.append(pastille);
  }
  if (nom === pseudo) ligne.setAttribute('aria-current', 'true');
  if (estRetirable(nom, robots, salon.hote === pseudo)) {
    const bouton = document.createElement('button');
    bouton.type = 'button';
    bouton.className = 'salon-retirer-robot';
    bouton.textContent = '×';
    bouton.setAttribute('aria-label', `Retirer ${nom}`);
    bouton.addEventListener('click', (e) => {
      e.stopPropagation();
      onRetirer?.(nom);
    });
    ligne.append(bouton);
  }
  return ligne;
}
