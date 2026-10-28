# ui/js/partie

Page `ui/partie/` : la table de jeu. Le moteur wasm (`../moteur/`) rend ; ce dossier gère le protocole `/ws/partie` et l'interface.

- `principal.js` : assemblage. `?rendu=webgl2|webgpu` force le rendu ; `?code=XXXX` envoie `rejoindre` à l'ouverture ;
  `?bouchon` remplace le serveur par un simulateur local ; `?pseudo=` fixe le pseudo (sinon `GET /api/moi` via `../api.js`,
  sinon `moi`).
- `connexion.js` : la fabrique de socket est **injectée** (`fabriqueWebSocket` ou `fabriqueBouchon`), ce qui rend le reste testable.
- `controleur.js` : un message serveur met à jour l'état (`etat/reducteur.js`), demande les textures manquantes, appelle
  `moteur.definir_table` puis redessine l'interface ; un clic sur une carte envoie `jouer` (couleur demandée pour un joker, joueur
  cible pour un 7 si `options.sept_zero`), un clic sur la pioche envoie `piocher`.
- `etat/` : réducteur pur, conversion vers le JSON de `definir_table`, règles d'affichage (joker, 7, contestation, bouton UNO
  seulement à ton tour, avec exactement 2 cartes en main dont au moins une jouable). Le client ne vérifie pas la légalité d'un coup : le serveur est autoritaire, son `erreur` est affichée.
- `textures/poudre.js` : quatre textures de poudre de craie blanche (`poudre:0` à `poudre:3`), peintes une fois au chargement
  (amas granuleux à bord lobé, traînées), que le moteur teinte pour le dépôt d'un joker.
- `textures/` : une texture 256×384 par carte, produite à la demande (illustration `GET /api/images/<sha>.webp` si elle existe,
  puis `dessin.dessinerCarte`, puis `createImageBitmap`, puis `moteur.charger_texture`). `dessin-minimal.js` est un remplaçant
  provisoire de `ui/js/cartes/dessin/` : même signature `dessinerCarte(ctx, carte, image|null, largeur, hauteur)`, plus
  `dessinerDos(ctx, image|null, largeur, hauteur)`. À l'intégration, changer l'import dans `principal.js`.
- Pas d'en-tête de site sur la table : seul un bouton « Quitter » en haut à droite, confirmé en deux temps par un `<details>`
  (aucun JS). Quitter renvoie au salon ; la fermeture du websocket suffit au serveur pour retirer le joueur.
- `interface/` : bandeau (tour, sens, couleur, pénalité), adversaires (cartes, UNO, bouton Signaler quand un joueur à 1 carte n'a pas
  dit UNO), sélecteurs en surimpression, écran de fin, pointeur (coordonnées client vers pixels du canvas).
- `bouchon/` : simulateur de serveur (salon, main, état, bots qui jouent toutes les 700 ms) pour développer sans serveur.

- `salon-set.js` : le message `salon` ne porte que l'identifiant du set ; on lit le set par l'API pour obtenir son dos illustré,
  sinon la table garderait le dos par défaut. La fabrique de textures distingue chaque dos (`dos:<sha>`) pour pouvoir remplacer le
  dos par défaut chargé avant l'arrivée du set.
- Sans `?code=` (ni `?bouchon`), la page affiche « Aucune partie en cours » et renvoie au salon au lieu d'une table vide :
  aucune connexion n'est ouverte, le bandeau est masqué et le décor de craie s'affiche (la table n'est pas définie).
