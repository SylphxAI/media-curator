//! File stats (size + MD5) — parity baseline for `src/jobs/fileStats.ts` core slice.

use std::fs;
use std::io;
use std::path::Path;

use serde::Serialize;

/// MD5 + byte size for a file on disk.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FileStats {
    pub md5: String,
    pub size: u64,
}

/// Compute MD5 hex digest and size for `path`.
pub fn file_stats_for_path(path: &Path) -> io::Result<FileStats> {
    let bytes = fs::read(path)?;
    let digest = format!("{:x}", md5::compute(&bytes));
    Ok(FileStats {
        md5: digest,
        size: bytes.len() as u64,
    })
}