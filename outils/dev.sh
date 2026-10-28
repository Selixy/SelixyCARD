#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
BDD=selixy-card-dev-bdd
if ! podman container exists "$BDD"; then
  podman run -d --name "$BDD" -e POSTGRES_PASSWORD=dev -p 127.0.0.1:55434:5432 -v selixy-card-dev:/var/lib/postgresql/data docker.io/library/postgres:17-alpine >/dev/null
elif [ "$(podman inspect -f '{{.State.Running}}' "$BDD")" != true ]; then
  podman start "$BDD" >/dev/null
fi
until podman exec "$BDD" pg_isready -q; do sleep 1; done
bash outils/construire-wasm.sh dev
PORT=${PORT:-8080} DOSSIER_UI=ui DATABASE_URL=${DATABASE_URL:-postgres://postgres:dev@127.0.0.1:55434/postgres} cargo run -p serveur
