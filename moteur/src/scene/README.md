
## Bloc de la pioche

La pioche est la carte extrudée jusqu'au sol : un anneau de rectangle arrondi (4 coins × 7 points, les arêtes verticales sont donc
les arrondis de la carte), des murs droits et un dessus plat sans biseau sur le bord haut. Au-dessus flotte une vraie carte (dos
texturé, tranche comprise) surélevée de `SURELEVATION` : le dos reste sur le chemin de rendu des cartes, éprouvé, et non sur un
éventail de triangles où il se déformait. Le maillage n'existe pas en mémoire : `shaders/bloc.wgsl` le calcule à partir de
`vertex_index` (252 sommets), partagé par le shader de scène et celui d'ombres (`concat!` à la création des pipelines). Une
instance suffit (`bloc.rs`) : `forme` = largeur, profondeur, hauteur, `COUCHE_BLOC`. Le plan (`rendu/plan.rs`) sépare les blocs des
quads pour leur donner leur propre appel de dessin, et la passe d'ombres fait de même. Le spawn des cartes piochées part toujours
de `pose_pioche`, le dessus de la carte flottante.

## Sélections

`Selection` (`selection.rs`) est la classe commune de tout ce qui reçoit une lueur : la main, la carte survolée, le premier plan, la
défausse, la main d'un adversaire, la pioche. Elle porte une couleur, une force (`couleur.a`, bornée à `FORCE_MAX`) et les
formes marquées (cartes et bloc de la pioche) ; `table/selections.rs` y range chaque cible selon son groupe (`table/lueurs.rs`).
Elle ne dessine aucun contour : le rendu en tire la lueur depuis l'écran (voir `rendu/README.md`), quelle que soit la forme.
