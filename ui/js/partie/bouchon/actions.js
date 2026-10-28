import { genererPaquet, jouable } from './paquet.js';

export function creerPartie(moi) {
  const paquet = genererPaquet();
  const main = paquet.splice(0, 7);
  const defausse = paquet.splice(paquet.findIndex(c => c.type === 'nombre'), 1)[0];
  const joueurs = [moi, 'Ana', 'Bob', 'Chloé'].map(pseudo => ({ pseudo, nb_cartes: 7, uno: false }));
  const options = { sept_zero: true, contester_plus4: true };
  return { moi, paquet, main, defausse, couleur: defausse.couleur, sens: 1, penalite: 0, courant: moi, joueurs, options };
}

export function diffuser(s, emettre) {
  emettre({ t: 'main', cartes: s.main });
  emettre({
    t: 'etat', defausse: s.defausse, couleur_courante: s.couleur, sens: s.sens, courant: s.courant,
    joueurs: s.joueurs, pioche: s.paquet.length, penalite: s.penalite,
  });
}

export function tourDesBots(s, emettre, rang = 1) {
  setTimeout(() => {
    const bot = s.joueurs[rang];
    const carte = Math.random() > 0.4 ? s.paquet.pop() : null;
    if (carte) {
      s.defausse = carte;
      s.couleur = carte.couleur ?? 'rouge';
      bot.nb_cartes--;
    } else {
      bot.nb_cartes++;
    }
    if (bot.nb_cartes <= 0) return emettre({ t: 'fin', gagnant: bot.pseudo });
    s.courant = s.joueurs[(rang + 1) % s.joueurs.length].pseudo;
    diffuser(s, emettre);
    if (rang + 1 < s.joueurs.length) tourDesBots(s, emettre, rang + 1);
  }, 700);
}

export function jouerCarte(s, message, emettre) {
  const carte = s.main.find(c => c.id === message.carte);
  if (s.courant !== s.moi || !carte || !jouable(carte, s.defausse, s.couleur)) {
    return emettre({ t: 'erreur', message: "Cette carte n'est pas jouable maintenant" });
  }
  s.main = s.main.filter(c => c !== carte);
  s.defausse = carte;
  s.couleur = message.couleur ?? carte.couleur;
  s.joueurs[0].nb_cartes = s.main.length;
  s.courant = s.joueurs[1].pseudo;
  diffuser(s, emettre);
  if (s.main.length === 0) return emettre({ t: 'fin', gagnant: s.moi });
  tourDesBots(s, emettre);
}

export function piocherCarte(s, emettre) {
  if (s.courant !== s.moi) return emettre({ t: 'erreur', message: "Ce n'est pas votre tour" });
  s.main.push(s.paquet.pop());
  s.joueurs[0].nb_cartes = s.main.length;
  s.courant = s.joueurs[1].pseudo;
  diffuser(s, emettre);
  tourDesBots(s, emettre);
}
