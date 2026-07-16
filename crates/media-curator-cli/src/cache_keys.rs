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
        SERIALIZE_MARKER_MSGPACK
            | SERIALIZE_MARKER_SHARED_ARRAY_BUFFER
            | SERIALIZE_MARKER_DATE
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

// ── wave68 pure residual dens: cache key layout residual dual-oracle ──
// Dual-oracle residual of LmdbCache / MetadataDB naming pure halves.
// LMDB/SQLite open I/O residual retained. dens ≠ flip.

/// Dual-oracle residual: default cache dir shell.
#[must_use]
pub fn default_cache_dir_shell() -> &'static str {
    DEFAULT_CACHE_DIR
}

/// Dual-oracle residual: default metadata filename shell.
#[must_use]
pub fn default_metadata_filename_shell() -> &'static str {
    DEFAULT_METADATA_FILENAME
}

/// Dual-oracle residual: serialize markers closed set.
#[must_use]
pub fn serialize_markers_closed() -> bool {
    is_serialize_marker(SERIALIZE_MARKER_MSGPACK)
        && is_serialize_marker(SERIALIZE_MARKER_SHARED_ARRAY_BUFFER)
        && is_serialize_marker(SERIALIZE_MARKER_DATE)
        && !is_serialize_marker(3)
        && !is_serialize_marker(255)
}

/// Dual-oracle residual: default layout plan shells.
#[must_use]
pub fn default_layout_shell_ok() -> bool {
    let plan = plan_default_cache_layout("fileStats", "abc");
    plan.root_dir == DEFAULT_CACHE_DIR
        && plan.results_db == "fileStats_results"
        && plan.config_db == "fileStats_config"
        && plan.mutex_key == "fileStats:abc"
        && plan.metadata_path == ".mediadb/metadata.sqlite"
}

/// Dual-oracle residual: LSH keys align with 4 bands.
#[must_use]
pub fn metadata_lsh_four_bands_ok() -> bool {
    let keys = metadata_lsh_keys(Some("0123456789abcdef"));
    keys.iter().all(|k| k.is_some())
        && keys[0].as_deref() == Some("0123")
        && keys[3].as_deref() == Some("cdef")
}

#[cfg(test)]
mod wave68_tests {
    use super::*;

    #[test]
    fn wave68_cache_key_layout_dual_oracle() {
        assert_eq!(default_cache_dir_shell(), ".mediadb");
        assert_eq!(default_metadata_filename_shell(), "metadata.sqlite");
        assert!(serialize_markers_closed());
        assert!(default_layout_shell_ok());
        assert!(metadata_lsh_four_bands_ok());
        assert_eq!(SERIALIZE_MARKER_MSGPACK, 0);
        assert_eq!(SERIALIZE_MARKER_SHARED_ARRAY_BUFFER, 1);
        assert_eq!(SERIALIZE_MARKER_DATE, 2);
        assert_eq!(
            metadata_db_path("/tmp/cache/", "meta.db"),
            "/tmp/cache/meta.db"
        );
        assert_eq!(job_results_db_name("phash"), "phash_results");
    }
}



// ── wave71 pure residual dens: cache LSH invalid + naming dual-oracle residual ──
// Dual-oracle residual of LmdbCache / MetadataDB naming pure halves.
// LMDB/SQLite open I/O residual retained. dens ≠ flip.
// product residual dens wave71

/// Dual-oracle residual: invalid LSH inputs yield four Nones.
#[must_use]
pub fn lsh_invalid_all_none() -> bool {
    metadata_lsh_keys(None) == [None, None, None, None]
        && metadata_lsh_keys(Some("short")) == [None, None, None, None]
        && metadata_lsh_keys(Some("0123456789abcdeg")) == [None, None, None, None]
}

/// Dual-oracle residual: job naming suffix ladder.
#[must_use]
pub fn job_db_name_suffix_shell() -> bool {
    job_results_db_name("phash") == "phash_results"
        && job_config_db_name("phash") == "phash_config"
        && cache_mutex_key("phash", "k1") == "phash:k1"
}

