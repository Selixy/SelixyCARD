import { creerRelanceur } from '../partie/relanceur.js';

const OUVERT = 1;

function brancherLiaison(socket, file, surMessage, surFermeture, relanceur, ralliement) {
  socket.onopen = () => {
    file.splice(0).forEach((texte) => socket.send(texte));
  };
  socket.onmessage = (evenement) => {
    try {
      surMessage(JSON.parse(evenement.data));
    } catch {
      surFermeture();
    }
  };
  socket.onclose = () => {
    surFermeture();
    if (ralliement.texte && !file.includes(ralliement.texte)) file.unshift(ralliement.texte);
    relanceur.planifier();
  };
}

export function ouvrir(url, SocketImpl, surMessage, surFermeture) {
  let socket = null;
  const file = [];
  const ralliement = { texte: null };
  const relanceur = creerRelanceur(() => relier());

  function relier() {
    if (!relanceur.etat.actif) return;
    try {
      socket = new SocketImpl(url);
      brancherLiaison(socket, file, surMessage, surFermeture, relanceur, ralliement);
    } catch {
      relanceur.planifier();
    }
  }

  relier();
  return {
    envoyer(message) {
      const texte = JSON.stringify(message);
      if (message.t === 'creer' || message.t === 'rejoindre') ralliement.texte = texte;
      if (socket?.readyState === OUVERT) socket.send(texte);
      else file.push(texte);
    },
    fermer() {
      relanceur.arreter();
      if (socket) {
        socket.onclose = null;
        socket.close();
      }
    },
  };
}
