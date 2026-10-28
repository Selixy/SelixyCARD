# ui/js/decor

Scène 3D vivante derrière chaque page : une ardoise vue en perspective, avec ses objets de tableau noir (bâtons de craie entiers ou cassés, poussière, brosse, bout de cadre en bois), des dessins et écritures à la craie posés à plat (étoile, flèche, cœur, gribouillis, quadrillage, « UNO », bâtons de score) et des cartes qui flottent. Le même moteur wasm que la table de jeu.

- `page.js` : chargé par chaque page (`<canvas id="decor">`, `data-decor` sur `<body>`). Crée le moteur, peuple la scène, suit le pointeur pour la parallaxe. Sans WebGL ni wasm, la page garde son fond d'ardoise (`decor-absent`).
- `scenes/` : une scène par page, en données pures. Un élément est `[nom, x, z, lacet, hauteur, flotte, échelle]`. `etroit` remplace des champs sous 700 px de large. Les cartes s'écrivent `carte:<variante>:<id>` : `defaut` donne la carte sans image, un nombre ajoute une illustration à l'aquarelle générée.
- `papiers/` : peintures canvas des objets (`craie.js`, `materiaux.js`, `traces.js`, `ecritures.js`, `illustration.js`), dessinées en unités du monde (le canvas 256×384 est mis à l'échelle de la taille de l'objet).
- `objets.js`, `textures.js` : registre des objets et envoi des textures au moteur, polices chargées avant.
- `index.js` : `peuplerDecor(moteur, nom, etroit)`, aussi utilisé par la page partie, où le décor entoure la table.

Dessins à la craie colorés (variantes `trace_*_jaune`, `_rose`, `_bleue`, `_verte`, `trace_plus4`, `trace_bravo`) sur les
marges en large ; en `etroit` (mobile), dans les bandes du haut et du bas, à l'échelle 1,7 pour rester lisibles sous la colonne
de l'interface.

Les hauteurs des cartes d'un même éventail sont espacées de 0,4 : leur flottement et leur ombre ne doivent pas se croiser.
