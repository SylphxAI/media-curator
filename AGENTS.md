# media-curator — local agent notes only

Doctrine and host delivery law live in the **host always-on constitution**
(`~/.grok/AGENTS.md` / Doctrine template). This file must **not** restate,
weaken, or fork that law (including PR-vs-direct-trunk delivery).

Local truth: `PROJECT.md`, `.doctrine/project.json` when present.

## Boundary hazards

- Never commit private media, private thumbnails, filesystem listings,
- Source/destination/duplicate/error dirs are user-owned; do not mutate outside
- `--move` / destructive transfers must be documented, test-covered, fail-closed
- Media processing is local by default; no cloud storage/indexing/upload without
- Normalize errors from FFmpeg / ExifTool / Sharp / SQLite / LMDB / WASM; do not
- Package publishing is Changesets / bot-owned; do not publish from a human shell

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

- Prefer the **narrowest** affected check before full workspace runs.
- Report layers honestly: local diff · trunk FF · deploy · prod proof (do not collapse).

## Language hygiene

Machine gate: `bash scripts/check-language-hygiene.sh`.
