import http from 'node:http';
import fs from 'node:fs';
import path from 'node:path';
import { spawn } from 'node:child_process';

const PORT = 4328;
const RACINE = path.resolve('ui');
const STATIONS = ['accueil', 'cartes', 'editeur', 'salon', 'compte'];

function creerServeur() {
  return http.createServer((req, res) => {
    let chemin = req.url.split('?')[0];
    if (chemin === '/') chemin = '/index.html';
    const fichier = path.join(RACINE, chemin);
    if (fs.existsSync(fichier) && fs.statSync(fichier).isFile()) {
      if (fichier.endsWith('.wasm')) res.setHeader('Content-Type', 'application/wasm');
      else if (fichier.endsWith('.js')) res.setHeader('Content-Type', 'application/javascript');
      else if (fichier.endsWith('.css')) res.setHeader('Content-Type', 'text/css');
      else if (fichier.endsWith('.html')) res.setHeader('Content-Type', 'text/html');
      res.end(fs.readFileSync(fichier));
    } else {
      res.statusCode = 404;
      res.end('Introuvable');
    }
  });
}

async function attendreTabs(port) {
  for (let i = 0; i < 40; i++) {
    try {
      const res = await fetch(`http://127.0.0.1:${port}/json/list`);
      const tabs = await res.json();
      const tab = tabs.find((t) => t.type === 'page');
      if (tab) return tab;
    } catch {}
    await new Promise((r) => setTimeout(r, 150));
  }
  throw new Error('Connexion Chrome impossible');
}

function attendreEtCapturer(ws, destination, largeur, hauteur) {
  const requeteSonde = JSON.stringify({
    id: 2,
    method: 'Runtime.evaluate',
    params: { expression: 'document.body.classList.contains("decor-actif")' },
  });
  return new Promise((resolve, reject) => {
    ws.onopen = () => {
      ws.send(JSON.stringify({ id: 1, method: 'Runtime.enable' }));
      ws.send(JSON.stringify({
        id: 4,
        method: 'Emulation.setDeviceMetricsOverride',
        params: { width: largeur, height: hauteur, deviceScaleFactor: 1, mobile: largeur <= 700 },
      }));
      ws.send(requeteSonde);
    };
    ws.onmessage = (event) => {
      const msg = JSON.parse(event.data);
      if (msg.id === 2) {
        if (msg.result?.result?.value) {
          ws.send(JSON.stringify({
            id: 3,
            method: 'Runtime.evaluate',
            params: { expression: 'document.querySelector("header")?.remove(); document.querySelector("main")?.remove();' },
          }));
          setTimeout(() => ws.send(JSON.stringify({
            id: 10,
            method: 'Page.captureScreenshot',
            params: { format: 'webp', quality: 95 },
          })), 300);
        } else setTimeout(() => ws.send(requeteSonde), 150);
      }
      if (msg.id === 10) {
        fs.mkdirSync(path.dirname(destination), { recursive: true });
        fs.writeFileSync(destination, Buffer.from(msg.result.data, 'base64'));
        resolve();
      }
    };
    ws.onerror = reject;
  });
}

async function capturerPage(largeur, hauteur, scene, destination, port) {
  const chrome = spawn('chromium', [
    '--headless=new',
    '--no-sandbox',
    `--remote-debugging-port=${port}`,
    `--user-data-dir=/tmp/chrome-cuire-${port}`,
    `--window-size=${largeur},${hauteur}`,
    `http://127.0.0.1:${PORT}/?sans_cartes=1&rendu=webgl2&scene=${scene}`,
  ]);
  try {
    const tab = await attendreTabs(port);
    const ws = new WebSocket(tab.webSocketDebuggerUrl);
    await attendreEtCapturer(ws, destination, largeur, hauteur);
    ws.close();
  } finally {
    chrome.kill();
  }
}

async function executer() {
  const serveur = creerServeur();
  await new Promise((resolve) => serveur.listen(PORT, resolve));
  let port = 9230;
  try {
    for (const nom of STATIONS) {
      port += 1;
      await capturerPage(3840, 1080, nom, `ui/images/fonds/fond-${nom}.webp`, port);
      port += 1;
      await capturerPage(700, 1024, nom, `ui/images/fonds/fond-${nom}-mobile.webp`, port);
    }
    fs.copyFileSync('ui/images/fonds/fond-accueil.webp', 'ui/images/fond-bureau.webp');
    fs.copyFileSync('ui/images/fonds/fond-accueil-mobile.webp', 'ui/images/fond-bureau-mobile.webp');
  } finally {
    serveur.close();
  }
}

executer().catch((err) => {
  console.error(err);
  process.exit(1);
});
