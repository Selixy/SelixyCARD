# session

Connexion, déconnexion, `GET /api/moi`, et l'extracteur `CompteCourant` que toute route protégée demande en argument (il renvoie
401 sans session valide).

- Jeton : 32 octets aléatoires (`getrandom`) en hexadécimal, dans le cookie `session` (HttpOnly, SameSite=Lax, Secure, 30 jours).
- La base ne garde que l'empreinte SHA-256 du jeton : une fuite de la table ne permet pas de se connecter. Pas de sel nécessaire,
  le jeton est déjà aléatoire.
- `Secure` : le cookie n'est accepté qu'en HTTPS, sauf sur `localhost` (Chrome et Firefox l'acceptent).
- Mauvais pseudo et mauvais mot de passe donnent le même message (voir `comptes/README.md` pour le temps de réponse).
