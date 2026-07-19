//! Pure cache key / DB naming planners for cli/cache-persistence residual.
//!
//! Ports LMDB sub-db naming + mutex keys + pHash LSH band split from
//! `src/caching/LmdbCache.ts` / `MetadataDBService.generateLshKeys` without
//! opening LMDB/SQLite. dens ≠ flip.

use serde::Serialize;

/// Default root cache directory name.
pub const DEFAULT_CACHE_DIR: &str = ".mediadb";

/// Default SQLite metadata filename.
pub const DEFAULT_METADATA_FILENAME: &str = "metadata.sqlite";

/// Serialization markers (LmdbCache.serialize).
pub const SERIALIZE_MARKER_MSGPACK: u8 = 0;
pub const SERIALIZE_MARKER_SHARED_ARRAY_BUFFER: u8 = 1;
pub const SERIALIZE_MARKER_DATE: u8 = 2;

/// Results sub-database name for a job.
#[must_use]
pub fn job_results_db_name(job_name: &str) -> String {
    format!("{job_name}_results")
}

/// Config sub-database name for a job.
#[must_use]
pub fn job_config_db_name(job_name: &str) -> String {
    format!("{job_name}_config")
}

/// Per-key mutex id: `{jobName}:{hashKey}`.
#[must_use]
pub fn cache_mutex_key(job_name: &str, hash_key: &str) -> String {
    format!("{job_name}:{hash_key}")
}

/// Join cache directory + metadata filename.
#[must_use]
pub fn metadata_db_path(db_directory: &str, db_filename: &str) -> String {
    let dir = db_directory.trim_end_matches(['/', '\\']);
    format!("{dir}/{db_filename}")
}

/// LSH band keys from 16-char hex pHash (MetadataDBService.generateLshKeys).
///
/// Invalid length → four nulls represented as None.
#[must_use]
pub fn metadata_lsh_keys(phash_hex: Option<&str>) -> [Option<String>; 4] {
    match phash_hex {
        Some(h) if h.len() == 16 && h.chars().all(|c| c.is_ascii_hexdigit()) => [
            Some(h[0..4].to_string()),
            Some(h[4..8].to_string()),
            Some(h[8..12].to_string()),
            Some(h[12..16].to_string()),
        ],
        _ => [None, None, None, None],
    }
}

/// Known serialize marker.
#[must_use]
pub fn is_serialize_marker(marker: u8) -> bool {
    matches!(
        marker,
        SERIALIZE_MARKER_MSGPACK | SERIALIZE_MARKER_SHARED_ARRAY_BUFFER | SERIALIZE_MARKER_DATE
    )
}

/// Pure plan describing cache open layout (no IO).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CacheLayoutPlan {
    pub root_dir: String,
    pub metadata_path: String,
    pub results_db: String,
    pub config_db: String,
    pub mutex_key: String,
}

/// Plan default cache layout for a job + hash key.
#[must_use]
pub fn plan_default_cache_layout(job_name: &str, hash_key: &str) -> CacheLayoutPlan {
    CacheLayoutPlan {
        root_dir: DEFAULT_CACHE_DIR.into(),
        metadata_path: metadata_db_path(DEFAULT_CACHE_DIR, DEFAULT_METADATA_FILENAME),
        results_db: job_results_db_name(job_name),
        config_db: job_config_db_name(job_name),
        mutex_key: cache_mutex_key(job_name, hash_key),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_and_mutex() {
        assert_eq!(job_results_db_name("fileStats"), "fileStats_results");
        assert_eq!(job_config_db_name("fileStats"), "fileStats_config");
        assert_eq!(cache_mutex_key("fileStats", "abc"), "fileStats:abc");
        assert_eq!(
            metadata_db_path(".mediadb", "metadata.sqlite"),
            ".mediadb/metadata.sqlite"
        );
    }

    #[test]
    fn lsh_keys() {
        let keys = metadata_lsh_keys(Some("0123456789abcdef"));
        assert_eq!(keys[0].as_deref(), Some("0123"));
        assert_eq!(keys[3].as_deref(), Some("cdef"));
        assert_eq!(metadata_lsh_keys(Some("short")), [None, None, None, None]);
        assert_eq!(metadata_lsh_keys(None), [None, None, None, None]);
    }

    #[test]
    fn layout_plan() {
        let plan = plan_default_cache_layout("phash", "k1");
        assert_eq!(plan.results_db, "phash_results");
        assert_eq!(plan.mutex_key, "phash:k1");
        assert!(is_serialize_marker(0));
        assert!(is_serialize_marker(2));
        assert!(!is_serialize_marker(9));
    }
}
