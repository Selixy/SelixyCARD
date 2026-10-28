# shaders

Shaders WGSL (`carte.wgsl`, `ombre_profondeur.wgsl`, `ombre_pcf.wgsl`, `decal.wgsl`) unifiés pour `wgpu`, inclus à la compilation (`include_str!`).

- Entrées par instance, 3 `vec4` : `pos_lacet` (x, y, z, rotation autour de Y), `couleur`, `forme` (largeur, hauteur, tangage, couche).
- Le quad est calculé dans le vertex shader : coin dans [-0.5, 0.5]², mis à l'échelle, incliné autour de X (tangage -90° = à plat,
  dessus vers -z), tourné autour de Y, translaté. L'UV vaut (x + 0.5, 0.5 - y) : le haut de la carte est la première ligne du bitmap.
- Sommet : transmet aussi la position monde et la normale de la carte, pour l'éclairage du matériau (`matiere`).
- Fragment : couche -1 = tapis (ardoise procédurale, `bruit` + `ardoise`) ; sinon les coins sont arrondis par `discard` (rayon 0,2 en unités de 2×3, soit 10 % de la largeur, à peine plus que l'arrondi de 9 % des textures ; un texel transparent restant prend le blanc du bord) et la
  couleur vient de la couche si elle est >= 0, de la teinte de l'instance sinon. L'échantillonnage est sans niveau de détail
  (`textureSampleLevel` / `textureLod`) : pas de mipmaps, et cela évite les contraintes d'uniformité du flot de contrôle en WGSL.
- Sélection (`couleur.a` ≥ 8, force = `a - 8`, au plus 1,5) : le fragment renvoie `rgb × poids` et `poids = force / 1,5` (couleur
  prémultipliée), les coins arrondis découpant toujours la forme ; c'est le masque de la lueur. `lueur.wgsl` : `fs_flou` (gaussienne
  de 49 prises, σ = 8 prises, direction en uniforme) puis `fs_composer` (rien là où le masque est plein ;
  `0,9 · clamp(2,4 · alpha)^1,6`, teinte = couleur prémultipliée / alpha).
- **PBR** (`eclairer`) : pipeline linéaire (textures converties de sRGB par `lineaire`, sortie par `ecran` = tone mapping ACES puis
  sRGB), BRDF Cook-Torrance (GGX, Smith, Fresnel de Schlick, F0 0,04), trois lampes (principale chaude ponctuelle, contre-jour
  froid, liseré arrière) et une ambiance hémisphérique ciel/sol. Position de l'œil en uniforme (`oeil` / `u_oeil`). Matériaux : face
  de carte rugosité ~0,5 modulée par le grain du carton plus un vernis (lobe 0,12), tranche 0,85, papiers 0,9, ardoise 0,78 à 0,97
  sous la craie avec micro-relief (`relief`), kraft 0,88 avec plis droits (`froissure`, 40 plis en crête ou en creux).
- Mode poudre (`couleur.a` entre 4 et 5) : texture de poudre de craie (couche >= 0) teintée de `couleur.rgb`, opacité
  `alpha × (a - 4)`, dessinée dans l'étape `Voile`.
- Thème de scène dans l'alpha de l'instance du tapis : 1 = ardoise, 2 = kraft froissé.
- Validés : le WGSL compile sans message et le pipeline WebGPU se crée (Chromium), le GLSL tourne en WebGL2 (SwiftShader).
