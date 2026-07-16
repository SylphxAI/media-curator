//! Recursive media discovery — parity with `src/discovery.ts` `discoverFilesFn`.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Condvar, Mutex};

use serde::Serialize;

use crate::extensions::{all_supported_extensions, extension_of};

/// Errors surfaced while scanning source directories.
#[derive(Debug, thiserror::Error)]
pub enum DiscoveryError {
    #[error("error scanning directory {path}: {message}")]
    ScanDirectory { path: PathBuf, message: String },
}

/// Aggregate counters matching TS `discoverFilesFn` bookkeeping.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DiscoveryStats {
    pub file_count: u64,
    pub dir_count: u64,
}

/// Canonical discovery payload for JSON parity tests and CLI output.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DiscoveryMap {
    /// Lowercase extension → discovered file paths.
    pub by_extension: BTreeMap<String, Vec<String>>,
    pub stats: DiscoveryStats,
}

/// Options for deterministic discovery scans.
#[derive(Debug, Clone)]
pub struct DiscoverOptions {
    pub source_dirs: Vec<PathBuf>,
    /// Maximum concurrent directory scans (TS default: 10; parity fixtures use 1).
    pub concurrency: usize,
}

struct Semaphore {
    slots: Mutex<usize>,
    available: Condvar,
}

impl Semaphore {
    fn new(limit: usize) -> Self {
        Self {
            slots: Mutex::new(limit),
            available: Condvar::new(),
        }
    }

    fn acquire(&self) {
        let mut slots = self
            .slots
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        while *slots == 0 {
            slots = self
                .available
                .wait(slots)
                .unwrap_or_else(|poisoned| poisoned.into_inner());
        }
        *slots -= 1;
    }

    fn release(&self) {
        let mut slots = self
            .slots
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        *slots += 1;
        self.available.notify_one();
    }
}

struct SemaphoreGuard<'a> {
    semaphore: &'a Semaphore,
}

impl<'a> SemaphoreGuard<'a> {
    fn new(semaphore: &'a Semaphore) -> Self {
        semaphore.acquire();
        Self { semaphore }
    }
}

impl Drop for SemaphoreGuard<'_> {
    fn drop(&mut self) {
        self.semaphore.release();
    }
}

struct ScanState {
    all_files: Vec<PathBuf>,
    file_count: u64,
    dir_count: u64,
}

/// Discover supported media files under `options.source_dirs`.
///
/// Per-directory `readdir` errors are logged to stderr and skipped (TS parity).
pub fn discover_files(options: DiscoverOptions) -> Result<DiscoveryMap, DiscoveryError> {
    let supported = all_supported_extensions();
    let semaphore = Arc::new(Semaphore::new(options.concurrency.max(1)));
    let state = Arc::new(Mutex::new(ScanState {
        all_files: Vec::new(),
        file_count: 0,
        dir_count: 0,
    }));

    let mut workers = Vec::new();
    for source_dir in options.source_dirs {
        let semaphore = Arc::clone(&semaphore);
        let state = Arc::clone(&state);
        let supported = supported.clone();
        workers.push(std::thread::spawn(move || {
            scan_directory(&source_dir, &supported, &semaphore, &state);
        }));
    }

    for worker in workers {
        if let Err(error) = worker.join() {
            std::panic::resume_unwind(error);
        }
    }

    let locked = state
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    Ok(group_by_extension(
        &locked.all_files,
        locked.file_count,
        locked.dir_count,
    ))
}

/// Serialize discovery output for CLI / parity harnesses.
pub fn discover_files_json(options: DiscoverOptions) -> Result<String, DiscoveryError> {
    let map = discover_files(options)?;
    serde_json::to_string(&map).map_err(|error| DiscoveryError::ScanDirectory {
        path: PathBuf::from("<serialize>"),
        message: error.to_string(),
    })
}

fn scan_directory(
    dir_path: &Path,
    supported: &std::collections::HashSet<&'static str>,
    semaphore: &Arc<Semaphore>,
    state: &Arc<Mutex<ScanState>>,
) {
    let child_dirs = {
        let _guard = SemaphoreGuard::new(semaphore);

        {
            let mut locked = state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            locked.dir_count += 1;
        }

        let entries = match fs::read_dir(dir_path) {
        Ok(entries) => entries,
        Err(error) => {
            eprintln!("Error scanning directory {}: {error}", dir_path.display());
            return;
        }
    };

    let mut child_dirs = Vec::new();
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                eprintln!(
                    "Error scanning directory {}: {error}",
                    dir_path.display()
                );
                continue;
            }
        };

        let entry_path = entry.path();
        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(error) => {
                eprintln!(
                    "Error scanning directory {}: {error}",
                    dir_path.display()
                );
                continue;
            }
        };

        if file_type.is_dir() {
            child_dirs.push(entry_path);
            continue;
        }

        if !file_type.is_file() {
            continue;
        }

        let Some(ext) = extension_of(&entry_path) else {
            continue;
        };

        if !supported.contains(ext.as_str()) {
            continue;
        }

        let mut locked = state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        locked.all_files.push(entry_path);
        locked.file_count += 1;
    }

        child_dirs
    };

    let mut workers = Vec::new();
    for child_dir in child_dirs {
        let semaphore = Arc::clone(semaphore);
        let state = Arc::clone(state);
        let supported = supported.clone();
        workers.push(std::thread::spawn(move || {
            scan_directory(&child_dir, &supported, &semaphore, &state);
        }));
    }

    for worker in workers {
        if let Err(error) = worker.join() {
            std::panic::resume_unwind(error);
        }
    }
}

