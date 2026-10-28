# Contrats entre modules

Figés pour la phase 2 : on ne les change qu'en le décidant ensemble. Chaque module code contre ces contrats, et bouchonne l'autre
côté tant qu'il n'est pas fusionné.

## Pages et navigation

Pages statiques, une par dossier : `ui/index.html` (accueil), `ui/compte/`, `ui/cartes/` (bibliothèque et éditeur), `ui/salon/`,
`ui/partie/`. Chaque page charge `css/atelier/jetons.css`, `papier.css`, `formulaires.css`, `entete.css`, puis ses propres CSS, pose un
`<canvas id="decor">` et `data-decor` sur `<body>` (scène 3D de fond, voir `ui/js/decor/README.md`), et reprend l'en-tête :

```html
<header class="header">
  <a class="logo" href="/">SelixyCARD</a>
  <nav class="header-tabs" aria-label="Pages">
    <a class="tab" href="/cartes/">Cartes</a>
    <a class="tab" href="/salon/">Jouer</a>
  </nav>
  <nav class="header-account" aria-label="Compte"><a class="tab" href="/compte/">Compte</a></nav>
</header>
```

Le lien de la page courante porte `aria-current="page"`. Le JS de chaque page vit dans `ui/js/<page>/`, en modules ES natifs, sans
dépendance. Les appels HTTP passent par `ui/js/api.js` (fourni par l'agent A ; en attendant, chacun peut le bouchonner localement).
`api.js` exporte `appeler(methode, chemin, corps?)`, qui renvoie le JSON ou lève une `Error` dont le message est en français.

## Carte et set

Un set compte 108 emplacements, avec des identifiants fixes :

- pour chaque couleur `rouge|jaune|vert|bleu` : `<c>-0`, `<c>-<1..9>-<a|b>`, `<c>-passer-<a|b>`, `<c>-inverser-<a|b>`,
  `<c>-plus2-<a|b>`, soit 25 cartes par couleur ;
- `joker-<1..4>` et `joker_plus4-<1..4>`.

```json
{
  "id": "uuid", "nom": "Mon set", "auteur": "pseudo", "dos": "sha256|null",
  "cartes": [ { "id": "rouge-7-a", "type": "nombre", "couleur": "rouge", "valeur": 7, "image": "sha256|null" } ]
}
```

- `type` vaut `nombre|passer|inverser|plus2|joker|joker_plus4`. `couleur` vaut `null` pour les jokers, `valeur` vaut `null` sauf
  pour le type `nombre`.
- `image: null` affiche le visuel par défaut (fond de couleur, valeur au centre). L'aperçu et le rendu 3D dessinent par-dessus
  l'illustration le **cadre commun** (bord de couleur, valeur dans les coins) : toutes les cartes restent lisibles.
- La liste des 108 emplacements se calcule avec `ui/js/cartes/emplacements.js` côté UI (agent B) et `commun::uno::paquet` côté Rust
  (agent C). Les deux sont testés pour donner exactement les mêmes identifiants.

## Images

- Le client recadre (ratio 2:3) puis exporte un WebP 512×768 avec `canvas.toBlob(…, 'image/webp', 0.9)`.
- `POST /api/images` : corps brut `image/webp`, 2 Mo au maximum, session obligatoire. Réponse : `{ "image": "<sha256>" }`. Le serveur
  ne décode pas l'image : il vérifie seulement la taille et l'en-tête `RIFF....WEBP`.
- `GET /api/images/<sha256>.webp` : public, avec `Cache-Control: public, max-age=31536000, immutable`.

## API HTTP (JSON, session par cookie `session` HttpOnly SameSite=Lax)

| Méthode | Chemin | Corps → réponse |
|---|---|---|
| POST | `/api/comptes` | `{pseudo, mot_de_passe}` → `{pseudo}` et ouvre la session |
| POST | `/api/session` | `{pseudo, mot_de_passe}` → `{pseudo}` |
| DELETE | `/api/session` | → 204 |
| GET | `/api/moi` | → `{pseudo}` ou 401 |
| GET | `/api/sets` | → `[{id, nom, auteur, nb_illustrees}]` (les miens et ceux des autres) |
| POST | `/api/sets` | `{nom}` → set complet, les 108 cartes avec `image: null` |
| GET | `/api/sets/<id>` | → set complet |
| PUT | `/api/sets/<id>` | set complet → set complet (seulement par son auteur) |
| DELETE | `/api/sets/<id>` | → 204 (seulement par son auteur) |

Erreurs : un code HTTP plus `{ "erreur": "message en français" }`.

## Règles optionnelles (`commun::uno::Options`)

Toutes sont des booléens. Valeur par défaut entre parenthèses.

| Champ | Effet |
|---|---|
| `cumul_plus` (non) | un +2 répond à un +2, un +4 répond à un +4, et la pénalité s'additionne |
| `sept_zero` (non) | un 7 échange sa main avec celle d'un joueur choisi, un 0 fait tourner toutes les mains |
| `jump_in` (non) | une carte identique (couleur et valeur) peut être posée hors de son tour |
| `piocher_jusqua_jouable` (non) | on pioche jusqu'à obtenir une carte jouable |
| `penalite_uno` (oui) | 2 cartes si on oublie de dire UNO et qu'un autre joueur le signale avant le coup suivant |
| `contester_plus4` (oui) | un +4 peut être contesté ; si le joueur avait la couleur demandée, c'est lui qui pioche 4 |

## Décor 3D (moteur wasm)

- `Moteur.definir_decor(json)` : `{vise:[x,z], distance, elevation, elements:[{cle, x, y, z, lacet, l, h, flotte, phase, feuille}]}`. `feuille:1` dessine
  la texture sans coins arrondis (papier, crayon) ; sans, une carte. Chaque élément projette une ombre.
- `Moteur.pointeur_decor(x, y)` : position du pointeur normalisée dans [-1, 1], pour le léger effet de parallaxe de la caméra.
- Tant que `definir_table` n'a pas été appelé, la caméra est celle du décor ; ensuite c'est celle de la table, le décor restant dessiné autour.

## Protocole de partie : WebSocket `/ws/partie`, messages JSON `{ "t": "...", ...}`

Avec une session ouverte, le pseudo est celui du compte ; le champ `pseudo` de `creer`/`rejoindre` ne sert qu'aux invités.

`commun` expose les types avec `#[derive(Serialize, Deserialize)]` derrière la feature `serde` (activée par `serveur` seulement ;
le moteur wasm n'en dépend pas).

Du client vers le serveur :
- `creer {set}` → le serveur répond `salon`
- `rejoindre {code}`
- `options {options}` : seulement l'hôte, avant le lancement
- `choisir_set {set}` : seulement l'hôte
- `lancer` : seulement l'hôte, il faut au moins 2 joueurs
- `jouer {carte, couleur?, cible?}` : `couleur` pour un joker, `cible` (pseudo) pour un 7
- `piocher`
- `uno`
- `signaler {pseudo}`
- `contester`

Du serveur vers le client :
- `salon {code, hote, joueurs:[pseudo], options, set}`
- `etat {defausse: carte, couleur_courante, sens: 1|-1, courant: pseudo, joueurs:[{pseudo, nb_cartes, uno}], pioche: n, penalite: n, contestable: bool, signalable: pseudo|null}`
- `main {cartes:[carte], jouables:[id], piochee: id|null}` : envoyé seulement au joueur concerné. `jouables` = cartes que le serveur
  accepterait maintenant (seules celles-ci sont cliquables) ; `piochee` = carte qu'il vient de piocher et peut encore jouer
  (`piocher` de nouveau = la garder et passer)
- `erreur {message}`
- `fin {gagnant}`

`carte` désigne l'objet carte complet du set de la partie (avec `image`) : tous les joueurs voient l'illustration de chaque carte
posée.

## Interface du moteur (wasm) pour la table de jeu

Le JS de `ui/js/partie/` gère la WebSocket et l'état. Le moteur ne fait que le rendu et le pointage :
- `Moteur.charger_texture(cle: &str, image: ImageBitmap)` (256×384) : `cle` = sha256, ou `defaut:<id-carte>`, ou `dos` ;
- `Moteur.definir_table(etat_json: &str)` : `{main:[cle], defausse: cle, adversaires:[nb_cartes], pioche: n, jouables:[indice],
  premier_plan: indice|-1, courant: -1|0 (moi)|1..n (adversaire)}` (parsé à la main ou
  avec `js_sys::JSON`, sans serde dans le wasm) ;
- `Moteur.carte_sous_pointeur(x, y) -> i32` (pixels du canvas, même unité que `redimensionner`) : index dans la main, ou `-1`, ou `-2` pour la pioche.
