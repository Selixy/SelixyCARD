# outils

- `construire-wasm.sh [profil]` : compile `moteur` en wasm32 et génère la glue JS dans `ui/wasm/` (`release` par défaut).
- `dev.sh` : construction wasm en `dev` puis serveur local.
- `audit-modulaire.sh` : outil d'audit étendu au Rust (`audit/lexeur_rust.py` : distingue lifetimes et
  caractères littéraux ; `fn` compte comme fonction). Écarts : lignes > 200, fonctions > 5, fonction > 50 lignes, commentaire.
  Les tests d'intégration Rust vivent dans `<crate>/tests/` pour ne pas peser sur le compte de fonctions des modules.
