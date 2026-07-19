#!/usr/bin/env bash
# Aggregate bc multibench JSON reports into total ops/s.
set -euo pipefail

REPORT_DIR="${1:-profiling/valence-bench/reports}"
PATTERN="${2:-bm-v7-*}"

total=0
count=0
for f in "$REPORT_DIR"/$PATTERN; do
  [[ -f "$f" ]] || continue
  ops=$(jq -r '.write.achieved_write_ops_per_sec // .ops_per_sec // 0' "$f")
  total=$(awk -v a="$total" -v b="$ops" 'BEGIN { print a + b }')
  count=$((count + 1))
  echo "$f -> $ops ops/s"
done

if [[ "$count" -eq 0 ]]; then
  echo "no reports matched $REPORT_DIR/$PATTERN" >&2
  exit 1
fi

echo "aggregate: $total ops/s across $count bench clients"
