import { appeler } from '../api.js';
import { demarrer as demarrerSalon } from './principal.js';

export function demarrer() {
  return demarrerSalon({ appeler, document, location, SocketImpl: WebSocket, stockage: window.sessionStorage });
}

if (!window.__navigation_active) demarrer();
