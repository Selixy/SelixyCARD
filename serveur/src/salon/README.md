# salon

WebSocket `/ws/partie` : salons en mémoire et parties UNO (`commun::uno`), protocole JSON du contrat.

- `connexion.rs` : une tâche par socket, `select!` entre messages reçus et file sortante (canal `mpsc`). Messages limités à 4 Ko.
- `global.rs` : le registre est un `static` (la route doit rester une seule ligne dans `main.rs`, sans toucher à `Partage`).
  `installer_source` branchera l'implémentation Postgres de `SourceSets`.
- `registre.rs` / `codes.rs` / `annonces.rs` / `diffusion.rs` : connexions, codes de 5 caractères sans I, O, 0, 1, envois ciblés.
  Les salons de partie lancée sans personne sont purgés après 10 minutes (le temps de passer de `/salon/` à `/partie/`).
- `lobby.rs` : créer, rejoindre, options, set, lancer (hôte seulement, 2 joueurs minimum). `routage.rs` : coups de jeu.
- `identite.rs` : seule fonction à remplacer pour passer au cookie de session (`pseudo_de`). Pour l'instant le pseudo vient de
  `creer` / `rejoindre`.
- `sets.rs` : `SourceSets::charger(id)` est synchrone ; l'implémentation Postgres devra précharger ou utiliser
  `block_in_place`. `SetParDefaut` renvoie les 108 emplacements sans illustration.
- `vues.rs` : messages sortants. Après chaque coup tout le salon reçoit `etat`, et chacun sa `main` (un échange 7/0 change plusieurs
  mains).

Champs ajoutés au contrat de `etat`, sans effet pour un client qui les ignore : `contestable` (un +4 attend une contestation) et
`signalable` (pseudo d'un joueur à une carte sans UNO). Sans eux l'interface ne saurait pas quand proposer « Contester » ou
« Signaler ».

Se reconnecter à une partie lancée se fait par `rejoindre` avec le même pseudo : le serveur renvoie `salon`, `etat` et `main`.

Les tests sont dans `essais/` (le serveur est un binaire, il n'a pas de dossier `tests/` importable).
