# Valence AWS E2E / bench campaign

Two-host campaign (parallel):

| Host | Role |
|------|------|
| `valence-campaign-e2e` | Docker wire services + `./scripts/aws-e2e-bench.sh --e2e` |
| `valence-campaign-bench` | Docker wire services + `./scripts/aws-e2e-bench.sh --bench all` |

Uses default VPC in `us-west-2`, key pair `valence-campaign` (imported from local `~/.ssh/id_ed25519`).

## Operator flow

```bash
export AWS_KEY_NAME=valence-campaign
export SSH_KEY_PATH=$HOME/.ssh/id_ed25519

./infra/aws/campaign/provision.sh
./infra/aws/campaign/bootstrap.sh e2e
./infra/aws/campaign/bootstrap.sh bench

# Parallel campaigns
./infra/aws/campaign/deploy-and-run.sh e2e &
./infra/aws/campaign/deploy-and-run.sh bench &
wait

./infra/aws/campaign/teardown.sh
```

State file: `infra/aws/campaign/instances.env` (gitignored).
Bench reports sync to `profiling/valence-bench/reports/aws/`.
