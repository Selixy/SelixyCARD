# ui/js/editeur

Éditeur d'une carte (`ui/cartes/editeur.html?set=<id>&carte=<id|dos>`).

- `recadrage.js`, `zoom.js` : calculs purs du cadre 2:3 en coordonnées de l'image source `{x, y, l}` (la hauteur vaut toujours
  `1,5 × l`). Tout est borné : le cadre ne sort jamais de l'image, le zoom va de 1× (cadre maximal) à 8×.
- `vue-recadrage.js` : image entière assombrie, cadre clair avec règle des tiers. `gestes.js` : glisser (pointeur), pincement (deux
  pointeurs), molette ; `touch-action: none` est indispensable sur le canvas, sinon le navigateur mobile fait défiler la page.
- `recadreur.js` assemble vue, gestes et curseur de zoom et notifie l'aperçu à chaque changement.
- `apercu.js` : l'aperçu appelle `dessinerCarte` avec le recadrage courant, donc ce qu'on voit est ce qui sera joué.
- `exporter.js` : WebP 512×768 qualité 0,9 ; erreur claire si le navigateur retombe sur du PNG (Safari ancien).
- `cibles.js` : cases des emplacements jumeaux. `enregistrer.js` : POST de l'image puis PUT du set complet avec le sha256 reçu pour
  l'emplacement courant et tous les emplacements cochés (`appliquerImage`, pure). Le dos s'édite avec `carte=dos`.
- `boutons.js` : actions et messages ; toute erreur du serveur s'affiche telle quelle (déjà en français).
- Seul l'auteur du set peut enregistrer ; les autres voient l'éditeur en lecture seule.

Pièges : le PUT renvoie le set complet, on le garde comme nouvelle référence (sinon un second enregistrement écraserait le premier).
Le canvas de recadrage est carré (640×640) quelle que soit l'image ; la mise à l'échelle se fait par `echelleAffichage`.