/// Dual-oracle residual: path join trims trailing slash.
#[must_use]
pub fn metadata_path_trim_shell() -> bool {
    metadata_db_path("/tmp/cache/", "meta.db") == "/tmp/cache/meta.db"
        && metadata_db_path("/tmp/cache", "meta.db") == "/tmp/cache/meta.db"
}

/// Dual-oracle residual: serialize marker ladder 0/1/2/3.
#[must_use]
pub fn serialize_marker_ladder() -> [bool; 4] {
    [
        is_serialize_marker(0),
        is_serialize_marker(1),
        is_serialize_marker(2),
        is_serialize_marker(3),
    ]
}

/// Dual-oracle residual: default layout for phash/k1.
#[must_use]
pub fn phash_default_layout_shell() -> bool {
    let plan = plan_default_cache_layout("phash", "k1");
    plan.root_dir == ".mediadb"
        && plan.results_db == "phash_results"
        && plan.config_db == "phash_config"
        && plan.mutex_key == "phash:k1"
        && plan.metadata_path == ".mediadb/metadata.sqlite"
}

#[cfg(test)]
mod wave71_tests {
    use super::*;

    #[test]
    fn wave71_cache_lsh_naming_dual_oracle() {
        assert!(lsh_invalid_all_none());
        assert!(job_db_name_suffix_shell());
        assert!(metadata_path_trim_shell());
        assert_eq!(serialize_marker_ladder(), [true, true, true, false]);
        assert!(phash_default_layout_shell());
        assert!(metadata_lsh_four_bands_ok());
        assert_eq!(DEFAULT_CACHE_DIR, ".mediadb");
    }
}

// ── wave75 pure residual dens: cache naming + LSH bands dual-oracle residual ──
// Dual-oracle residual of LmdbCache / MetadataDB naming pure halves.
// LMDB/SQLite open I/O residual retained. dens ≠ flip.

/// Dual-oracle residual: default layout constants.
#[must_use]
pub fn wave75_default_layout_shell() -> bool {
    DEFAULT_CACHE_DIR == ".mediadb"
        && DEFAULT_METADATA_FILENAME == "metadata.sqlite"
        && SERIALIZE_MARKER_MSGPACK == 0
        && SERIALIZE_MARKER_SHARED_ARRAY_BUFFER == 1
        && SERIALIZE_MARKER_DATE == 2
        && serialize_markers_closed()
}

/// Dual-oracle residual: job db names + mutex key.
#[must_use]
pub fn wave75_job_naming_shell() -> bool {
    job_results_db_name("phash") == "phash_results"
        && job_config_db_name("fileStats") == "fileStats_config"
        && cache_mutex_key("phash", "abc") == "phash:abc"
}

/// Dual-oracle residual: metadata LSH four bands valid hex.
#[must_use]
pub fn wave75_metadata_lsh_shell() -> bool {
    let keys = metadata_lsh_keys(Some("0123456789abcdef"));
    keys == [
        Some("0123".to_string()),
        Some("4567".to_string()),
        Some("89ab".to_string()),
        Some("cdef".to_string()),
    ] && lsh_invalid_all_none()
}

/// Dual-oracle residual: path join + default plan.
#[must_use]
pub fn wave75_path_and_plan_shell() -> bool {
    metadata_db_path("/data/", "m.db") == "/data/m.db"
        && metadata_db_path("/data", "m.db") == "/data/m.db"
        && phash_default_layout_shell()
}

#[cfg(test)]
mod wave75_tests {
    use super::*;

    #[test]
    fn wave75_cache_naming_lsh_bands_dual_oracle() {
        assert!(wave75_default_layout_shell());
        assert!(wave75_job_naming_shell());
        assert!(wave75_metadata_lsh_shell());
        assert!(wave75_path_and_plan_shell());
        assert_eq!(serialize_marker_ladder(), [true, true, true, false]);
        assert_eq!(default_cache_dir_shell(), ".mediadb");
    }
}
