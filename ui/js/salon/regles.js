export const REGLES = [
  {
    cle: 'cumul_plus',
    libelle: 'Cumuler les +2 et les +4',
    aide: 'Un +2 répond à un +2, un +4 répond à un +4 : la pénalité s\'additionne jusqu\'au premier qui ne peut pas répondre.',
  },
  {
    cle: 'sept_zero',
    libelle: 'Règle du 7 et du 0',
    aide: 'Un 7 échange ta main avec un joueur de ton choix, un 0 fait tourner toutes les mains dans le sens du jeu.',
  },
  {
    cle: 'jump_in',
    libelle: 'Jouer hors de son tour',
    aide: 'Une carte strictement identique à celle de la défausse (même couleur, même valeur) peut être posée à tout moment.',
  },
  {
    cle: 'piocher_jusqua_jouable',
    libelle: 'Piocher jusqu\'à une carte jouable',
    aide: 'Sans cette règle, on pioche une seule carte. Avec elle, on continue de piocher jusqu\'à pouvoir jouer.',
  },
  {
    cle: 'penalite_uno',
    libelle: 'Pénalité si on oublie UNO',
    aide: 'Quand il ne reste qu\'une carte sans avoir dit UNO, un autre joueur peut te signaler avant le coup suivant : tu piocheras 2 cartes.',
  },
  {
    cle: 'contester_plus4',
    libelle: 'Contester le +4',
    aide: 'Le joueur visé peut contester un +4. Si le poseur avait la couleur demandée, il pioche 4 cartes ; sinon le contestataire en pioche 6.',
  },
];

export function creerLigne(document, regle) {
  const ligne = document.createElement('label');
  ligne.className = 'interrupteur';
  const texte = document.createElement('span');
  texte.className = 'regle-texte';
  const nom = document.createElement('span');
  nom.className = 'regle-nom';
  nom.textContent = regle.libelle;
  const aide = document.createElement('span');
  aide.className = 'regle-aide';
  aide.textContent = regle.aide;
  texte.append(nom, aide);
  const entree = document.createElement('input');
  entree.type = 'checkbox';
  entree.name = regle.cle;
  ligne.append(texte, entree);
  return ligne;
}

export function rendreRegles(conteneur, options, modifiable) {
  if (!conteneur.children.length) {
    REGLES.forEach((regle) => conteneur.append(creerLigne(conteneur.ownerDocument, regle)));
  }
  conteneur.querySelectorAll('input').forEach((entree) => {
    entree.checked = Boolean(options[entree.name]);
    entree.disabled = !modifiable;
  });
}

export function lireRegles(conteneur) {
  const options = {};
  conteneur.querySelectorAll('input').forEach((entree) => {
    options[entree.name] = entree.checked;
  });
  return options;
}
