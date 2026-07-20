#!/usr/bin/env bash
set -euo pipefail
cd "$(cd "$(dirname "$0")/.." && pwd)"
tmp="$(mktemp)"; trap 'rm -f "$tmp"' EXIT
while IFS= read -r -d '' file; do
  case "$file" in scripts/check-language-hygiene.sh) continue ;; esac
  if grep -nEi '(^|[^[:alnum:]_])(dens|densed|fleet)([^[:alnum:]_]|$)' -- "$file" >>"$tmp" 2>/dev/null; then :; fi
done < <(git ls-files -z)
if [[ -s "$tmp" ]]; then echo "ERROR dens/fleet pollution:" >&2; cat "$tmp" >&2; exit 1; fi
echo "OK: language hygiene"
