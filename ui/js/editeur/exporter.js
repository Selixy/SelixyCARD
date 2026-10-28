export const EXPORT_L = 512;
export const EXPORT_H = 768;

export function rogner(image, cadre, largeur, hauteur) {
  const canvas = document.createElement('canvas');
  canvas.width = largeur;
  canvas.height = hauteur;
  canvas.getContext('2d').drawImage(image, cadre.x, cadre.y, cadre.l, cadre.l * 1.5, 0, 0, largeur, hauteur);
  return canvas;
}

export function exporterWebp(image, cadre) {
  const canvas = rogner(image, cadre, EXPORT_L, EXPORT_H);
  return new Promise((resolve, reject) => {
    canvas.toBlob((blob) => {
      if (blob && blob.type === 'image/webp') resolve(blob);
      else reject(new Error("Ce navigateur ne sait pas exporter en WebP."));
    }, 'image/webp', 0.9);
  });
}
