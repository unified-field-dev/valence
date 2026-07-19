#!/usr/bin/env bash
# Install Rust + Docker + compose wire services on a campaign host.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ENV_FILE="${INSTANCES_ENV:-$ROOT/instances.env}"
# shellcheck disable=SC1091
source "$ENV_FILE"

ROLE="${1:?usage: bootstrap.sh e2e|bench}"
case "$ROLE" in
  e2e)
    HOST="${E2E_PUBLIC_IP}"
    ;;
  bench)
    HOST="${BENCH_PUBLIC_IP}"
    ;;
  *)
    echo "role must be e2e or bench" >&2
    exit 1
    ;;
esac

SSH_KEY="${SSH_KEY_PATH:-$HOME/.ssh/id_ed25519}"
SSH_OPTS=(-o StrictHostKeyChecking=accept-new -o ConnectTimeout=20 -i "$SSH_KEY")

echo "Waiting for SSH on ${SSH_USER}@${HOST}..."
for _ in $(seq 1 60); do
  if ssh "${SSH_OPTS[@]}" "${SSH_USER}@${HOST}" "echo ok" >/dev/null 2>&1; then
    break
  fi
  sleep 5
done
ssh "${SSH_OPTS[@]}" "${SSH_USER}@${HOST}" "echo ok" >/dev/null

echo "Bootstrapping ${ROLE} on ${HOST}..."
scp "${SSH_OPTS[@]}" "$ROOT/compose/docker-compose.yml" "${SSH_USER}@${HOST}:/tmp/valence-docker-compose.yml"

ssh "${SSH_OPTS[@]}" "${SSH_USER}@${HOST}" bash -s <<'REMOTE'
set -euo pipefail
sudo apt-get update -qq
sudo DEBIAN_FRONTEND=noninteractive apt-get install -y -qq \
  curl pkg-config libssl-dev build-essential ca-certificates gnupg lsb-release git \
  clang libclang-dev llvm-dev

if ! command -v docker >/dev/null 2>&1; then
  curl -fsSL https://get.docker.com | sudo sh
  sudo usermod -aG docker "$USER"
fi

if ! command -v cargo >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
fi
# shellcheck disable=SC1091
source "$HOME/.cargo/env"
rustup default nightly
rustup component add rustfmt clippy || true

mkdir -p "$HOME/valence-services"
mv /tmp/valence-docker-compose.yml "$HOME/valence-services/docker-compose.yml"
cd "$HOME/valence-services"
# New group membership for docker may require newgrp; use sudo if needed.
if docker info >/dev/null 2>&1; then
  docker compose up -d
else
  sudo docker compose up -d
  sudo chown -R "$USER:$USER" "$HOME/valence-services" || true
fi

for i in $(seq 1 60); do
  if docker compose ps 2>/dev/null | grep -q postgres && \
     curl -sf http://127.0.0.1:5432 >/dev/null 2>&1 || \
     sudo docker compose exec -T postgres pg_isready -U valence >/dev/null 2>&1; then
    break
  fi
  sleep 3
done
sudo docker compose exec -T postgres pg_isready -U valence
sudo docker compose exec -T redis redis-cli ping
echo "bootstrap services ready"
REMOTE

echo "Bootstrap ${ROLE} complete."
