export const lireImage = (fichier) => createImageBitmap(fichier);

export function brancherFichier(doc, auFichier) {
  const entree = doc.getElementById('fichier');
  const zone = doc.getElementById('zone');
  const prendre = (fichier) => fichier && fichier.type.startsWith('image/') && auFichier(fichier);
  entree.addEventListener('change', () => prendre(entree.files[0]));
  zone.addEventListener('dragover', (e) => {
    e.preventDefault();
    zone.classList.add('zone--survol');
  });
  zone.addEventListener('dragleave', () => zone.classList.remove('zone--survol'));
  zone.addEventListener('drop', (e) => {
    e.preventDefault();
    zone.classList.remove('zone--survol');
    prendre(e.dataTransfer.files[0]);
  });
}
