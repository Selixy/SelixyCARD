export async function chargerImage(sha256) {
  const reponse = await fetch(`/api/images/${sha256}.webp`);
  if (!reponse.ok) throw new Error(`image ${sha256} introuvable`);
  return createImageBitmap(await reponse.blob());
}
