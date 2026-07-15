#!/usr/bin/env bash
# Fail-closed: published package must ship bin/native/media-curator-cli for Rust authority.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BIN="$ROOT/bin/native/media-curator-cli"

if [[ ! -x "$BIN" ]]; then
  echo "FAIL: missing executable $BIN — run bun run stage:rust" >&2
  exit 1
fi
if ! grep -q 'bin/native/media-curator-cli' "$ROOT/src/external/rustCli.ts"; then
  echo "FAIL: rustCli.ts must resolve bin/native/media-curator-cli" >&2
  exit 1
fi
if ! node -e "const p=require('./package.json'); if(!Array.isArray(p.files)||!p.files.some(f=>String(f).includes('bin'))) process.exit(1)"; then
  echo "FAIL: package.json files must include bin/" >&2
  exit 1
fi
echo "PASS: native media-curator-cli is staged for npm pack"
