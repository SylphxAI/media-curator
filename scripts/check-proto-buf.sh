#!/usr/bin/env bash
# Protobuf+Buf conformance gate (ADR-168 S0) for media-curator.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
BUF_BIN="${BUF_BIN:-}"
if [[ -z "$BUF_BIN" ]]; then
  for c in "$ROOT/.tools/buf" /usr/local/bin/buf "$HOME/.local/bin/buf"; do
    if [[ -x "$c" ]]; then BUF_BIN=$c; break; fi
  done
fi
if [[ -z "$BUF_BIN" ]] && command -v buf >/dev/null 2>&1; then
  BUF_BIN=$(command -v buf)
fi
if [[ -z "$BUF_BIN" ]]; then
  echo "buf CLI required" >&2
  exit 1
fi
"$BUF_BIN" lint
"$BUF_BIN" build -o /dev/null
echo "check-proto-buf: OK"
