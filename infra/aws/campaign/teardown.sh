#!/usr/bin/env bash
# Terminate campaign instances and delete the campaign security group.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ENV_FILE="${INSTANCES_ENV:-$ROOT/instances.env}"
# shellcheck disable=SC1091
source "$ENV_FILE"

AWS_REGION="${AWS_REGION:-us-west-2}"
IDS=()
[[ -n "${INSTANCE_E2E:-}" ]] && IDS+=("$INSTANCE_E2E")
[[ -n "${INSTANCE_BENCH:-}" ]] && IDS+=("$INSTANCE_BENCH")

if [[ "${#IDS[@]}" -gt 0 ]]; then
  echo "Terminating ${IDS[*]}..."
  aws ec2 terminate-instances --region "$AWS_REGION" --instance-ids "${IDS[@]}" >/dev/null
  aws ec2 wait instance-terminated --region "$AWS_REGION" --instance-ids "${IDS[@]}"
fi

if [[ -n "${SECURITY_GROUP_ID:-}" ]]; then
  echo "Deleting security group ${SECURITY_GROUP_ID}..."
  aws ec2 delete-security-group --region "$AWS_REGION" --group-id "$SECURITY_GROUP_ID" || true
fi

echo "Teardown complete."
