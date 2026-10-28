let enCours = false;

function lienInterne(a) {
  if (!a || a.origin !== location.origin || a.target) return null;
  const chemin = a.pathname;
  if (chemin.startsWith('/partie') || chemin.startsWith('/api') || chemin.startsWith('/ws')) return null;
  return a.href;
}

function synchroniserFeuilles(doc, url) {
  const existants = new Set([...document.querySelectorAll('link[rel="stylesheet"]')].map((l) => l.href));
  for (const lien of doc.querySelectorAll('link[rel="stylesheet"]')) {
    const cible = new URL(lien.getAttribute('href'), url).href;
    if (!existants.has(cible)) {
      const nouveau = document.createElement('link');
      nouveau.rel = 'stylesheet';
      nouveau.href = cible;
      document.head.append(nouveau);
    }
  }
}

function animerSortie(element) {
  if (!element) return Promise.resolve();
  element.classList.add('ui-sortie');
  return new Promise((resoudre) => {
    setTimeout(resoudre, 190);
    element.addEventListener('animationend', () => resoudre(), { once: true });
  });
}

async function chargerPage(url, basculer) {
  const ancienMain = document.querySelector('main');
  const chemin = new URL(url, location.origin).pathname;
  const decorEstime = chemin.includes('/editeur') ? 'editeur'
    : chemin.startsWith('/cartes') ? 'cartes'
    : chemin.startsWith('/salon') ? 'salon'
    : chemin.startsWith('/compte') ? 'compte'
    : (chemin === '/' || chemin.endsWith('/index.html')) ? 'accueil' : null;
  let promesseDecor = decorEstime ? basculer(decorEstime) : null;
  const [reponse] = await Promise.all([fetch(url), animerSortie(ancienMain)]);
  if (!reponse.ok) return location.assign(url);
  const doc = new DOMParser().parseFromString(await reponse.text(), 'text/html');
  const decor = doc.body?.dataset.decor;
  if (!decor) return location.assign(url);
  if (!promesseDecor || decor !== decorEstime) promesseDecor = basculer(decor);
  synchroniserFeuilles(doc, url);
  document.title = doc.title;
  const nouveauMain = doc.querySelector('main');
  if (ancienMain && nouveauMain) {
    nouveauMain.classList.add('ui-entree');
    nouveauMain.addEventListener('animationend', () => nouveauMain.classList.remove('ui-entree'), { once: true });
    ancienMain.replaceWith(nouveauMain);
  }
  const nouvelEntete = doc.querySelector('.header');
  const ancienEntete = document.querySelector('.header');
  if (ancienEntete && nouvelEntete) ancienEntete.replaceWith(nouvelEntete);
  document.body.className = doc.body.className;
  document.body.classList.add('decor-actif', 'polices-pretes');
  document.body.dataset.decor = decor;
  await promesseDecor;
  for (const s of doc.querySelectorAll('script[type="module"]')) {
    const src = s.getAttribute('src');
    if (src && !src.includes('decor/page.js')) {
      const cible = new URL(src, url).href;
      const mod = await import(cible);
      await mod.demarrer?.();
    }
  }
}

export function brancherNavigation(basculer) {
  window.__navigation_active = true;
  document.addEventListener('click', (e) => {
    const a = e.target.closest('a');
    const href = lienInterne(a);
    if (!href || href === location.href || a.getAttribute('href')?.startsWith('#') || enCours) return;
    e.preventDefault();
    history.pushState(null, '', href);
    enCours = true;
    chargerPage(href, basculer).finally(() => { enCours = false; });
  });
  window.addEventListener('popstate', () => {
    enCours = true;
    chargerPage(location.href, basculer).finally(() => { enCours = false; });
  });
}
