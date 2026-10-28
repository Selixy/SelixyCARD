#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
export PATH="$HOME/.cargo/bin:$PATH"
PROFIL=${1:-release}
cargo build -p moteur --target wasm32-unknown-unknown --profile "$PROFIL"
DOSSIER=$([ "$PROFIL" = dev ] && echo debug || echo "$PROFIL")
wasm-bindgen --target web --no-typescript --out-dir ui/wasm "target/wasm32-unknown-unknown/$DOSSIER/moteur.wasm"
sed -i "s/'moteur_bg.wasm'/'moteur_bg.wasm?v=glow3'/g" ui/wasm/moteur.js
echo ">> wasm prêt dans ui/wasm/ ($(du -h ui/wasm/moteur_bg.wasm | cut -f1))"
