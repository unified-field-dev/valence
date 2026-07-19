#!/usr/bin/env bash
# Rsync valence tree and run E2E or bench campaign on a provisioned host.
set -euo pipefail

ROOT_INFRA="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$ROOT_INFRA/../../.." && pwd)"
ENV_FILE="${INSTANCES_ENV:-$ROOT_INFRA/instances.env}"
# shellcheck disable=SC1091
source "$ENV_FILE"

ROLE="${1:?usage: deploy-and-run.sh e2e|bench}"
case "$ROLE" in
  e2e)
    HOST="${E2E_PUBLIC_IP}"
    HARDWARE="${E2E_INSTANCE_TYPE:-$VALENCE_BENCH_HARDWARE}"
    REMOTE_CMD='./scripts/aws-e2e-bench.sh --e2e'
    ;;
  bench)
    HOST="${BENCH_PUBLIC_IP}"
    HARDWARE="${BENCH_INSTANCE_TYPE:-$VALENCE_BENCH_HARDWARE}"
    REMOTE_CMD='./scripts/aws-e2e-bench.sh --bench all'
    ;;
  *)
    echo "role must be e2e or bench" >&2
    exit 1
    ;;
esac

SSH_KEY="${SSH_KEY_PATH:-$HOME/.ssh/id_ed25519}"
SSH_OPTS=(-o StrictHostKeyChecking=accept-new -o ConnectTimeout=30 -i "$SSH_KEY")

echo "== Rsync repo to ${SSH_USER}@${HOST} =="
rsync -az --delete \
  --exclude target --exclude 'target-*' --exclude .git \
  --exclude node_modules --exclude .cursor --exclude profiling \
  --exclude 'infra/aws/campaign/instances.env' \
  -e "ssh ${SSH_OPTS[*]}" \
  "$REPO_ROOT/" "${SSH_USER}@${HOST}:~/valence/"

echo "== Run ${ROLE} campaign =="
ssh "${SSH_OPTS[@]}" "${SSH_USER}@${HOST}" bash -s <<EOF
set -euo pipefail
# shellcheck disable=SC1091
source "\$HOME/.cargo/env"
export CARGO_BUILD_JOBS=1
export CARGO_INCREMENTAL=0
export RUST_BACKTRACE=1
export VALENCE_BENCH_HARDWARE=${HARDWARE}
export VALENCE_BENCH_ROCKSDB=1
# Demo-only credentials matching compose/docker-compose.yml — not for production.
export DATABASE_URL=postgres://valence:valence@127.0.0.1:5432/valence
export VALENCE_MONGODB_URI=mongodb://127.0.0.1:27017
export VALENCE_REDIS_URL=redis://127.0.0.1:6379
export VALENCE_REDIS_URLS=redis://127.0.0.1:6379
cd "\$HOME/valence"
${REMOTE_CMD}
EOF

if [[ "$ROLE" == "bench" ]]; then
  mkdir -p "$REPO_ROOT/profiling/valence-bench/reports/aws"
  rsync -az -e "ssh ${SSH_OPTS[*]}" \
    "${SSH_USER}@${HOST}:~/valence/profiling/valence-bench/reports/" \
    "$REPO_ROOT/profiling/valence-bench/reports/aws/" || true
  echo "Fetched bench reports to profiling/valence-bench/reports/aws/"
fi

echo "== ${ROLE} campaign finished =="
