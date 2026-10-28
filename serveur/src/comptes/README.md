# comptes

Inscription (`POST /api/comptes`) : pseudo + mot de passe, la session s'ouvre aussitôt.

- Pas d'e-mail : le contrat n'identifie un compte que par son pseudo. Pas de récupération de mot de passe pour l'instant.
- Pseudo : 3 à 24 caractères (lettres, chiffres, `_`, `-`, `.`), unique sans tenir compte de la casse (index sur `lower(pseudo)`),
  l'inscription simultanée de deux pseudos identiques est tranchée par la base (`on conflict do nothing`).
- Mot de passe : 8 caractères au moins, 1024 octets au plus (Argon2 coûte cher), haché en Argon2id dans `spawn_blocking`.
- `mot_de_passe::verifier` fait le même travail de hachage quand le pseudo n'existe pas, pour que le temps de réponse ne révèle pas
  quels pseudos existent.
