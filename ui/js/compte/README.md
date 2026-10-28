# ui/js

- `api.js` : seul point d'accès HTTP des pages. `appeler(methode, chemin, corps?)` renvoie le JSON (ou `null` pour un 204) et lève
  une `Error` au message français. Un `Blob` part brut avec son propre `Content-Type` (dépôt d'image WebP), tout autre corps part
  en JSON. Le cookie de session passe tout seul (`same-origin`).
- `compte/` : page `ui/compte/`. `vue.js` touche au DOM, `principal.js` pilote inscription, connexion et déconnexion ; la page
  interroge `/api/moi` au chargement pour savoir si une session existe.

- `badge.js` : étiquette de compte de l'accueil. Elle lit `GET /api/moi` : sans session, « Connexion » ; avec session, le pseudo, et
  une médaille d'avatar pend à un cordon accroché à la punaise de l'étiquette (`css/atelier/badge-compte.css`, léger balancement,
  coupé si `prefers-reduced-motion`). L'avatar est généré à partir du pseudo (`avatar` : initiale, teinte et motif stables) :
  le serveur ne stocke pas encore d'avatar.
