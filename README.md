# SelixyCARD

Un jeu de cartes type UNO qui se joue dans le navigateur, sur une table en 3D. Le moteur de rendu est écrit en Rust, compilé en wasm avec wgpu. Les joueurs illustrent eux-mêmes les cartes de leurs sets, et un serveur autoritaire tient les règles.

Le projet est en cours, commencé en octobre 2026. Il est en ligne sur https://card.selixy.dev, et la fiche détaillée est ici : https://selixy.dev/selixy-card/.

## Comment ça marche, en gros

- `commun/` : les règles du UNO, écrites une seule fois (cumul des +2 et +4, contestation du +4, saut en cours de tour, règle du 7 et du 0, pénalité de UNO). Le mélange est reproductible à partir d'une graine, pratique pour rejouer un bug.
- `serveur/` : Axum et PostgreSQL. Comptes, sets, images et salons de jeu par WebSocket. Le serveur est le seul juge de la partie.
- `moteur/` : le moteur 3D, compilé en wasm. Il fait le rendu (WebGPU, avec repli WebGL2), l'animation des cartes et le pointage. L'état de la partie, la WebSocket et l'interface restent côté JavaScript.
- `graphe_pbr/` : un petit module qui décrit des matériaux sous forme de graphe de nœuds et les traduit en GLSL et en WGSL.
- `ui/` : les pages, le CSS et le JavaScript, sans npm ni étape de build. On y trouve l'éditeur de cartes avec recadrage 2:3, le salon et la table de jeu.
- `outils/` : construction du wasm, lancement en local, rendu des fonds, et un script d'audit qui vérifie les règles de style du dépôt (200 lignes par fichier, 5 fonctions par fichier, pas de commentaires dans le code).
- `docs/contrats.md` : les contrats entre les modules.

Chaque module a son propre README qui explique les choix.

## Pile technique

Rust (édition 2024), wgpu, wasm-bindgen, Axum, Tokio, sqlx, PostgreSQL, WGSL, JavaScript sans framework.

## Lancer en local

Il faut Rust avec la cible wasm, `wasm-bindgen-cli` dans la même version que celle de `Cargo.lock`, et Podman pour la base de test.

    rustup target add wasm32-unknown-unknown
    cargo install wasm-bindgen-cli --version 0.2.129 --locked
    bash outils/dev.sh

Le script démarre un PostgreSQL local dans un conteneur, construit le wasm puis lance le serveur sur http://localhost:8080. Pages : `/cartes/`, `/salon/`, `/partie/`, `/compte/`. Sans `DATABASE_URL`, le serveur ne sert que le salon en invité.

## Tests

    cargo check --workspace
    cargo test -p commun -p moteur -p graphe_pbr
    node --test 'tests/ui/*.test.js'
    bash outils/audit-modulaire.sh

Les tests du serveur ont besoin d'un PostgreSQL de test (variable `DATABASE_URL_TEST`, par défaut le port 55433 en local).

## Ce qui n'est pas là

Le déploiement sur mon serveur (configuration, scripts) n'est pas dans ce dépôt. Le jeu est encore en chantier : certaines parties peuvent bouger d'un jour à l'autre.

## Licence

MIT, voir `LICENSE`. Les polices (Lora, Caveat Brush) sont sous licence SIL OFL, dans `ui/polices/`.
