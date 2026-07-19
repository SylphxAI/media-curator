# ADR-168 — Fleet Media Curator Rust North Star architecture

- **Status:** Accepted
- **Date:** 2026-07-09
- **Relates to:** ADR-167 (SylphxAI/doctrine)
- **Change class:** `required-future` for Media Curator; `advisory` for fleet

## Context

Media Curator is a local CLI for organizing and deduplicating large photo and video
libraries. It discovers media, extracts metadata and perceptual hashes, indexes through
SQLite/LMDB-backed state, compares similarity via LSH/WASM-assisted logic, and
transfers files. The processing pipeline is the primary authority surface.

Doctrine [ADR-167](https://github.com/SylphxAI/doctrine/blob/main/docs/adr/ADR-167-boundary-contract-stack-and-platform-pillars.md)
uses Rust for performance-sensitive filesystem primitives and TypeScript for the
CLI pipeline and ecosystem adapters. This is the terminal architecture boundary:
language choice follows responsibility, not a repository-wide language rewrite.

## Decision

### 1. North Star production stack (Media Curator repo)

| Layer                   | North Star                                                         | Transitional (until sunset slice)     |
| ----------------------- | ------------------------------------------------------------------ | ------------------------------------- |
| Cross-boundary contract | Protobuf + Buf (`proto/media_curator/v1/`) for plugin/MCP surfaces | CLI flags as implicit contract        |
| CLI orchestration       | TypeScript pipeline                                                 | —                                     |
| Native primitives       | Rust `crates/media-curator-cli`                                    | filesystem-heavy TypeScript code      |
| Distribution            | npm package with staged Rust binary                                | —                                     |
| WASM acceleration       | Rust-native SIMD where applicable                                  | TS/WASM worker during cutover         |
| Persistence             | SQLite/LMDB via Rust (`media-curator-store`)                       | TS-backed cache format during cutover |

### 2. Ownership matrix

| Concern                                     | Owner                      | Media Curator may                  | Media Curator must not        |
| ------------------------------------------- | -------------------------- | ---------------------------------- | ----------------------------- |
| Local dedup/organize CLI, hash/LSH pipeline | **SylphxAI/media-curator** | Own cache format and algorithms    | Become hosted cloud product   |
| User filesystem                             | End user                   | Operate only on CLI-selected roots | Sync or upload media remotely |

### 3. Strangler-fig cutover posture

- **S0:** Rust CLI crate + fixture corpus; npm wrapper spawns Rust binary.
- **S1:** Metadata/hash extraction parity against TS fixtures on sample libraries.
- **S2:** LSH similarity + transfer logic on Rust; dual-run on fixture sets.
- **S3:** Delete TS engine authority; npm package is distribution-only.
- Destructive filesystem actions require explicit flags preserved across cutover.

### 4. Contract stack (ADR-167 alignment)

- **Protobuf + Buf** for any future MCP/plugin cross-boundary surfaces.
- CLI flag contract is the public HTTP-equivalent edge until proto surfaces land.
- Hand-written parallel schema definitions rejected for cross-repo integration.

## Alternatives considered

| Alternative               | Why rejected                                     |
| ------------------------- | ------------------------------------------------ |
| Permanent TS CLI engine   | Contradicts ADR-167 for tooling authority        |
| Hosted SaaS pivot         | Violates PROJECT.md non-goals                    |
| Skip Rust for "small CLI" | Tooling layer is in fleet cutover registry scope |

## Consequences

- New pipeline code defaults to `crates/media-curator-*`.
- npm package ships the Rust binary used by native primitives.
- No mutable migration ledger or dual implementation is retained after cutover.

## Validation

- Fixture library parity (hash, grouping, transfer dry-run)
- CLI flag contract unchanged through cutover
- `cargo test` + `cargo clippy -D warnings`
- `python3 $DOCTRINE/scripts/project-control-plane-audit.py --local . --fail-on-drift`
