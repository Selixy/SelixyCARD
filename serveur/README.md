# serveur

axum + tokio + Postgres (sqlx). Sert `ui/` (`DOSSIER_UI`), l'API `/api/*`, les salons `/ws/partie` et `/sante`. Port `PORT`,
base `DATABASE_URL`. Détail par module dans `src/<module>/README.md`.

- `main.rs` : assemblage. Sans `DATABASE_URL`, seul `/ws/partie` est servi (invités, set par défaut) : pratique pour essayer les
  règles sans base.
- Avec la base, `/ws/partie` passe par `salon::accepter_connecte` : le cookie de session fixe le pseudo du joueur, que le client ne
  peut donc pas usurper ; sans session on retombe sur le pseudo d'invité envoyé dans `creer`/`rejoindre`.
- `salon::SetsPostgres` lit les sets depuis le registre des salons, qui est synchrone (mutex `std`) : la lecture passe par
  `block_in_place`, ce qui bloque le registre le temps de la requête. Acceptable au lancement d'une partie ; à revoir si des salons
  nombreux se créent en même temps.
- `sets::emplacements` dérive des 108 emplacements de `commun::uno` : une seule source de vérité.
- `lib.rs` expose les modules pour les tests d'intégration de `tests/` (Postgres de test sur `DATABASE_URL_TEST`, par défaut le
  port 55433).
