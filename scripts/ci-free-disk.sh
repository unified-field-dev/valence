#!/usr/bin/env bash
# Reclaim space on GitHub-hosted Ubuntu runners before heavy Cargo builds.
# Hosted images ship large unused SDKs; a full Valence workspace build needs the headroom.
set -euo pipefail

df -h || true
echo "::group::Remove unused runner software"
sudo rm -rf /usr/share/dotnet || true
sudo rm -rf /usr/local/lib/android || true
sudo rm -rf /opt/ghc || true
sudo rm -rf /usr/local/.ghcup || true
sudo rm -rf /opt/hostedtoolcache/CodeQL || true
sudo rm -rf /opt/hostedtoolcache/Python || true
sudo rm -rf /opt/hostedtoolcache/PyPy || true
sudo rm -rf /opt/hostedtoolcache/node || true
sudo rm -rf /opt/hostedtoolcache/go || true
sudo docker system prune -af || true
echo "::endgroup::"
df -h || true
