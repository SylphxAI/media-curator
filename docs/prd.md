# PRD — Media Curator

**Authority:** capability inventory. Field-level I/O and behavior live in the
README, architecture notes, schemas, and tests.

## Problem

Organize, perceptually deduplicate, and safely transfer large local media
libraries without silent data loss.

## Product shape

Media Curator is a local CLI. TypeScript owns CLI orchestration and user-facing
formatting; the staged Rust binary owns native filesystem and hashing primitives
on the declared path. It is not a hosted media service, cloud index, uploader,
or background sync process.

## Capability jobs

| Job | Success looks like | Explicit non-goal |
| --- | --- | --- |
| `media.organize` | Files land in the requested format/metadata tree | Silent data loss or unreported transfer failure |
| `media.dedupe` | Exact and perceptual clusters retain every selected representative | Exact-hash-only behavior |
| `media.transfer` | Copy/move creates a new destination exclusively; move removes the source only after the copy succeeds | Overwriting a target or reporting success after a failed transfer |

## North-star evidence

Correct organize/dedupe/transfer results on ground-truth fixtures. TS-vs-Rust
parity theater and a second mutable store are anti-proxies, not success
metrics.

## Boundaries

- Operate only on source, destination, duplicate, and error roots supplied by
  the user.
- Keep destructive `--move` behavior explicit, documented, and test-covered.
- Do not add a hosted API, cloud storage, telemetry collector, or remote media
  index to local/static tooling.
- Do not encode customer-specific organization policy into shared core logic.
