# media-curator — local agent notes only

Static engineering and delivery standards load from the active Skills runtime
([SylphxAI/skills](https://github.com/SylphxAI/skills) is binding instruction
SSOT). Doctrine and Mission Control are retired historical lineage and must not
be loaded as current instruction authority.

Local truth: `PROJECT.md`.

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

## Backend false-authority fence

Work: wi_01KYFN6993PMG8WD00Q51AE231

If this repository has completed a **Rust backend** cutover:

1. Production backend behavior authority is the Rust crate/binary/service path declared in deploy manifests / package native bin / Docker ENTRYPOINT / `sylphx.toml`.
2. Residual TypeScript service trees are **not** product authority unless explicitly proven still on the live path.
3. Do not "fix production" by editing residual TypeScript and assuming runtime will pick it up.
4. Prefer deleting residual TS backend trees after Rust sole proof; keep history in Git.
5. Intentional TypeScript frontends, npm packaging wrappers, and native-binding surfaces may remain.