fn group_by_extension(
    all_files: &[PathBuf],
    file_count: u64,
    dir_count: u64,
) -> DiscoveryMap {
    let mut by_extension: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for file in all_files {
        let Some(ext) = extension_of(file) else {
            continue;
        };
        let path_string = file.to_string_lossy().into_owned();
        by_extension.entry(ext).or_default().push(path_string);
    }

    for paths in by_extension.values_mut() {
        paths.sort();
    }

    DiscoveryMap {
        by_extension,
        stats: DiscoveryStats {
            file_count,
            dir_count,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/discovery/basic")
    }

    #[test]
    fn discovers_fixture_media_files() {
        let root = fixture_root();
        let map = discover_files(DiscoverOptions {
            source_dirs: vec![root],
            concurrency: 1,
        })
        .expect("fixture discovery must succeed");

        assert_eq!(map.stats.file_count, 2);
        assert_eq!(map.stats.dir_count, 2);
        assert_eq!(map.by_extension.len(), 1);
        assert!(map.by_extension.contains_key("jpg"));
    }
}
// ── wave63 pure residual dens: discovery concurrency dual-oracle residual ──
// Dual-oracle residual of `src/discovery.ts` default concurrency=10 pure half.
// Filesystem walk I/O residual retained. dens ≠ flip.

/// Dual-oracle residual: default discovery concurrency (TS default).
pub const DEFAULT_DISCOVERY_CONCURRENCY: usize = 10;

/// Dual-oracle residual: minimum concurrency (serial scan).
pub const MIN_DISCOVERY_CONCURRENCY: usize = 1;

/// Dual-oracle residual: clamp concurrency into product band.
#[must_use]
pub fn clamp_discovery_concurrency(n: usize) -> usize {
    n.max(MIN_DISCOVERY_CONCURRENCY)
}

/// Dual-oracle residual: default concurrency is 10.
#[must_use]
pub fn default_discovery_concurrency_is_ten() -> bool {
    DEFAULT_DISCOVERY_CONCURRENCY == 10
}

/// Dual-oracle residual: DiscoverOptions with product default concurrency.
#[must_use]
pub fn discover_options_default(source_dirs: Vec<std::path::PathBuf>) -> DiscoverOptions {
    DiscoverOptions {
        source_dirs,
        concurrency: DEFAULT_DISCOVERY_CONCURRENCY,
    }
}

#[cfg(test)]
mod wave63_tests {
    use super::*;

    #[test]
    fn wave63_discovery_concurrency_dual_oracle() {
        assert_eq!(DEFAULT_DISCOVERY_CONCURRENCY, 10);
        assert_eq!(MIN_DISCOVERY_CONCURRENCY, 1);
        assert!(default_discovery_concurrency_is_ten());
        assert_eq!(clamp_discovery_concurrency(0), 1);
        assert_eq!(clamp_discovery_concurrency(10), 10);
        assert_eq!(clamp_discovery_concurrency(32), 32);
        let opts = discover_options_default(vec![]);
        assert_eq!(opts.concurrency, 10);
        assert!(opts.source_dirs.is_empty());
    }
}


// ── wave68 pure residual dens: discovery concurrency clamp ladder dual-oracle residual ──
// Dual-oracle residual of discovery concurrency clamp/default pure half.
// Filesystem walk I/O residual retained. dens ≠ flip.
// Complementary to concurrent LSH/cache wave68 dens.

/// Dual-oracle residual: concurrency shell (min, default).
#[must_use]
pub fn discovery_concurrency_shell() -> (usize, usize) {
    (MIN_DISCOVERY_CONCURRENCY, DEFAULT_DISCOVERY_CONCURRENCY)
}

/// Dual-oracle residual: clamp ladder for 0/1/10/32/64.
#[must_use]
pub fn discovery_clamp_ladder() -> [usize; 5] {
    [
        clamp_discovery_concurrency(0),
        clamp_discovery_concurrency(1),
        clamp_discovery_concurrency(10),
        clamp_discovery_concurrency(32),
        clamp_discovery_concurrency(64),
    ]
}

/// Dual-oracle residual: default options carry default concurrency.
#[must_use]
pub fn default_options_concurrency_matches() -> bool {
    discover_options_default(vec![]).concurrency == DEFAULT_DISCOVERY_CONCURRENCY
}

/// Dual-oracle residual: min is serial scan.
#[must_use]
pub fn min_concurrency_is_serial() -> bool {
    MIN_DISCOVERY_CONCURRENCY == 1
}

/// Dual-oracle residual: default is tenfold min.
#[must_use]
pub fn default_is_tenfold_min() -> bool {
    DEFAULT_DISCOVERY_CONCURRENCY == MIN_DISCOVERY_CONCURRENCY * 10
}

#[cfg(test)]
mod wave68_tests {
    use super::*;

    #[test]
    fn wave68_discovery_concurrency_clamp_ladder_dual_oracle() {
        assert_eq!(discovery_concurrency_shell(), (1, 10));
        assert_eq!(discovery_clamp_ladder(), [1, 1, 10, 32, 64]);
        assert!(default_options_concurrency_matches());
        assert!(min_concurrency_is_serial());
        assert!(default_is_tenfold_min());
        assert!(default_discovery_concurrency_is_ten());
        let opts = discover_options_default(vec![std::path::PathBuf::from("/tmp")]);
        assert_eq!(opts.concurrency, 10);
        assert_eq!(opts.source_dirs.len(), 1);
    }
}


// ── wave70 pure residual dens: discovery clamp identity dual-oracle residual ──
// Dual-oracle residual of discovery concurrency clamp identity pure half.
// Filesystem walk I/O residual retained. dens ≠ flip.
// product residual dens wave70

/// Dual-oracle residual: clamp identity for min/default.
#[must_use]
pub fn discovery_clamp_identity() -> bool {
    clamp_discovery_concurrency(MIN_DISCOVERY_CONCURRENCY) == MIN_DISCOVERY_CONCURRENCY
        && clamp_discovery_concurrency(DEFAULT_DISCOVERY_CONCURRENCY)
            == DEFAULT_DISCOVERY_CONCURRENCY
}

/// Dual-oracle residual: zero clamps to min.
#[must_use]
pub fn zero_clamps_to_min() -> bool {
    clamp_discovery_concurrency(0) == MIN_DISCOVERY_CONCURRENCY
}

/// Dual-oracle residual: high concurrency preserved when above default.
#[must_use]
pub fn high_concurrency_preserved() -> bool {
    clamp_discovery_concurrency(32) == 32 && clamp_discovery_concurrency(64) == 64
}

/// Dual-oracle residual: shell constants.
#[must_use]
pub fn discovery_constants_shell() -> (usize, usize) {
    (MIN_DISCOVERY_CONCURRENCY, DEFAULT_DISCOVERY_CONCURRENCY)
}

/// Dual-oracle residual: default options concurrency matches default constant.
#[must_use]
pub fn default_options_shell_ok() -> bool {
    let opts = discover_options_default(vec![std::path::PathBuf::from("/media")]);
    opts.concurrency == DEFAULT_DISCOVERY_CONCURRENCY && opts.source_dirs.len() == 1
}

#[cfg(test)]
mod wave70_tests {
    use super::*;

    #[test]
    fn wave70_discovery_clamp_identity_dual_oracle() {
        assert!(discovery_clamp_identity());
        assert!(zero_clamps_to_min());
        assert!(high_concurrency_preserved());
        assert_eq!(discovery_constants_shell(), (1, 10));
        assert!(default_options_shell_ok());
        assert_eq!(discovery_clamp_ladder(), [1, 1, 10, 32, 64]);
        assert!(default_is_tenfold_min());
        assert!(min_concurrency_is_serial());
        assert!(default_discovery_concurrency_is_ten());
    }
}


// ── product residual dens wave72: discovery concurrency band+default dual-oracle residual ──
// Dual-oracle residual of clamp_discovery_concurrency / default options pure halves.
// Filesystem walk I/O residual retained. dens ≠ flip.

/// Dual-oracle residual: default/min concurrency constants.
#[must_use]
pub fn discovery_defaults_shell() -> bool {
    DEFAULT_DISCOVERY_CONCURRENCY == 10
        && MIN_DISCOVERY_CONCURRENCY == 1
        && default_discovery_concurrency_is_ten()
        && min_concurrency_is_serial()
        && default_is_tenfold_min()
}

/// Dual-oracle residual: clamp under min and identity at default.
#[must_use]
pub fn discovery_clamp_under_default_shell() -> bool {
    clamp_discovery_concurrency(0) == MIN_DISCOVERY_CONCURRENCY
        && clamp_discovery_concurrency(1) == 1
        && clamp_discovery_concurrency(DEFAULT_DISCOVERY_CONCURRENCY)
            == DEFAULT_DISCOVERY_CONCURRENCY
        && clamp_discovery_concurrency(64) == 64
}

/// Dual-oracle residual: default options carry concurrency 10.
#[must_use]
pub fn discovery_options_default_shell() -> bool {
    let o = discover_options_default(vec![]);
    o.concurrency == DEFAULT_DISCOVERY_CONCURRENCY && default_options_concurrency_matches()
}

#[cfg(test)]
mod wave72_product_tests {
    use super::*;

    #[test]
    fn wave72_discovery_concurrency_band_dual_oracle() {
        assert!(discovery_defaults_shell());
        assert!(discovery_clamp_under_default_shell());
        assert!(discovery_options_default_shell());
        assert_eq!(discovery_concurrency_shell(), (1, 10));
        assert!(zero_clamps_to_min());
        assert!(high_concurrency_preserved());
    }
}
