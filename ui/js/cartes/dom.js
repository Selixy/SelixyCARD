export function el(balise, classe, texte) {
  const noeud = document.createElement(balise);
  if (classe) noeud.className = classe;
  if (texte !== undefined) noeud.textContent = texte;
  return noeud;
}

export function vider(noeud) {
  noeud.replaceChildren();
  return noeud;
}
