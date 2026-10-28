import { creerRelanceur } from './relanceur.js';

export function fabriqueWebSocket(adresse) {
  return () => new WebSocket(adresse);
}

function brancherSocket(socket, file, ecouteur, relanceur) {
  socket.onopen = () => {
    ecouteur.ouverte();
    file.splice(0).forEach((m) => socket.send(JSON.stringify(m)));
  };
  socket.onclose = () => {
    ecouteur.fermee?.();
    relanceur.planifier();
  };
  socket.onmessage = (e) => {
    try {
      ecouteur.message(JSON.parse(e.data));
    } catch {
      ecouteur.message({ t: 'erreur', message: 'Message illisible du serveur' });
    }
  };
}

export function creerConnexion(fabriqueSocket, ecouteur) {
  const etat = { socket: null, file: [] };
  const relanceur = creerRelanceur(() => {
    if (!relanceur.etat.actif) return;
    try {
      etat.socket = fabriqueSocket();
      brancherSocket(etat.socket, etat.file, ecouteur, relanceur);
    } catch {
      relanceur.planifier();
    }
  });

  relanceur.lancer();
  return {
    envoyer(message) {
      if (etat.socket?.readyState === 1) etat.socket.send(JSON.stringify(message));
      else etat.file.push(message);
    },
    fermer() {
      relanceur.arreter();
      if (etat.socket) {
        etat.socket.onclose = null;
        etat.socket.close();
      }
    },
  };
}
