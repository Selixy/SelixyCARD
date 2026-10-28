export function messageRejoindre(parametres, moi) {
  if (!parametres.has('code')) return null;
  return { t: 'rejoindre', code: parametres.get('code'), pseudo: moi };
}
