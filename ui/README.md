# ui

Modules ES natifs, ni npm ni build. `wasm/` est généré par `outils/construire-wasm.sh` (ignoré par git).

- `principal.js` : orchestration et boucle `requestAnimationFrame`.
- `moteur/choix-rendu.js` : WebGPU si `navigator.gpu`, sinon WebGL2 ; `?rendu=` force le choix. Testé dans `tests/ui/`.
- `moteur/chargeur.js` : si WebGPU est annoncé mais qu'aucun adaptateur n'est obtenu, repli silencieux sur WebGL2.
- `moteur/taille.js` : résolution du canvas plafonnée à 2× `devicePixelRatio`.
- `entrees.js` : relâche toutes les touches à la perte de focus (sinon le cube continue d'avancer).
- `reseau.js` : WebSocket binaire, reconnexion automatique.
- `css/atelier/` : thème « Atelier d'illustrateur », scrapbooking sombre posé sur l'ardoise du décor 3D. Fond d'ardoise
  (`--ardoise`, `--bruit-ardoise`) ; feuilles en carton sombre (`--papier`, `--papier-clair`) cousues d'un pointillé doré, encre
  claire (`--encre`), titres or (`--or`), encre gel (`--blanc-gel`), washi saturés, punaises dorées, tampons. Les jetons ont le
  sens « surface / encre » : `--papier` est sombre et `--encre` claire. `sombre.css` regroupe les surcharges propres au thème sombre
  (fonds carton sous les titres posés sur le décor, tampons, zone de dépôt de l'éditeur). Polices Google Fonts Caveat, Caveat Brush et Lora ;
  une feuille CSS par page dans `css/`.
- En-têtes réduits au nécessaire : sur l'accueil, la seule étiquette de compte (« Connexion », ou le pseudo avec sa médaille d'avatar, voir `js/compte/`) (le héros et ses feuilles Mes cartes, Jouer, Amis et Communauté — ces deux
  dernières marquées « bientôt », pas encore de page — font la navigation), la seule étiquette « Accueil », en haut à droite,
  sur cartes, éditeur, salon et compte, et seulement l'étiquette « Quitter » sur la table de jeu.
- `js/decor/` : scène 3D de fond de chaque page, voir son README.
- `js/polices.js` : charge les polices avant de peindre les cartes sur canvas.
