//! Pure transfer routing planner (TS transfer residual — no FS IO).
//!
//! Classifies paths into target / duplicate-sidecar / error buckets from
//! deduplication + gather results. Path *format* templating remains residual
//! (date/RND-dependent); this slice only plans *which* bucket + relative key.

use serde::Serialize;
use std::collections::BTreeMap;

/// One planned transfer action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlannedTransfer {
    pub source_path: String,
    /// Bucket: "target" | "duplicate" | "error" | "skip"
    pub bucket: String,
    /// Relative key under the bucket root (basename of best file for duplicates).
    pub relative_key: String,
}

/// Full transfer plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferPlan {
    pub actions: Vec<PlannedTransfer>,
    pub target_count: usize,
    pub duplicate_count: usize,
    pub error_count: usize,
    pub skip_count: usize,
}

/// One duplicate set for planning.
#[derive(Debug, Clone)]
pub struct DupSetInput {
    pub best_file: String,
    pub duplicates: Vec<String>,
}

/// Plan transfer destinations without touching the filesystem.
///
/// Rules (TS `transferOrganizedFiles` shape):
/// - unique files → target (relative_key = basename)
/// - duplicate non-best members → duplicate when `has_duplicate_dir`, else skip
/// - error files → error when `has_error_dir`, else skip
/// - best files that also appear in uniqueFiles are only planned once as target
pub fn plan_transfer_destinations(
    unique_files: &[String],
    duplicate_sets: &[DupSetInput],
    error_files: &[String],
    has_duplicate_dir: bool,
    has_error_dir: bool,
) -> TransferPlan {
    let mut actions = Vec::new();
    let mut seen = std::collections::BTreeSet::new();

    for path in unique_files {
        if !seen.insert(path.clone()) {
            continue;
        }
        actions.push(PlannedTransfer {
            source_path: path.clone(),
            bucket: "target".into(),
            relative_key: basename(path),
        });
    }

    for set in duplicate_sets {
        let folder = basename_stem(&set.best_file);
        for dup in &set.duplicates {
            if seen.contains(dup) {
                continue;
            }
            seen.insert(dup.clone());
            if has_duplicate_dir {
                actions.push(PlannedTransfer {
                    source_path: dup.clone(),
                    bucket: "duplicate".into(),
                    relative_key: format!("{folder}/{}", basename(dup)),
                });
            } else {
                actions.push(PlannedTransfer {
                    source_path: dup.clone(),
                    bucket: "skip".into(),
                    relative_key: String::new(),
                });
            }
        }
    }

    for path in error_files {
        if seen.contains(path) {
            continue;
        }
        seen.insert(path.clone());
        if has_error_dir {
            actions.push(PlannedTransfer {
                source_path: path.clone(),
                bucket: "error".into(),
                relative_key: basename(path),
            });
        } else {
            actions.push(PlannedTransfer {
                source_path: path.clone(),
                bucket: "skip".into(),
                relative_key: String::new(),
            });
        }
    }

    actions.sort_by(|a, b| a.source_path.cmp(&b.source_path));

    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for a in &actions {
        *counts.entry(a.bucket.as_str()).or_default() += 1;
    }

    TransferPlan {
        target_count: *counts.get("target").unwrap_or(&0),
        duplicate_count: *counts.get("duplicate").unwrap_or(&0),
        error_count: *counts.get("error").unwrap_or(&0),
        skip_count: *counts.get("skip").unwrap_or(&0),
        actions,
    }
}

fn basename(path: &str) -> String {
    path.rsplit(['/', '\\']).next().unwrap_or(path).to_string()
}

fn basename_stem(path: &str) -> String {
    let base = basename(path);
    match base.rfind('.') {
        Some(i) if i > 0 => base[..i].to_string(),
        _ => base,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plans_unique_and_duplicates() {
        let plan = plan_transfer_destinations(
            &["/in/a.jpg".into(), "/in/b.jpg".into()],
            &[DupSetInput {
                best_file: "/in/a.jpg".into(),
                duplicates: vec!["/in/a-copy.jpg".into()],
            }],
            &["/in/bad.dat".into()],
            true,
            true,
        );
        assert_eq!(plan.target_count, 2);
        assert_eq!(plan.duplicate_count, 1);
        assert_eq!(plan.error_count, 1);
        let dup = plan
            .actions
            .iter()
            .find(|a| a.source_path == "/in/a-copy.jpg")
            .unwrap();
        assert_eq!(dup.bucket, "duplicate");
        assert_eq!(dup.relative_key, "a/a-copy.jpg");
    }

    #[test]
    fn skips_when_dirs_absent() {
        let plan = plan_transfer_destinations(
            &[],
            &[DupSetInput {
                best_file: "best.png".into(),
                duplicates: vec!["dup.png".into()],
            }],
            &["err.bin".into()],
            false,
            false,
        );
        assert_eq!(plan.skip_count, 2);
        assert_eq!(plan.duplicate_count, 0);
        assert_eq!(plan.error_count, 0);
    }
}
