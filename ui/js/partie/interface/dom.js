export function element(balise, classe, texte = '') {
  const noeud = document.createElement(balise);
  if (classe) noeud.className = classe;
  noeud.textContent = texte;
  return noeud;
}

export function bouton(libelle, classe, action) {
  const noeud = element('button', `button ${classe}`, libelle);
  noeud.type = 'button';
  noeud.addEventListener('click', action);
  return noeud;
}
