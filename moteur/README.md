# moteur

Moteur 3D compilé en wasm (`cdylib`) pour la table de jeu. Le JS gère la WebSocket, l'état de la partie et l'interface ; le moteur
ne fait que le rendu, l'animation et le pointage.

## API exportée (contrat de `docs/contrats.md`)

- `Moteur.webgl2(canvas)` / `Moteur.webgpu(canvas, device, format)`, `redimensionner(largeur, hauteur)` (pixels du canvas), `tick(dt)`.
- `charger_texture(cle, ImageBitmap)` : `cle` est opaque pour le moteur. Le bitmap fait **256×384** (ratio 2:3) ; un bitmap plus
  petit ne remplit que le coin de la couche. L'interface choisit `defaut:<id>` pour une carte sans image, `<sha256>:<id>` pour une
  carte illustrée (le cadre dépend de la carte : deux cartes peuvent partager une illustration sans partager de texture) et `dos`.
- `definir_table(json)` : `{main:[cle], defausse: cle|null, adversaires:[nb_cartes], pioche: n}`. Lève une exception si le JSON est
  invalide. Le JSON est lu par un analyseur écrit à la main (`json/`) : pas de serde dans le wasm, et il se teste hors navigateur.
  Limite connue : les échappements `\uXXXX` ne sont pas décodés (les clés sont en ASCII).
- `carte_sous_pointeur(x, y) -> i32` : `x`, `y` en pixels du canvas (mêmes unités que `redimensionner`). Renvoie l'index dans la
  main, `-1` (rien) ou `-2` (pioche). Effet de bord voulu : mémorise la carte survolée, qui se soulève au `tick` suivant ; l'interface
  l'appelle donc à chaque mouvement du pointeur.

## Organisation

- `decor/` (`apparition.rs`, `etat.rs`) + `api_decor.rs` : décor animé hors partie (`definir_decor`, `pointeur_decor`, `animer_apparition_decor`). Éléments posés sur le plan de travail, animation d'apparition en cascade des cartes à l'ouverture puis flottement oscillant, ombres portées, parallaxe de la caméra.
- `lib.rs` : façade `Moteur` ; `boucle.rs` : `tick` ; `api_table.rs` : les trois fonctions de table.
- `scene/` : `Instance` (12 flottants : position+lacet, couleur, forme = largeur, hauteur, tangage, couche), caméra, géométrie des
  cartes (`Pose`, coins, projection écran).
- `table/` : `etat.rs` (JSON → `EtatTable`), `disposition.rs` (poses au repos : éventail, piles), `compose.rs` (liste des cartes à
  dessiner, survol compris), `pointage.rs` (carte sous le pointeur), `mod.rs` (poses animées).
- `textures.rs` : clé → couche du tableau de textures.
- `json/` : analyseur JSON minimal (objets, listes, chaînes, nombres).
- `maths/` : matrices 4x4 en colonnes, comme GLSL et WGSL. `perspective` prend `zero_un` : WebGPU attend une profondeur NDC dans
  [0, 1], WebGL dans [-1, 1].
- `rendu/` et `shaders/` : voir leurs README.

## Choix de conception

- **Tout est posé à plat** (tangage de -90°), la caméra de table est basse (environ 37°, plus proche et visant le centre de la table) pour un point de vue immersif. Des cartes parallèles se superposent par simple
  décalage en hauteur : un éventail de cartes inclinées se traverserait au z-buffer. Le survol soulève la carte en hauteur.
- **Animation** : chaque carte est identifiée (`main[i]`, rang dans la pioche, défausse numérotée, adversaire/carte). Sa pose
  courante rejoint la pose cible par lissage exponentiel (`1 - exp(-12 dt)`), indépendant du débit d'images. Une carte nouvelle part
  de la pioche ; une carte qui disparaît s'efface (une nouvelle défausse arrive donc depuis la pioche).
- **Pointage sans lancer de rayon** : on projette les quatre coins de chaque carte au repos (sans survol, pour que le pointage ne
  vacille pas) et on teste l'appartenance au quadrilatère convexe, de la carte du dessus à celle du dessous.
- **Caméra** : la distance augmente quand l'écran est étroit (`13.7 / aspect`) pour garder la main entière sur téléphone.

Piège : les liaisons WebGPU de `web-sys` exigent `--cfg=web_sys_unstable_apis`, posé dans `.cargo/config.toml` pour toutes les
cibles (sinon `cargo test` natif ne compile pas).

## Plan de travail et ombres

