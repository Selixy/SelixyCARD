import { creerPartie, diffuser, jouerCarte, piocherCarte } from './actions.js';

export function creerSimulateur(emettre, moi = 'moi') {
  const s = creerPartie(moi);
  return {
    demarrer() {
      const joueurs = s.joueurs.map(j => j.pseudo);
      emettre({ t: 'salon', code: 'DEMO', hote: moi, joueurs, options: s.options, set: null });
      diffuser(s, emettre);
    },
    recevoir(message) {
      if (message.t === 'jouer') jouerCarte(s, message, emettre);
      else if (message.t === 'piocher') piocherCarte(s, emettre);
      else if (message.t === 'uno') s.joueurs[0].uno = true;
    },
  };
}

export function fabriqueBouchon(moi) {
  return () => {
    const socket = { readyState: 0, onopen: null, onmessage: null, onclose: null, send: texte => simulateur.recevoir(JSON.parse(texte)) };
    const simulateur = creerSimulateur(m => socket.onmessage?.({ data: JSON.stringify(m) }), moi);
    setTimeout(() => {
      socket.readyState = 1;
      socket.onopen?.();
      simulateur.demarrer();
    }, 50);
    return socket;
  };
}
