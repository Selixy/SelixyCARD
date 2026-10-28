export function normaliserCode(code) {
  return String(code ?? '').trim().toUpperCase();
}

export function urlSocket(location) {
  const protocole = location.protocol === 'https:' ? 'wss' : 'ws';
  return `${protocole}://${location.host}/ws`;
}

export function urlPartie(code, pseudo = '') {
  const params = new URLSearchParams({ code });
  if (pseudo) params.set('pseudo', pseudo);
  return `/partie/?${params.toString()}`;
}
