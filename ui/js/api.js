function preparer(corps) {
  if (corps === undefined) return {};
  if (typeof Blob !== 'undefined' && corps instanceof Blob) {
    return { headers: { 'Content-Type': corps.type }, body: corps };
  }
  return { headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(corps) };
}

async function lireJson(reponse) {
  try {
    return await reponse.json();
  } catch {
    return null;
  }
}

export async function appeler(methode, chemin, corps) {
  let reponse;
  try {
    reponse = await fetch(chemin, { method: methode, credentials: 'same-origin', ...preparer(corps) });
  } catch {
    throw new Error('Le serveur est injoignable.');
  }
  if (reponse.status === 204) return null;
  const donnees = await lireJson(reponse);
  if (!reponse.ok) throw new Error(donnees?.erreur ?? `Erreur ${reponse.status}.`);
  return donnees;
}
