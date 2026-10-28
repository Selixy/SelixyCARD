# ui/js/cartes

Bibliothèque des sets (`ui/cartes/index.html`) et dessin des cartes.

- `emplacements.js` : les 108 emplacements du contrat, mêmes identifiants que `commun::uno::paquet`. `jumeaux(id)` donne les emplacements
  de même valeur (l'autre exemplaire et les autres couleurs) pour appliquer une illustration à plusieurs cartes d'un coup.
- `dessin/` : fonctions pures de dessin, sans accès au DOM. `dessinerCarte(ctx, carte, image|null, largeur, hauteur)` peint le cadre
  commun (bord de couleur, quadrants pour les jokers, valeur dans deux coins, valeur centrale seulement sans illustration) ;
  `dessinerDos(ctx, image|null, l, h)` fait de même pour le dos. `image` est n'importe quelle source acceptée par `drawImage`
  (ImageBitmap, canvas, HTMLImageElement) : l'agent du rendu 3D peut donc l'appeler sur un `OffscreenCanvas` pour produire ses textures.
  `miseEnPageCadre` est le seul endroit où vivent les proportions.
- `service-sets.js`, `images.js` : appels HTTP. `appeler` est toujours injecté en paramètre (testable sans réseau). `envoyerImage` passe le
  `Blob` tel quel à `appeler('POST', '/api/images', blob)` : `ui/js/api.js` ne doit ni le sérialiser en JSON ni forcer un autre
  `Content-Type` (le blob porte `image/webp`).
- `bibliotheque.js` : routage par `location.hash` (`#set=<id>`), liste ou grille d'un set.
- `session.js` : `GET /api/moi` ; un 401 donne `null` (pas d'auteur : lecture seule, pas de bouton Supprimer).

Pièges : `ctx.roundRect` exige un navigateur récent. Les vignettes sont des canvas 160×240 redessinés quand l'image est chargée ;
`chargerImage` met en cache par sha256, une image partagée par plusieurs cartes n'est téléchargée qu'une fois.
