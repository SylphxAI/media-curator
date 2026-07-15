//! Exact-duplicate clustering by perceptual hash (TS `deduplicateFilesFn` step 1 oracle).
//!
//! Groups paths that share an identical pHash hex. Empty/missing hashes are treated as
//! singleton candidates (not exact-dup clusters). Similarity/LSH remains a residual slice.

use serde::Serialize;
use std::collections::BTreeMap;

/// One exact-duplicate cluster (identical pHash).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExactDupCluster {
    pub phash_hex: String,
    pub paths: Vec<String>,
}

/// Result of exact pHash clustering.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExactDupResult {
    pub clusters: Vec<ExactDupCluster>,
    pub singletons: Vec<String>,
    pub missing_phash: Vec<String>,
}

/// Cluster files by identical pHash hex.
///
/// `entries` is (path, optional pHash hex). Paths with missing/empty pHash go to
/// `missing_phash` and are excluded from exact clusters. Groups of size >= 2 become
/// clusters; size 1 become singletons (similarity candidates in the full pipeline).
pub fn cluster_exact_by_phash(entries: &[(String, Option<String>)]) -> ExactDupResult {
    let mut by_hash: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut missing_phash = Vec::new();

    for (path, phash) in entries {
        match phash {
            Some(h) if !h.is_empty() => {
                by_hash.entry(h.clone()).or_default().push(path.clone());
            }
            _ => missing_phash.push(path.clone()),
        }
    }

    let mut clusters = Vec::new();
    let mut singletons = Vec::new();
    for (phash_hex, mut paths) in by_hash {
        paths.sort();
        if paths.len() >= 2 {
            clusters.push(ExactDupCluster { phash_hex, paths });
        } else if let Some(path) = paths.into_iter().next() {
            singletons.push(path);
        }
    }
    clusters.sort_by(|a, b| a.phash_hex.cmp(&b.phash_hex));
    singletons.sort();
    missing_phash.sort();

    ExactDupResult {
        clusters,
        singletons,
        missing_phash,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_dup_clusters_identical_phash() {
        let entries = vec![
            ("a.jpg".into(), Some("aabb".into())),
            ("b.jpg".into(), Some("aabb".into())),
            ("c.jpg".into(), Some("ccdd".into())),
            ("d.jpg".into(), None),
        ];
        let r = cluster_exact_by_phash(&entries);
        assert_eq!(r.clusters.len(), 1);
        assert_eq!(r.clusters[0].phash_hex, "aabb");
        assert_eq!(r.clusters[0].paths, vec!["a.jpg", "b.jpg"]);
        assert_eq!(r.singletons, vec!["c.jpg"]);
        assert_eq!(r.missing_phash, vec!["d.jpg"]);
    }

    #[test]
    fn empty_input_yields_empty_result() {
        let r = cluster_exact_by_phash(&[]);
        assert!(r.clusters.is_empty());
        assert!(r.singletons.is_empty());
        assert!(r.missing_phash.is_empty());
    }
}
