import { ouvrir } from './liaison.js';
import { rejoindre } from './messages.js';
import { urlSocket } from './adresses.js';

export function connecterLienRobot(deps, code, nom) {
  const lien = ouvrir(urlSocket(deps.location), deps.SocketImpl, () => {}, () => {});
  lien.envoyer(rejoindre(nom, code));
  return lien;
}

export function fermerLiens(liens) {
  for (const lien of liens.values()) lien.fermer();
  liens.clear();
}
