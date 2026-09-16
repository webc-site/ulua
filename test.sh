#!/usr/bin/env bash
set -euo pipefail

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$DIR"

cargo build --workspace --bins -q

exec cargo nextest run \
  --status-level fail --final-status-level fail \
  --workspace "$@"
