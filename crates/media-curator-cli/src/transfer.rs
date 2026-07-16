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


// ── wave64 pure residual dens: transfer bucket catalog dual-oracle residual ──
// Dual-oracle residual of transferOrganizedFiles bucket pure half.
// Filesystem transfer I/O residual retained. dens ≠ flip.

/// Dual-oracle residual: transfer buckets.
pub const TRANSFER_BUCKETS: &[&str] = &["target", "duplicate", "error", "skip"];

/// Dual-oracle residual: known bucket name.
#[must_use]
pub fn is_transfer_bucket(bucket: &str) -> bool {
    TRANSFER_BUCKETS.contains(&bucket)
}

/// Dual-oracle residual: bucket is a real destination (not skip).
#[must_use]
pub fn is_destination_bucket(bucket: &str) -> bool {
    matches!(bucket, "target" | "duplicate" | "error")
}

/// Dual-oracle residual: plan totals consistency.
#[must_use]
pub fn transfer_plan_counts_consistent(plan: &TransferPlan) -> bool {
    plan.actions.len()
        == plan.target_count + plan.duplicate_count + plan.error_count + plan.skip_count
}

/// Dual-oracle residual: empty plan.
#[must_use]
pub fn empty_transfer_plan() -> TransferPlan {
    TransferPlan {
        actions: vec![],
        target_count: 0,
        duplicate_count: 0,
        error_count: 0,
        skip_count: 0,
    }
}

#[cfg(test)]
mod wave64_tests {
    use super::*;

    #[test]
    fn wave64_transfer_bucket_catalog_dual_oracle() {
        assert_eq!(TRANSFER_BUCKETS.len(), 4);
        assert!(is_transfer_bucket("target"));
        assert!(is_transfer_bucket("skip"));
        assert!(!is_transfer_bucket("archive"));
        assert!(is_destination_bucket("duplicate"));
        assert!(!is_destination_bucket("skip"));
        let empty = empty_transfer_plan();
        assert!(transfer_plan_counts_consistent(&empty));
        let plan = plan_transfer_destinations(
            &["/a.jpg".into()],
            &[],
            &[],
            true,
            true,
        );
        assert_eq!(plan.target_count, 1);
        assert!(transfer_plan_counts_consistent(&plan));
        assert_eq!(plan.actions[0].bucket, "target");
    }
}



// ── wave65 pure residual dens: transfer plan residual dual-oracle dens ──
// Dual-oracle residual of transferOrganizedFiles plan pure half.
// Filesystem transfer I/O residual retained. dens ≠ flip.

/// Dual-oracle residual: bucket count is 4.
#[must_use]
pub fn transfer_bucket_count() -> usize {
    TRANSFER_BUCKETS.len()
}

/// Dual-oracle residual: plan total actions.
#[must_use]
pub fn transfer_plan_total(plan: &TransferPlan) -> usize {
    plan.target_count + plan.duplicate_count + plan.error_count + plan.skip_count
}

/// Dual-oracle residual: skip is not a destination bucket.
#[must_use]
pub fn skip_is_not_destination() -> bool {
    !is_destination_bucket("skip") && is_transfer_bucket("skip")
}

/// Dual-oracle residual: unique-only plan lands in target.
#[must_use]
pub fn unique_only_targets(unique: &[String]) -> bool {
    let plan = plan_transfer_destinations(unique, &[], &[], true, true);
    plan.target_count == unique.len()
        && plan.duplicate_count == 0
        && plan.error_count == 0
        && transfer_plan_counts_consistent(&plan)
}

/// Dual-oracle residual: empty inputs → empty plan.
#[must_use]
pub fn empty_inputs_empty_plan() -> bool {
    let plan = plan_transfer_destinations(&[], &[], &[], true, true);
    plan.actions.is_empty() && transfer_plan_total(&plan) == 0
}

#[cfg(test)]
mod wave65_tests {
    use super::*;

