//! TRUE differential parity: frozen TS oracle vs native Rust SSOT (media-curator-cli).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use media_curator_cli::discovery::{discover_files, DiscoverOptions};
use media_curator_cli::file_stats::file_stats_for_path;
use media_curator_cli::hamming::hamming_distance;
use serde::Deserialize;
use serde_json::{json, Value};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[derive(Debug, Deserialize)]
struct OracleCase {
    id: String,
    slice: String,
    input: Value,
    output: Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OracleCorpus {
    corpus_version: u32,
    fixture_corpus_hash: String,
    behavior_spec_hash: String,
    cases: Vec<OracleCase>,
}

fn run_ts_oracle() -> OracleCorpus {
    if let Ok(path) = std::env::var("MEDIA_CURATOR_ORACLE_JSON") {
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read oracle at {path}: {error}"));
        return serde_json::from_str(&raw).expect("oracle file must be valid JSON");
    }

    let script = repo_root().join("scripts/differential/media-curator-oracle.ts");
    let output = Command::new("bun")
        .arg("run")
        .arg(&script)
        .current_dir(repo_root())
        .output()
        .unwrap_or_else(|error| panic!("spawn TS oracle at {}: {error}", script.display()));

    assert!(
        output.status.success(),
        "TS oracle failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    serde_json::from_slice(&output.stdout).expect("oracle output must be valid JSON")
}

fn compare_case(case: &OracleCase) {
    let native = match case.slice.as_str() {
        "cli/metadata-extraction" => {
            let file = repo_root().join(case.input["file"].as_str().expect("file"));
            let stats = file_stats_for_path(&file).expect("file stats");
            json!({ "md5": stats.md5, "size": stats.size })
        }
        "cli/discovery-gather" => {
            let root = repo_root();
            let source_dirs = case.input["sourceDirs"]
                .as_array()
                .expect("sourceDirs")
                .iter()
                .map(|value| root.join(value.as_str().expect("dir")))
                .collect::<Vec<_>>();
            let concurrency = case.input["concurrency"].as_u64().unwrap_or(1) as usize;
            let map = discover_files(DiscoverOptions {
                source_dirs,
                concurrency,
            })
            .expect("discover");
            let prefix = root.to_string_lossy().to_string();
            let prefix = if prefix.ends_with('/') {
                prefix
            } else {
                format!("{prefix}/")
            };
            let mut by_extension = serde_json::Map::new();
            for (ext, paths) in map.by_extension {
                let normalized = paths
                    .into_iter()
                    .map(|path| {
                        if let Some(stripped) = path.strip_prefix(&prefix) {
                            stripped.to_string()
                        } else if let Ok(relative) = Path::new(&path).strip_prefix(&root) {
                            relative.to_string_lossy().into_owned()
                        } else {
                            path
                        }
                    })
                    .collect::<Vec<_>>();
                by_extension.insert(ext, json!(normalized));
            }
            json!({
                "byExtension": by_extension,
                "stats": {
                    "fileCount": map.stats.file_count,
                    "dirCount": map.stats.dir_count,
                },
            })
        }
        "cli/perceptual-hash-lsh" => {
            let hash1 = hex::decode(case.input["hash1Hex"].as_str().expect("hash1Hex"))
                .expect("hash1 hex");
            let hash2 = hex::decode(case.input["hash2Hex"].as_str().expect("hash2Hex"))
                .expect("hash2 hex");
            json!({ "distance": hamming_distance(&hash1, &hash2) })
        }
        "cli/deduplication-engine" => {
            use media_curator_cli::dedup::cluster_exact_by_phash;
            let entries = case.input["entries"]
                .as_array()
                .expect("entries")
                .iter()
                .map(|entry| {
                    let path = entry["path"].as_str().expect("path").to_string();
                    let phash = entry
                        .get("phashHex")
                        .and_then(Value::as_str)
                        .map(str::trim)
                        .filter(|value| !value.is_empty())
                        .map(str::to_string);
                    (path, phash)
                })
                .collect::<Vec<_>>();
            let result = cluster_exact_by_phash(&entries);
            json!({
                "clusters": result.clusters.iter().map(|cluster| json!({
                    "phashHex": cluster.phash_hex,
                    "paths": cluster.paths,
                })).collect::<Vec<_>>(),
                "singletons": result.singletons,
                "missingPhash": result.missing_phash,
            })
        }
        other => panic!("unsupported slice {other} in case {}", case.id),
    };

    assert_eq!(native, case.output, "case {}", case.id);
}

fn run_slice(slice: &str) {
    let oracle = run_ts_oracle();
    for case in oracle.cases.iter().filter(|case| case.slice == slice) {
        compare_case(case);
    }
}

#[test]
fn cli_metadata_extraction_differential_matches_ts_oracle() {
    run_slice("cli/metadata-extraction");
}

#[test]
fn cli_discovery_gather_differential_matches_ts_oracle() {
    run_slice("cli/discovery-gather");
}

#[test]
fn cli_perceptual_hash_lsh_differential_matches_ts_oracle() {
    run_slice("cli/perceptual-hash-lsh");
}

#[test]
fn cli_deduplication_engine_exact_dup_differential_matches_ts_oracle() {
    run_slice("cli/deduplication-engine");
}

#[test]
fn media_curator_differential_matches_ts_oracle() {
    let oracle = run_ts_oracle();
    assert_eq!(oracle.corpus_version, 1);
    for case in &oracle.cases {
        compare_case(case);
    }
}