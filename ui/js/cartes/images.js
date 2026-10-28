const cache = new Map();

export const urlImage = (sha) => `/api/images/${sha}.webp`;

export async function envoyerImage(appeler, blob) {
  const reponse = await appeler('POST', '/api/images', blob);
  return reponse.image;
}

export function chargerImage(sha) {
  if (!cache.has(sha)) {
    cache.set(sha, new Promise((resolve) => {
      const image = new Image();
      image.onload = () => resolve(image);
      image.onerror = () => resolve(null);
      image.src = urlImage(sha);
    }));
  }
  return cache.get(sha);
}

export function appliquerImage(set, cibles, sha) {
  const copie = { ...set, cartes: set.cartes.map((c) => ({ ...c })) };
  if (cibles.includes('dos')) copie.dos = sha;
  for (const carte of copie.cartes) {
    if (cibles.includes(carte.id)) carte.image = sha;
  }
  return copie;
}