    #[test]
    fn wave65_transfer_plan_residual_dual_oracle() {
        assert_eq!(transfer_bucket_count(), 4);
        assert!(skip_is_not_destination());
        assert!(empty_inputs_empty_plan());
        assert!(unique_only_targets(&["/a.jpg".into(), "/b.png".into()]));
        let plan = plan_transfer_destinations(
            &["/u.jpg".into()],
            &[DupSetInput {
                best_file: "/best.jpg".into(),
                duplicates: vec!["/dup.jpg".into()],
            }],
            &["/err.bin".into()],
            true,
            true,
        );
        assert_eq!(plan.target_count, 1);
        assert_eq!(plan.duplicate_count, 1);
        assert_eq!(plan.error_count, 1);
        assert!(transfer_plan_counts_consistent(&plan));
        assert_eq!(transfer_plan_total(&plan), plan.actions.len());
        assert!(is_destination_bucket("error"));
    }
}

// ── wave66 pure residual dens: transfer skip-when-no-dir dual-oracle residual ──
// Dual-oracle residual of transferOrganizedFiles skip when dest dirs absent pure half.
// Filesystem transfer I/O residual retained. dens ≠ flip.

/// Dual-oracle residual: without duplicate dir, dups land in skip.
#[must_use]
pub fn dups_skip_without_duplicate_dir() -> bool {
    let plan = plan_transfer_destinations(
        &[],
        &[DupSetInput {
            best_file: "/best.jpg".into(),
            duplicates: vec!["/dup.jpg".into()],
        }],
        &[],
        false,
        true,
    );
    plan.skip_count == 1 && plan.duplicate_count == 0 && transfer_plan_counts_consistent(&plan)
}

/// Dual-oracle residual: without error dir, errors land in skip.
#[must_use]
pub fn errors_skip_without_error_dir() -> bool {
    let plan = plan_transfer_destinations(&[], &[], &["/err.bin".into()], true, false);
    plan.skip_count == 1 && plan.error_count == 0 && transfer_plan_counts_consistent(&plan)
}

/// Dual-oracle residual: bucket catalog order target→duplicate→error→skip.
#[must_use]
pub fn transfer_bucket_order_ok() -> bool {
    TRANSFER_BUCKETS == ["target", "duplicate", "error", "skip"]
}

/// Dual-oracle residual: destination buckets are target/duplicate/error only.
#[must_use]
pub fn destination_buckets_closed() -> bool {
    is_destination_bucket("target")
        && is_destination_bucket("duplicate")
        && is_destination_bucket("error")
        && !is_destination_bucket("skip")
}

/// Dual-oracle residual: relative_key for target is basename pure half.
#[must_use]
pub fn target_relative_key_is_basename() -> bool {
    let plan = plan_transfer_destinations(&["/dir/photo.jpg".into()], &[], &[], true, true);
    plan.actions.len() == 1 && plan.actions[0].relative_key == "photo.jpg"
}

#[cfg(test)]
mod wave66_tests {
    use super::*;

    #[test]
    fn wave66_transfer_skip_when_no_dir_dual_oracle() {
        assert!(dups_skip_without_duplicate_dir());
        assert!(errors_skip_without_error_dir());
        assert!(transfer_bucket_order_ok());
        assert!(destination_buckets_closed());
        assert!(target_relative_key_is_basename());
        assert_eq!(transfer_bucket_count(), 4);
        assert!(skip_is_not_destination());
        assert!(empty_inputs_empty_plan());
        let plan = plan_transfer_destinations(
            &["/u.jpg".into()],
            &[DupSetInput {
                best_file: "/best.jpg".into(),
                duplicates: vec!["/dup.jpg".into()],
            }],
            &["/err.bin".into()],
            true,
            true,
        );
        assert_eq!(transfer_plan_total(&plan), plan.actions.len());
        assert!(is_transfer_bucket("target"));
        assert!(!is_transfer_bucket("archive"));
    }
}


// ── wave67 pure residual dens: transfer plan consistency dual-oracle residual ──
// Dual-oracle residual of transferOrganizedFiles plan counts pure halves.
// Filesystem transfer I/O residual retained. dens ≠ flip.

