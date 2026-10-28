# rendu

Stratégie : `trait Rendu` (`profondeur_zero_un`, `redimensionner`, `dessiner`, `charger_texture`). Le reste du moteur ignore le backend.

- Backend unique `wgpu` (`wgpu/`) avec repli automatique sur WebGL2 via la fonctionnalité `webgl` de `wgpu`.
- `plan.rs` range la scène, les voiles, puis les formes de chaque `Selection` dans un seul tampon et produit des `Trace` (étape, début, nombre, sommets).
  **Lueur d'une sélection, dans l'espace écran** (`wgpu/lueur/`) : une passe `passe_masque` redessine les formes des sélections
  (mêmes shaders, `couleur.a` ≥ 8) dans une texture RGBA8 plein écran, profondeur de la scène en lecture seule, couleur prémultipliée
  par la force et mélange `Max`. Un flou gaussien séparable (49 prises, demi-résolution, `lueur.wgsl`) en tire un dégradé depuis le
  contour de l'union des formes, puis la composition l'ajoute à l'écran partout où le masque est vide. Aucun quad de lueur, aucune
  accumulation, et toute forme (cartes, bloc de la pioche) donne la même lueur.
- Ordre : passe de profondeur d'ombres dynamiques (`passe_ombres`), puis scène, scène et voiles (`passe_principale`), puis masque, flou et composition de la lueur. Les voiles
  passent après toutes les cartes opaques, sinon une carte à lueur (le joker posé) recouvrirait la poudre déposée sur elle.
- **Voiles** (couche -3 : ombres portées, halos, poussière des impacts ; mode poudre) : étape `Voile`, après les cartes, profondeur testée mais
  pas écrite. Ils se mélangent entre eux et avec le décor au lieu de se découper comme des disques opaques.
- Chaque trace est **un appel instancié** : 6 sommets par carte, `MAX_INSTANCES` = 2048 instances.
  Il n'y a **aucun tampon de sommets** : le coin du quad vient de `@builtin(vertex_index)`, le reste de l'instance.
- **Textures : un tableau de textures** (`texture_2d_array` en WGSL), 128 couches de 256×384 RGBA8
  (environ 50 Mo), plutôt qu'un atlas : pas de calcul d'UV par case, pas de bavure entre cartes, et l'index de couche (le `w` de la
  forme de l'instance) est un flottant plat.
- Pas de mipmaps : filtrage linéaire simple. Les coins arrondis sont découpés par `discard` dans le fragment shader.
- `wgpu/` : `contexte.rs` (initialisation asynchrone, redimensionnement), `tampons.rs` (uniformes et instances), `textures.rs` (tableau, échantillonneur, profondeur, ombre), `pipeline.rs` (pipelines et bind groups), `passe.rs` (passes d'ombres et principale).
- `donnees.rs` : `flottants_instances` et `octets` réinterprètent des tranches sans copie : sûr car `Instance` est `#[repr(C)]` et
  ne contient que des `f32`.