- `definir_theme("ardoise" | "kraft")` choisit le fond (`chargeur.js` le lit dans `data-scene` du `<body>`).
- Le tapis est une ardoise générée dans le shader (`ardoise` : matière mate faite de grain fin sur trois échelles, paillettes et
  taches minérales, marbrure et voile de craie très discrets pour éviter l'aspect brillant, vignettage) et des coups de craie (`coups` : une cellule de 3,5 unités sur deux porte
  un trait, voisines comprises ; paquet de 1 à 3 hachures légèrement courbes, traînée dans le sens du geste, pression variable,
  ou trace d'effaçage large et pâle) : aucune texture, donc pas d'étirement à l'écran. Ombres presque noires.
- Les deux backends activent le mélange alpha (couleur seulement ; l'alpha du canvas reste à 1) pour les ombres (couche -3, bord adouci dans le shader).
- Les instances sont triées par hauteur avant le dessin : une ombre se place juste sous sa carte et n'en cache jamais une plus basse.
- Mode `couleur.a = 2` : « feuille », sans coins arrondis, avec rejet des texels transparents (crayon, ruban) ; le rgb est divisé par l'alpha pour éviter le liseré sombre du filtrage.
- Une carte qui flotte garde au moins 0,1 de garde en hauteur au-dessus de son ombre, sinon son bord incliné passerait sous elle.

Caméra de table en portrait (`scene/camera.rs`) : plus plongeante et un peu plus proche quand l'écran est plus haut que large, pour
que la main occupe le bas de l'écran au lieu d'y laisser un vide. Le pointage utilise la même caméra : rien à recaler.
Animations (`mouvement.rs`, `trajet.rs`, `origines.rs`, `souffle.rs`) : un déplacement de plus de 1,2 unité suit un trajet en
arc avec un léger tour sur soi ; pour la carte jouée, montée qui ralentit jusqu'au sommet puis chute cubique qui accélère jusqu'à
l'impact (`courbes`), les petits ajustements restent un lissage exponentiel. Une
carte de la main est identifiée par sa clé (`compose::identifiant`), pas par son rang : à chaque `definir`, `origines` fait partir
la carte jouée de sa place dans la main (ou de l'éventail de l'adversaire qui jouait) et les cartes piochées du sommet de la pioche,
avec une bouffée de craie. Les trajets vers la main sont rapides (0,2 à 0,38 s), celui vers la défausse plus haut
(0,26 à 0,5 s), et son atterrissage soulève un nuage de poussière entièrement de la couleur de la carte (couleur choisie pour un joker,
`lueurs::teinte_carte`) : flash bref au sol et 40 grains tirés d'une graine propre à chaque impact (angle, portée lancée vite puis
fortement freinée, montée basse en volutes tournées vers la caméra ou reste au sol, taille, durée de vie, nuance), portée de 0,8 à 2,7 unités (1 s).
Un joker posé souffle un nuage plus dense et plus large (force 1,7) et laisse un dépôt de poussière de la couleur choisie
(`depot.rs`, 66 taches de poudre tirées de la graine de l'impact) : 30 toujours sur le joker lui-même, sur la carte du dessous
autour, sur l'ardoise au-delà ; les cartes suivantes recouvrent ce qui est dessous. Les dépôts restent (800 taches au plus).
La défausse garde les `PILE_DEFAUSSE` (12) dernières cartes en pile : angle et décalage pseudo-aléatoires tirés de leur numéro,
hauteur selon leur rang, la plus récente au-dessus.

L'éventail de la main s'ouvre selon la place visible : `disposition::ouverture` projette la profondeur de la main avec la
caméra de table, en déduit la demi-largeur visible (marge de 6 %) et limite l'angle total (au plus `OUVERTURE_MAX`). Les cartes se
resserrent donc d'elles-mêmes avec leur nombre et en portrait ; le pointage refait le même calcul, rien à recaler.
En portrait, la caméra se rapproche et les éventails adverses se resserrent vers le centre du même facteur que la main
(`eventail::serrage`, tiré de l'ouverture visible) pour rester dans le cadre.
Hors de ton tour, la main rétrécit (×0,75, éventail resserré) et descend, par un facteur `retrait` lissé dans `Table` ;
elle ne réagit alors plus au pointeur. Les cartes jouables montent sans avancer vers le centre, pour ne pas cacher la pioche et la défausse depuis cette vue basse.

Matériau des cartes (`matiere` dans les shaders) : carton mat éclairé par une lumière ponctuelle (dégradé sur chaque carte), reflet
satiné discret, grain de carton, léger biseau clair au bord. Chaque carte a une tranche (`scene::tranche`, un quad blanc cassé
décalé derrière elle) qui donne l'épaisseur ; le bord des faces et des dos est blanc (`BORD` dans `ui/js/cartes/dessin/palette.js`).

Table de jeu (`table/`) :
- `eventail.rs` : mains adverses en éventail de dos, inclinées vers le joueur ; pose « premier plan » de la carte piochée, face
  caméra, au-dessus de la table.
- `compose.rs` : à mon tour, les cartes jouables sont légèrement soulevées ; la carte au premier plan sort de l'éventail (le
  pointage l'ignore, on la joue par les boutons de l'interface).
- `lueurs.rs` : lueur unique, fine et fixe (blanche pour toutes les mains et la carte piochée ; sur la pioche à ton tour tant que tu n'as pas pioché, discrète si tu
  as une carte jouable, renforcée si tu n'as pas le choix, la lueur de ta main
  devenant alors très faible ; le survol et le glissement sont réservés aux cartes jouables, qui sortent de la main dans leur
  propre plan (`Pose::glisser`) sans passer devant leurs voisines ; de la couleur demandée pour la défausse) autour d'un groupe de cartes (ta main à ton tour, la carte survolée à part comme détachée, la carte piochée, la main de l'adversaire dont c'est
  le tour, la défausse dans la couleur demandée). Chaque groupe devient une `scene::Selection` ; le
  rendu en tire la lueur de l'union des formes dans l'espace écran (voir `src/rendu/README.md`), donc aucun trait entre les cartes.
- `MAX_INSTANCES` passe à 2048 : décor + éventails adverses + ombres dépassaient 512, et les dernières instances (triées par
  hauteur, donc la carte au premier plan) étaient silencieusement coupées.
