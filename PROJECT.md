# Media Curator Project

Media Curator is a local CLI package for organizing and deduplicating large
photo and video libraries. It discovers user-selected media, extracts metadata
and perceptual hashes, indexes files through SQLite/LMDB-backed state, compares
similarity through LSH/WASM-assisted logic, and transfers files into the
requested destination structure.

## Lifecycle

- Lifecycle: `active`
- Layer: `tooling`
- Doctrine source of truth: [SylphxAI/doctrine](https://github.com/SylphxAI/doctrine)
- Machine manifest: `.doctrine/project.json`

## Goals

- Provide a reliable local CLI for organizing and deduplicating large media
  libraries.
- Own media discovery, metadata extraction, perceptual hashing, LSH similarity,
  cache/database state, worker/WASM performance, and transfer behavior.
- Keep destructive filesystem actions explicit, documented, and validated with
  fixtures before production release.

## Non-Goals

- Do not become a hosted media library, cloud storage product, telemetry
  collector, or background sync service.
- Do not own user media outside CLI-selected roots, destination directories,
  duplicate directories, or error directories.
- Do not encode product-specific or customer-specific organization policies into
  shared core logic.

## Boundaries

Media Curator owns the local CLI, processing pipeline, metadata/hash extraction,
deduplication algorithms, cache/database format, worker/WASM acceleration,
package docs, and package release path. It does not own cloud media storage,
remote identity, hosted retention, operating-system photo libraries, or
filesystem paths outside user-supplied arguments.

## Public Surfaces

- CLI package: `package.json`
- CLI entry point: `index.ts`
- User docs: `README.md`
- Architecture docs: `ARCHITECTURE.md`
- CI/release workflows: `.github/workflows/ci.yml` and
  `.github/workflows/release.yml`

## Delivery

Pull requests currently use the legacy `Validate Code & Run Tests` GitHub
Actions context on Sylphx self-hosted runners. Package release intent is present
through Changesets and the central reusable release workflow, but the older
tag-driven publish path remains an adoption gap until release ownership is
fully consolidated.

Docs-only boundary changes do not alter runtime behavior, filesystem mutation,
external tools, package output, or npm publication. Runtime changes need focused
tests or fixture-backed smoke proof; package releases need build output,
Changesets intent, bot/workflow ownership, and npm readback.

## Commercial Direction

This repo is a public utility package. Pricing, packaging, hosted media
processing, enterprise deduplication, or commercial roadmap changes require a
decision record backed by competitor and customer analysis rather than
repo-local preference.
