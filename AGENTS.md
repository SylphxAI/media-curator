# Media Curator Agent Instructions

## Scope

This file is the repo-local operating policy for agents working in
`SylphxAI/media-curator`. Org-wide engineering doctrine is owned by
`SylphxAI/doctrine`; `PROJECT.md` and `.doctrine/project.json` own this
repository's local identity, lifecycle, boundary, and delivery facts.

Media Curator is a local CLI package for organizing and deduplicating user-owned
photo and video libraries. It may scan, copy, move, and classify filesystem
content selected by the user, so media roots, transfer behavior, cache/database
state, and destructive actions must stay explicit and testable.

## Read First

Before proposing or implementing changes, read the smallest relevant set of
these source-of-truth documents:

1. `PROJECT.md` and `.doctrine/project.json` — project goal, lifecycle,
   boundaries, public surfaces, delivery proof, and adoption gaps.
2. `README.md` — public CLI contract, install surface, command options,
   external prerequisites, and user-facing behavior.
3. `ARCHITECTURE.md` — pipeline stages, service wrappers, cache/database
   boundaries, workerpool, and WASM architecture.
4. `memory-bank/projectbrief.md` — product goal, current scope, non-goals, and
   scalability requirements.
5. `package.json` and the touched `index.ts`/pipeline/service/test files before
   changing CLI behavior, package release, or validation commands.
6. `.github/workflows/ci.yml` and `.github/workflows/release.yml` before
   changing CI, package publishing, or release behavior.

## Non-Negotiables

- Do not commit private media, generated thumbnails from private libraries,
  filesystem listings, credentials, `.env` files, or machine-local cache data.
- Treat source directories, destination directories, duplicate directories, and
  error directories as user-owned data boundaries. Do not delete or mutate
  outside explicit CLI arguments.
- Any `--move` or destructive transfer behavior must be documented, covered by
  tests or fixtures, and fail closed on path ambiguity.
- Keep media processing local by default. Do not add cloud storage, hosted
  indexing, telemetry, or remote media upload without a new explicit product
  decision.
- Preserve external-tool boundaries around FFmpeg, ExifTool, Sharp/libvips,
  SQLite, LMDB, workerpool, and WASM. Normalize errors instead of leaking
  provider-specific failure modes into user workflows.
- Package publishing must be Changesets-driven and bot/workflow-owned before it
  is treated as a production release path. Do not publish from a human shell or
  personal token.
- Use branch -> commit -> PR. Do not push directly to `main`, force-push, merge,
  publish, or release without manager-visible evidence and required gates.

## Workflow

1. Identify the owning boundary: CLI options, discovery, metadata extraction,
   hashing, deduplication, transfer, cache/database state, worker/WASM
   performance, docs, CI, or package release.
2. Check open PRs/issues for the same boundary before editing, especially active
   CI repair and release-control changes.
3. Prefer small, evidence-backed slices with fixtures for filesystem behavior
   and benchmarks for algorithmic/performance claims.
4. For behavior that can move files, include before/after examples and failure
   modes in docs or tests.
5. Keep shared media algorithms tenant-neutral; product-specific organization
   policies belong in config/adapters, not hard-coded core logic.

## Validation

Use the narrowest meaningful validation first, then broaden as needed:

- `bun run check-format`
- `bun run lint`
- `bun run typecheck`
- `bun run test`
- `bun run test:cov`
- `bun run build`
- `bun run validate`

Docs-only boundary changes may be validated by reviewing the diff and checking
referenced files exist. CLI transfer, deduplication, cache/database, and
release changes need targeted tests and, where safe, fixture-backed smoke
evidence that does not touch real media libraries.

## Reporting

When reporting completed work, include changed files, boundaries read,
validation run, PR/issue links, and residual risk. Be explicit when no runtime
behavior, filesystem mutation, cache/database state, external-tool behavior,
npm package, or release state changed.
