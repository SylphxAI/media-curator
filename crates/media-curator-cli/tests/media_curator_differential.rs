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
            use media_curator_cli::dedup::{
                adaptive_threshold, cluster_exact_by_phash, generate_lsh_keys,
                merge_and_deduplicate_clusters,
            };
            let op = case
                .input
                .get("op")
                .and_then(Value::as_str)
                .unwrap_or("exactDup");
            match op {
                "lshKeys" => {
                    let hex = case.input["phashHex"].as_str().expect("phashHex");
                    match generate_lsh_keys(hex) {
                        Some(keys) => json!({ "keys": keys }),
                        None => json!({ "keys": Value::Null }),
                    }
                }
                "adaptiveThreshold" => {
                    let threshold = adaptive_threshold(
                        case.input["duration1"].as_f64().unwrap_or(0.0),
                        case.input["duration2"].as_f64().unwrap_or(0.0),
                        case.input["imageSimilarityThreshold"]
                            .as_f64()
                            .expect("imageSimilarityThreshold"),
                        case.input["imageVideoSimilarityThreshold"]
                            .as_f64()
                            .expect("imageVideoSimilarityThreshold"),
                        case.input["videoSimilarityThreshold"]
                            .as_f64()
                            .expect("videoSimilarityThreshold"),
                    );
                    json!({ "threshold": threshold })
                }
                "mergeClusters" => {
                    let clusters: Vec<Vec<String>> = case.input["clusters"]
                        .as_array()
                        .expect("clusters")
                        .iter()
                        .map(|c| {
                            c.as_array()
                                .expect("cluster")
                                .iter()
                                .map(|v| v.as_str().expect("path").to_string())
                                .collect()
                        })
                        .collect();
                    let merged = merge_and_deduplicate_clusters(&clusters);
                    json!({ "clusters": merged })
                }
                _ => {
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
            }
        }
        "cli/cache-persistence" => {
            use media_curator_cli::cache_keys::{
                metadata_lsh_keys, plan_default_cache_layout, SERIALIZE_MARKER_DATE,
                SERIALIZE_MARKER_MSGPACK, SERIALIZE_MARKER_SHARED_ARRAY_BUFFER,
            };
            if case.input.get("markers").and_then(Value::as_bool) == Some(true) {
                json!({
                    "msgpack": SERIALIZE_MARKER_MSGPACK,
                    "sharedArrayBuffer": SERIALIZE_MARKER_SHARED_ARRAY_BUFFER,
                    "date": SERIALIZE_MARKER_DATE,
                })
            } else if case.input.get("phashHex").is_some() {
                let keys = metadata_lsh_keys(case.input["phashHex"].as_str());
                json!({
                    "lshKeys": [
                        keys[0],
                        keys[1],
                        keys[2],
                        keys[3],
                    ]
                })
            } else {
                let plan = plan_default_cache_layout(
                    case.input["jobName"].as_str().expect("jobName"),
                    case.input["hashKey"].as_str().expect("hashKey"),
                );
                json!({
                    "rootDir": plan.root_dir,
                    "metadataPath": plan.metadata_path,
                    "resultsDb": plan.results_db,
                    "configDb": plan.config_db,
                    "mutexKey": plan.mutex_key,
                })
            }
        }
        "cli/transfer-reporting" => {
            use media_curator_cli::transfer::{plan_transfer_destinations, DupSetInput};
            let unique: Vec<String> = case.input["uniqueFiles"]
                .as_array()
                .expect("uniqueFiles")
                .iter()
                .map(|v| v.as_str().expect("path").to_string())
                .collect();
            let sets: Vec<DupSetInput> = case.input["duplicateSets"]
                .as_array()
                .expect("duplicateSets")
                .iter()
                .map(|s| DupSetInput {
                    best_file: s["bestFile"].as_str().expect("bestFile").to_string(),
                    duplicates: s["duplicates"]
                        .as_array()
                        .expect("duplicates")
                        .iter()
                        .map(|v| v.as_str().expect("dup").to_string())
                        .collect(),
                })
                .collect();
            let errors: Vec<String> = case.input["errorFiles"]
                .as_array()
                .expect("errorFiles")
                .iter()
                .map(|v| v.as_str().expect("path").to_string())
                .collect();
            let plan = plan_transfer_destinations(
                &unique,
                &sets,
                &errors,
                case.input["hasDuplicateDir"].as_bool().unwrap_or(false),
                case.input["hasErrorDir"].as_bool().unwrap_or(false),
            );
            json!({
                "targetCount": plan.target_count,
                "duplicateCount": plan.duplicate_count,
                "errorCount": plan.error_count,
                "skipCount": plan.skip_count,
                "actions": plan.actions.iter().map(|a| json!({
                    "sourcePath": a.source_path,
                    "bucket": a.bucket,
                    "relativeKey": a.relative_key,
                })).collect::<Vec<_>>(),
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
fn cli_transfer_reporting_differential_matches_ts_oracle() {
    run_slice("cli/transfer-reporting");
}

#[test]
fn cli_cache_persistence_differential_matches_ts_oracle() {
    run_slice("cli/cache-persistence");
}

#[test]
fn media_curator_differential_matches_ts_oracle() {
    let oracle = run_ts_oracle();
    assert_eq!(oracle.corpus_version, 1);
    for case in &oracle.cases {
        compare_case(case);
    }
}