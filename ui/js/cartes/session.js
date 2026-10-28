export async function lirePseudo(appeler) {
  try {
    return (await appeler('GET', '/api/moi')).pseudo;
  } catch {
    return null;
  }
}

export const lireId = () => new URLSearchParams(location.hash.slice(1)).get('set');
