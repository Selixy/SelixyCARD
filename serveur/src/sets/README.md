# sets

CRUD des sets de 108 cartes (`/api/sets`, `/api/sets/{id}`). Lecture et écriture réservées à l'auteur connecté (401 sans
session, 403 pour un autre utilisateur, 404 si le set n'existe pas).

- `emplacements.rs` génère les 108 identifiants du contrat (`docs/contrats.md`). `commun::uno::paquet` fera la même chose côté
  règles ; à l'intégration, ce module appellera `commun` au lieu de dupliquer la liste.
- Une ligne par carte dans `cartes_set` (108 par set), `rang` fixe l'ordre. On pourrait stocker un `jsonb`, mais le nombre de
  cartes illustrées (`count(image)`) pour la liste devient alors une requête sur du JSON ; ici c'est un simple `count`.
- `PUT` ne prend de la carte que son `image` : type, couleur et valeur découlent de l'emplacement et ne se modifient pas. Il exige
  exactement les 108 identifiants, sans doublon, et que chaque image (et le dos) existe déjà (`POST /api/images`), sinon un set
  pourrait pointer vers une image absente.
- La mise à jour est une transaction : le nom, le dos et les 108 images changent ensemble ou pas du tout.
