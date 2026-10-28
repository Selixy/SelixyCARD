# commun::uno

Moteur de règles UNO, sans dépendance (serde seulement derrière la feature `serde`, activée par `serveur`, jamais par le moteur wasm).
Toute la logique est dans `commun` pour rester testable sans réseau et identique si un jour le client doit prédire un coup.

## Organisation

- `cartes.rs`, `options.rs`, `erreur.rs` : types du contrat (`Carte`, `Couleur`, `TypeCarte`, `Options`) et erreurs en français.
- `paquet.rs` : les 108 emplacements aux identifiants du contrat. `paquet(set)` renvoie toujours 108 cartes : celles du set quand
  l'identifiant existe, l'emplacement par défaut (sans image) sinon.
- `melange.rs` : xorshift64* maison (`Alea`). Même graine, même partie : indispensable pour rejouer un bogue.
- `etat.rs` : `Partie`, tous les champs sont publics (le serveur en fait une vue, les tests fabriquent des situations précises).
- `demarrage.rs` : 7 cartes par joueur, première carte retournée (un +4 est remis au fond, un joker fixe une couleur tirée au sort,
  passer / inverser / +2 s'appliquent au premier joueur).
- `jouable.rs`, `tour.rs`, `pioche.rs` : carte jouable, voisin selon le sens, pioche avec recyclage de la défausse.
- `coups/` : `jouer`, `piocher`, `dire_uno` / `signaler`, `contester`. `effets/` : ce que produit une carte posée.

## Choix de règles

- Inverser à 2 joueurs fait rejouer le même joueur (équivaut à passer).
- Un +4 n'est refusé quand on a la couleur que si `contester_plus4` est désactivée : sinon le bluff est permis, c'est précisément ce
  que la contestation sanctionne. La couleur jouée par le +4 reste active après une contestation.
- Une pénalité (+2 / +4) est immédiate, sauf avec `cumul_plus` (ou +4 contestable) : elle est alors stockée dans `penalite` et le
  joueur suivant doit répondre par une carte du même type, contester ou piocher.
- Contestation réussie : le poseur pioche 4 et le contestataire joue normalement. Ratée : le contestataire pioche la pénalité + 2
  et passe son tour.
- Sans `piocher_jusqua_jouable`, on pioche une carte ; si elle est jouable on peut la poser (seulement elle), sinon le tour passe.
  Piocher à nouveau après cela passe le tour.
- `jump_in` : carte strictement identique à la défausse (couleur, type, valeur), jamais un joker, jamais pendant une pénalité ; le
  tour repart du joueur qui a sauté.
- `sept_zero` : un 7 exige une `cible`. Le gagnant est déclaré dès que sa main est vide, avant tout effet de la dernière carte.
- UNO : `penalite_uno` marque le joueur à une carte sans UNO comme signalable jusqu'au coup suivant de n'importe qui. `dire_uno` est
  accepté avec 1 ou 2 cartes (annonce avant de poser). Toute pioche, échange ou rotation de mains remet le drapeau à zéro.
