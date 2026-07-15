#!/usr/bin/env bash
# Fail-closed: production file-stats must default to Rust authority (no silent TS).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# Default must be Rust when env is unset
if ! grep -q 'return true' "$ROOT/src/external/rustCli.ts"; then
  echo "FAIL: rustCliDelegationEnabled must default true when MEDIA_CURATOR_RUST unset" >&2
  exit 1
fi

# processFileStats must route through fileStatsViaRust under Rust authority
if ! grep -q 'fileStatsViaRust' "$ROOT/src/jobs/fileStats.ts"; then
  echo "FAIL: processFileStats must call fileStatsViaRust" >&2
  exit 1
fi

# TS core must be named opt-out path, not production default symbol alone
if ! grep -q 'processFileStatsTsCore' "$ROOT/src/jobs/fileStats.ts"; then
  echo "FAIL: TS parity baseline processFileStatsTsCore must exist for opt-out only" >&2
  exit 1
fi

# Discovery production path must call discoverViaRust
if ! grep -q 'discoverViaRust' "$ROOT/src/discovery.ts"; then
  echo "FAIL: discoverFilesFn must call discoverViaRust under Rust authority" >&2
  exit 1
fi

echo "PASS: production file-stats + discovery default to Rust authority"
