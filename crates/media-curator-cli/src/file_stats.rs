//! Pure file-stats decision core — size+hash digest envelope (I/O-free).

use serde::{Deserialize, Serialize};

/// Deterministic stats envelope used by differential harnesses.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FileStatsCore {
    pub size: u64,
    pub hash_hex: String,
}

/// Build a stats core from precomputed size + hex hash (TS parity input shape).
#[must_use]
pub fn build_file_stats_core(size: u64, hash_hex: impl Into<String>) -> FileStatsCore {
    FileStatsCore {
        size,
        hash_hex: hash_hex.into(),
    }
}

/// Validate hex hash shape (even length, hex chars only).
#[must_use]
pub fn is_valid_hash_hex(hash_hex: &str) -> bool {
    !hash_hex.is_empty()
        && hash_hex.len().is_multiple_of(2)
        && hash_hex.bytes().all(|b| b.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_envelope() {
        let s = build_file_stats_core(1024, "abcd");
        assert_eq!(s.size, 1024);
        assert_eq!(s.hash_hex, "abcd");
    }

    #[test]
    fn validates_hex() {
        assert!(is_valid_hash_hex("deadbeef"));
        assert!(!is_valid_hash_hex("xyz"));
        assert!(!is_valid_hash_hex("abc"));
    }
}