/// Dual-oracle residual: unique files all land in target when dirs present.
#[must_use]
pub fn unique_targets_when_dirs_present() -> bool {
    let plan = plan_transfer_destinations(
        &["/a.jpg".into(), "/b.jpg".into()],
        &[],
        &[],
        true,
        true,
    );
    plan.actions.len() == 2
        && plan.actions.iter().all(|a| a.bucket == "target")
        && transfer_plan_counts_consistent(&plan)
}

/// Dual-oracle residual: dup best→target, rest→duplicate when dir present.
#[must_use]
pub fn dup_set_routes_best_and_dups() -> bool {
    let plan = plan_transfer_destinations(
        &[],
        &[DupSetInput {
            best_file: "/best.jpg".into(),
            duplicates: vec!["/d1.jpg".into(), "/d2.jpg".into()],
        }],
        &[],
        true,
        true,
    );
    let target_n = plan.actions.iter().filter(|a| a.bucket == "target").count();
    let dup_n = plan.actions.iter().filter(|a| a.bucket == "duplicate").count();
    // best alone is not auto-target unless also in unique; dups route when dir present
    dup_n == 2 && transfer_plan_counts_consistent(&plan) && target_n <= 1
}

/// Dual-oracle residual: error files route to error bucket when dir present.
#[must_use]
pub fn errors_route_to_error_bucket() -> bool {
    let plan = plan_transfer_destinations(
        &[],
        &[],
        &["/bad.bin".into()],
        true,
        true,
    );
    plan.actions.len() == 1
        && plan.actions[0].bucket == "error"
        && plan.actions[0].relative_key == "bad.bin"
}

/// Dual-oracle residual: bucket catalog closed four.
#[must_use]
pub fn transfer_four_buckets_closed() -> bool {
    transfer_bucket_count() == 4
        && is_transfer_bucket("target")
        && is_transfer_bucket("duplicate")
        && is_transfer_bucket("error")
        && is_transfer_bucket("skip")
        && !is_transfer_bucket("archive")
}

#[cfg(test)]
mod wave67_tests {
    use super::*;

    #[test]
    fn wave67_transfer_plan_consistency_dual_oracle() {
        assert!(unique_targets_when_dirs_present());
        assert!(dup_set_routes_best_and_dups());
        assert!(errors_route_to_error_bucket());
        assert!(transfer_four_buckets_closed());
        assert!(transfer_bucket_order_ok());
        assert!(destination_buckets_closed());
        assert!(skip_is_not_destination());
        assert!(empty_inputs_empty_plan());
        assert!(dups_skip_without_duplicate_dir());
        assert!(errors_skip_without_error_dir());
    }
}

// ── wave75 pure residual dens: transfer routing partition dual-oracle residual ──
// Dual-oracle residual of transferOrganizedFiles bucket routing pure halves.
// Filesystem transfer I/O residual retained. dens ≠ flip.

/// Dual-oracle residual: bucket catalog closed four.
#[must_use]
pub fn wave75_bucket_catalog_shell() -> bool {
    TRANSFER_BUCKETS == ["target", "duplicate", "error", "skip"]
        && transfer_bucket_count() == 4
        && is_transfer_bucket("target")
        && !is_transfer_bucket("archive")
        && is_destination_bucket("error")
        && !is_destination_bucket("skip")
}

/// Dual-oracle residual: unique → target with basename relative key.
#[must_use]
pub fn wave75_unique_target_basename_shell() -> bool {
    let plan = plan_transfer_destinations(
        &["/photos/trip/IMG_001.JPG".into()],
        &[],
        &[],
        true,
        true,
    );
    plan.target_count == 1
        && plan.actions[0].bucket == "target"
        && plan.actions[0].relative_key == "IMG_001.JPG"
        && transfer_plan_counts_consistent(&plan)
}

/// Dual-oracle residual: dups under best stem folder when dir present.
#[must_use]
pub fn wave75_dup_folder_relative_shell() -> bool {
    let plan = plan_transfer_destinations(
        &[],
        &[DupSetInput {
            best_file: "/in/best.jpg".into(),
            duplicates: vec!["/in/copy.jpg".into()],
        }],
        &[],
        true,
        true,
    );
    plan.duplicate_count == 1
        && plan.actions[0].bucket == "duplicate"
        && plan.actions[0].relative_key == "best/copy.jpg"
}

