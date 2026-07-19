//! Deduplication pure cores (TS `deduplicateFilesFn` + `comparatorUtils` residual).
//!
//! - Exact pHash clustering (step 1)
//! - LSH key generation from 64-bit hex pHash
//! - Adaptive similarity thresholds by media type
//! - Overlapping-cluster merge (post-DBSCAN)
//!
//! Full pipeline (DB LSH query, transfer) remains residual until subsequent slices.

use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, HashMap};

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

/// Generate four LSH band keys from a 16-char (64-bit) hex pHash.
///
/// Mirrors TS `deduplicateFilesFn` `generateLshKeys`: invalid/non-16 length → None.
#[must_use]
pub fn generate_lsh_keys(phash_hex: &str) -> Option<[String; 4]> {
    let h = phash_hex.trim();
    if h.len() != 16 {
        return None;
    }
    if !h.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    Some([
        h[0..4].to_string(),
        h[4..8].to_string(),
        h[8..12].to_string(),
        h[12..16].to_string(),
    ])
}

/// Adaptive similarity threshold by media duration (0 = image).
///
/// Mirrors TS `getAdaptiveThreshold`: image-image / image-video / video-video.
#[must_use]
pub fn adaptive_threshold(
    duration1: f64,
    duration2: f64,
    image_similarity_threshold: f64,
    image_video_similarity_threshold: f64,
    video_similarity_threshold: f64,
) -> f64 {
    let is_image1 = duration1 == 0.0;
    let is_image2 = duration2 == 0.0;
    if is_image1 && is_image2 {
        image_similarity_threshold
    } else if is_image1 || is_image2 {
        image_video_similarity_threshold
    } else {
        video_similarity_threshold
    }
}

/// Merge overlapping clusters (TS `mergeAndDeduplicateClusters`).
///
/// Union-find over cluster membership produces the same partition as the TS
/// element→cluster map merge. Output is sorted for dual-oracle stability.
pub fn merge_and_deduplicate_clusters(clusters: &[Vec<String>]) -> Vec<Vec<String>> {
    let mut parent: HashMap<String, String> = HashMap::new();

    fn find(parent: &mut HashMap<String, String>, x: &str) -> String {
        let p = parent.get(x).cloned().unwrap_or_else(|| x.to_string());
        if p == x {
            return p;
        }
        let root = find(parent, &p);
        parent.insert(x.to_string(), root.clone());
        root
    }

    fn union(parent: &mut HashMap<String, String>, a: &str, b: &str) {
        let ra = find(parent, a);
        let rb = find(parent, b);
        if ra != rb {
            // Prefer lexicographically smaller root for determinism.
            if ra < rb {
                parent.insert(rb, ra);
            } else {
                parent.insert(ra, rb);
            }
        }
    }

    for cluster in clusters {
        if cluster.is_empty() {
            continue;
        }
        for element in cluster {
            parent
                .entry(element.clone())
                .or_insert_with(|| element.clone());
        }
        let first = &cluster[0];
        for element in cluster.iter().skip(1) {
            union(&mut parent, first, element);
        }
    }

    let mut groups: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let keys: Vec<String> = parent.keys().cloned().collect();
    for element in keys {
        let root = find(&mut parent, &element);
        groups.entry(root).or_default().insert(element);
    }

    let mut out: Vec<Vec<String>> = groups
        .into_values()
        .map(|set| set.into_iter().collect())
        .collect();
    out.sort();
    out
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

    #[test]
    fn lsh_keys_from_16_hex() {
        let keys = generate_lsh_keys("0123456789abcdef").unwrap();
        assert_eq!(keys, ["0123", "4567", "89ab", "cdef"]);
        assert!(generate_lsh_keys("short").is_none());
        assert!(generate_lsh_keys("0123456789abcdeg").is_none()); // non-hex g
    }

    #[test]
    fn adaptive_threshold_by_media_type() {
        assert_eq!(adaptive_threshold(0.0, 0.0, 0.9, 0.8, 0.7), 0.9);
        assert_eq!(adaptive_threshold(0.0, 5.0, 0.9, 0.8, 0.7), 0.8);
        assert_eq!(adaptive_threshold(3.0, 5.0, 0.9, 0.8, 0.7), 0.7);
    }

    #[test]
    fn merge_overlapping_clusters() {
        let clusters = vec![
            vec!["a".into(), "b".into()],
            vec!["b".into(), "c".into()],
            vec!["d".into()],
        ];
        let merged = merge_and_deduplicate_clusters(&clusters);
        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0], vec!["a", "b", "c"]);
        assert_eq!(merged[1], vec!["d"]);
    }

    #[test]
    fn merge_disjoint_clusters() {
        let clusters = vec![vec!["a".into()], vec!["b".into()]];
        let merged = merge_and_deduplicate_clusters(&clusters);
        assert_eq!(merged, vec![vec!["a"], vec!["b"]]);
    }
}
