#!/usr/bin/env bash
# Construit le serveur : lancé sur Heroku par RustConfig une fois Rust installé, ou à la
# main : ./heroku-build.sh. Le Procfile démarre target/release/docs.
set -euo pipefail

# Si des dépôts de Paquets sont privés, Cargo les récupère avec git, qui demande ses
# identifiants à un assistant lisant la variable REPOS_TOKEN (un jeton GitHub qui peut
# les lire). Le jeton n'apparaît dans aucune adresse.
if [ -z "${REPOS_TOKEN:-}" ] && [ -n "${ENV_DIR:-}" ] && [ -f "$ENV_DIR/REPOS_TOKEN" ]; then
  REPOS_TOKEN=$(<"$ENV_DIR/REPOS_TOKEN")
fi
if [ -n "${REPOS_TOKEN:-}" ]; then
  export REPOS_TOKEN CARGO_NET_GIT_FETCH_WITH_CLI=true GIT_CONFIG_COUNT=1 \
    GIT_CONFIG_KEY_0=credential.https://github.com.helper \
    GIT_CONFIG_VALUE_0='!f() { test "$1" = get && echo username=x-access-token && echo "password=$REPOS_TOKEN"; }; f'
fi
cd "$(dirname "$0")"
target="${CARGO_TARGET_DIR:-target}"
echo "-----> Construction du serveur"
cargo build --release --locked
mkdir -p target/release
if [ "$target" != target ]; then
  cp "$target/release/docs" target/release/
fi
echo "       target/release/docs prêt"