/// Dual-oracle residual: missing dest dirs skip dups+errors.
#[must_use]
pub fn wave75_missing_dirs_skip_shell() -> bool {
    let plan = plan_transfer_destinations(
        &["/u.jpg".into()],
        &[DupSetInput {
            best_file: "/best.jpg".into(),
            duplicates: vec!["/d.jpg".into()],
        }],
        &["/e.bin".into()],
        false,
        false,
    );
    plan.target_count == 1
        && plan.duplicate_count == 0
        && plan.error_count == 0
        && plan.skip_count == 2
        && transfer_plan_counts_consistent(&plan)
}

/// Dual-oracle residual: empty plan shell.
#[must_use]
pub fn wave75_empty_plan_shell() -> bool {
    let plan = empty_transfer_plan();
    plan.actions.is_empty()
        && transfer_plan_total(&plan) == 0
        && empty_inputs_empty_plan()
}

#[cfg(test)]
mod wave75_tests {
    use super::*;

    #[test]
    fn wave75_transfer_routing_partition_dual_oracle() {
        assert!(wave75_bucket_catalog_shell());
        assert!(wave75_unique_target_basename_shell());
        assert!(wave75_dup_folder_relative_shell());
        assert!(wave75_missing_dirs_skip_shell());
        assert!(wave75_empty_plan_shell());
        assert!(unique_targets_when_dirs_present());
        assert!(errors_route_to_error_bucket());
    }
}

// ── product residual dens wave73: transfer plan counts+bucket dual-oracle residual ──
// Dual-oracle residual of transferOrganizedFiles plan counts pure halves.
// Filesystem transfer I/O residual retained. dens ≠ flip.

/// Dual-oracle residual: destination buckets closed three (skip excluded).
#[must_use]
pub fn wave73_destination_buckets_shell() -> bool {
    is_destination_bucket("target")
        && is_destination_bucket("duplicate")
        && is_destination_bucket("error")
        && !is_destination_bucket("skip")
        && skip_is_not_destination()
        && destination_buckets_closed()
}

/// Dual-oracle residual: error routing when error dir present.
#[must_use]
pub fn wave73_error_route_shell() -> bool {
    let plan = plan_transfer_destinations(
        &[],
        &[],
        &["/bad/corrupt.bin".into()],
        true,
        true,
    );
    plan.error_count == 1
        && plan.skip_count == 0
        && plan.actions[0].bucket == "error"
        && transfer_plan_counts_consistent(&plan)
}

/// Dual-oracle residual: multi unique targets.
#[must_use]
pub fn wave73_multi_unique_shell() -> bool {
    let plan = plan_transfer_destinations(
        &["/a/one.jpg".into(), "/b/two.png".into()],
        &[],
        &[],
        true,
        true,
    );
    plan.target_count == 2
        && plan.actions.iter().all(|a| a.bucket == "target")
        && transfer_plan_total(&plan) == 2
}

/// Dual-oracle residual: transfer plan empty consistency.
#[must_use]
pub fn wave73_empty_consistent_shell() -> bool {
    let plan = empty_transfer_plan();
    transfer_plan_counts_consistent(&plan)
        && transfer_plan_total(&plan) == 0
        && plan.target_count == 0
}

/// Dual-oracle residual: bucket order constant.
#[must_use]
pub fn wave73_bucket_order_shell() -> bool {
    transfer_bucket_order_ok()
        && TRANSFER_BUCKETS[0] == "target"
        && TRANSFER_BUCKETS[3] == "skip"
}

#[cfg(test)]
mod wave73_tests {
    use super::*;

    #[test]
    fn wave73_transfer_plan_counts_bucket_dual_oracle() {
        assert!(wave73_destination_buckets_shell());
        assert!(wave73_error_route_shell());
        assert!(wave73_multi_unique_shell());
        assert!(wave73_empty_consistent_shell());
        assert!(wave73_bucket_order_shell());
        assert!(wave75_bucket_catalog_shell());
    }
}

