# bdd

Connexion à PostgreSQL et migrations (`serveur/migrations/`, appliquées au démarrage par `sqlx::migrate!`).

- La base démarre souvent en même temps que le serveur (pod Podman) : `connecter` réessaie 15 fois, 2 s d'écart, puis abandonne.
- Les requêtes sont écrites avec `sqlx::query` (vérifiées à l'exécution), pas avec les macros vérifiées à la compilation, pour que
  `cargo build` ne demande pas de base.
- Schéma : `comptes` (pseudo unique sans tenir compte de la casse), `sessions` (empreinte SHA-256 du jeton, jamais le jeton),
  `sets` et `cartes_set` (108 lignes par set, `rang` garde l'ordre des emplacements), `images` (octets en `bytea`, clé = sha256).
