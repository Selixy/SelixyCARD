# ui/js/salon

Page `/salon/` : créer ou rejoindre un salon, régler les règles (hôte), choisir le set, lancer.

- `index.js` : seul fichier qui importe `../api.js` (fourni par l'agent A) ; il injecte `appeler`, `WebSocket`, `sessionStorage`.
  Tout le reste reçoit ses dépendances en paramètre, ce qui permet de le tester sans navigateur.
- `principal.js` : `demarrer(deps)` branche les boutons et la WebSocket. `liaison.js` met en file les messages envoyés avant l'ouverture.
- `etat.js` : réducteur pur des messages serveur. Le premier message `etat` signale le lancement : redirection vers `/partie/?code=XXX`.
- `regles.js` : les six règles du contrat avec libellé et explication. Les interrupteurs sont créés une fois puis mis à jour sur place
  (la case en cours de clic ne perd pas le focus). `sets.js` : `GET /api/sets`, avec « set par défaut » toujours proposé.
- Le pseudo est saisi sur la page et mémorisé dans `sessionStorage` (clé `pseudo`) pour que la page de partie puisse refaire `rejoindre`.
  À remplacer par la session quand le cookie sera branché côté serveur (`identite.rs`).
