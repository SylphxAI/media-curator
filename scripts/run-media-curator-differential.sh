#!/usr/bin/env bash
# Media Curator bounded differential parity — frozen TS oracle vs Rust SSOT.
# Fail-closed: requires bun (no SKIP-as-pass).
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SCRATCH="${SCRATCH_DIR:-/tmp/media-curator-differential}"
mkdir -p "$SCRATCH"
LOG="$SCRATCH/differential.log"
ARTIFACT="$SCRATCH/verification.json"
ORACLE_JSON="$SCRATCH/oracle.json"
SLICE_FILTER="all"
: >"$LOG"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --slice)
      SLICE_FILTER="${2:-}"
      shift 2
      ;;
    *)
      echo "::error::unknown argument: $1" | tee -a "$LOG"
      exit 1
      ;;
  esac
done

case "$SLICE_FILTER" in
  all|cli/metadata-extraction|cli/discovery-gather|cli/perceptual-hash-lsh) ;;
  *)
    echo "::error::invalid --slice value: $SLICE_FILTER" | tee -a "$LOG"
    exit 1
    ;;
esac

cd "$REPO_ROOT"

if ! command -v bun >/dev/null 2>&1; then
  echo "::error::bun required for media-curator differential parity — no SKIP-as-pass" | tee -a "$LOG"
  exit 1
fi

echo "=== media-curator bounded differential $(date -Iseconds) slice=$SLICE_FILTER ===" | tee -a "$LOG"

echo "--- TS oracle ---" | tee -a "$LOG"
bun run "$REPO_ROOT/scripts/differential/media-curator-oracle.ts" >"$ORACLE_JSON" 2>>"$LOG"

run_rust_slice_test() {
  local label="$1"
  local test_name="$2"
  echo "--- Rust bounded slice: $label ---" | tee -a "$LOG"
  MEDIA_CURATOR_ORACLE_JSON="$ORACLE_JSON" \
    cargo test -p media-curator-cli --test media_curator_differential "$test_name" -- --nocapture 2>&1 | tee -a "$LOG"
}

case "$SLICE_FILTER" in
  cli/metadata-extraction)
    run_rust_slice_test "cli/metadata-extraction" cli_metadata_extraction_differential_matches_ts_oracle
    ;;
  cli/discovery-gather)
    run_rust_slice_test "cli/discovery-gather" cli_discovery_gather_differential_matches_ts_oracle
    ;;
  cli/perceptual-hash-lsh)
    run_rust_slice_test "cli/perceptual-hash-lsh" cli_perceptual_hash_lsh_differential_matches_ts_oracle
    ;;
  all)
    run_rust_slice_test "full-corpus" media_curator_differential_matches_ts_oracle
    ;;
esac

CANDIDATE_SHA="${CANDIDATE_SHA:-$(git -C "$REPO_ROOT" rev-parse HEAD 2>/dev/null || echo unknown)}"
BASELINE_TS_SHA="$(git -C "$REPO_ROOT" log -1 --format=%H -- src/comparatorUtils.ts src/discovery.ts src/jobs/fileStats.ts scripts/differential 2>/dev/null || echo unknown)"
BEHAVIOR_SPEC_HASH="$(jq -r '.behaviorSpecHash' "$ORACLE_JSON")"
FIXTURE_CORPUS_HASH="$(jq -r '.fixtureCorpusHash' "$ORACLE_JSON")"
CASE_COUNT="$(jq '.cases | length' "$ORACLE_JSON")"

jq -n \
  --arg verifiedAt "$(date -Iseconds)" \
  --arg candidateSha "$CANDIDATE_SHA" \
  --arg baselineTsSha "$BASELINE_TS_SHA" \
  --arg rustCandidateSha "$CANDIDATE_SHA" \
  --arg behaviorSpecHash "$BEHAVIOR_SPEC_HASH" \
  --arg fixtureCorpusHash "$FIXTURE_CORPUS_HASH" \
  --arg sliceFilter "$SLICE_FILTER" \
  --argjson caseCount "$CASE_COUNT" \
  '{
    schemaVersion: 2,
    slice: (if $sliceFilter == "all" then "cli/metadata-extraction|cli/discovery-gather|cli/perceptual-hash-lsh" else $sliceFilter end),
    sliceFilter: $sliceFilter,
    status: "differential_green",
    verifiedAt: $verifiedAt,
    lastComparedMainSha: $candidateSha,
    mergeGroupSha: $candidateSha,
    baselineTsSha: $baselineTsSha,
    rustCandidateSha: $rustCandidateSha,
    behaviorSpecHash: $behaviorSpecHash,
    fixtureCorpusHash: $fixtureCorpusHash,
    caseCount: $caseCount,
    harness: "scripts/run-media-curator-differential.sh",
    differentialTest: "crates/media-curator-cli/tests/media_curator_differential.rs#media_curator_differential_matches_ts_oracle",
    oracle: "scripts/differential/media-curator-oracle.ts"
  }' >"$ARTIFACT"

echo "media-curator-differential: OK (cases=$CASE_COUNT)" | tee -a "$LOG"
echo "verification artifact: $ARTIFACT" | tee -a "$LOG"