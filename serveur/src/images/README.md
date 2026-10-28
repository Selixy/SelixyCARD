# images

`POST /api/images` (session obligatoire) et `GET /api/images/<sha256>.webp` (public).

- Stockage : octets en `bytea` dans Postgres, clé primaire = sha256. Choix fait pour qu'une seule sauvegarde (`pg_dump`) couvre
  comptes, sets et images, et que le conteneur serveur reste en lecture seule sans volume de données. Une image fait au plus 2 Mo
  (WebP 512x768, souvent quelques dizaines de Ko) : la base reste petite. Si le volume explose, ce module est le seul à changer
  (`depot.rs`) pour écrire sur disque.
- Le sha256 sert d'identifiant : la même image envoyée deux fois n'est stockée qu'une fois (`on conflict do nothing`), et l'URL
  ne change jamais de contenu, d'où `Cache-Control: immutable`.
- Le serveur ne décode pas l'image : il contrôle la taille (413 au-delà de 2 Mo) et l'en-tête `RIFF....WEBP` (400 sinon).
  La limite de corps de la route vaut 2 Mo + 1 octet pour distinguer « trop gros » de « tout juste accepté ».
- Les images ne sont jamais supprimées avec un set : un autre set peut partager la même image.
