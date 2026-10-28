# api

Assemblage des routes `/api/*` et types communs.

- `etat.rs` : l'état partagé (le pool Postgres). Rien d'autre : le monde de la démo WebSocket garde son propre état.
- `erreur.rs` : toutes les erreurs de l'API en un seul type, rendu en `{ "erreur": "message français" }` avec le bon code HTTP.
  `From<sqlx::Error>` permet d'écrire `?` partout ; le détail technique reste dans le journal, jamais dans la réponse.
- `routeur.rs` : fusionne les routes des modules `comptes`, `session`, `sets`, `images` et fournit l'état.
