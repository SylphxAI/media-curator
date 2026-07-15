# media-curator — local agent notes only

Doctrine and fleet delivery law live in the **host always-on constitution**
(`~/.grok/AGENTS.md` / Doctrine template). This file must **not** restate,
weaken, or fork that law (including PR-vs-direct-trunk delivery).

Local truth: [`PROJECT.md`](./PROJECT.md), [`.doctrine/project.json`](./.doctrine/project.json)
when present.

## Boundary hazards

- Never commit private media, private thumbnails, filesystem listings,
  credentials, `.env`, or machine-local cache data.
- Source/destination/duplicate/error dirs are user-owned; do not mutate outside
  explicit CLI arguments.
- `--move` / destructive transfers must be documented, test-covered, fail-closed
  on path ambiguity.
- Media processing is local by default; no cloud storage/indexing/upload without
  an explicit product decision.
- Normalize errors from FFmpeg / ExifTool / Sharp / SQLite / LMDB / WASM; do not
  leak provider-specific failure modes.
- Package publishing is Changesets / bot-owned; do not publish from a human shell
  or personal token.

## Local commands

```bash
bun run check-format
bun run lint
bun run typecheck
bun run test
bun run test:cov
bun run build
bun run validate
```

## Validation notes

- Docs-only: diff review + referenced paths exist.
- CLI transfer / dedupe / cache / release: fixture-backed tests; do not touch
  real media libraries in smoke.
- Report layers honestly: local diff · trunk FF · publish (if in scope).
