//! Supported media extensions — parity with `src/utils.ts#ALL_SUPPORTED_EXTENSIONS`.

use std::collections::HashSet;
use std::path::Path;
use std::sync::OnceLock;

static SUPPORTED: OnceLock<HashSet<&'static str>> = OnceLock::new();

/// Lowercase extensions without leading dot.
#[must_use]
pub fn all_supported_extensions() -> HashSet<&'static str> {
    SUPPORTED
        .get_or_init(|| {
            [
                "jpg", "jpeg", "png", "gif", "webp", "heic", "heif", "tif", "tiff", "bmp", "raw",
                "mp4", "mov", "m4v", "mkv", "avi", "webm", "mpg", "mpeg", "3gp", "wmv", "flv",
                "divx",
            ]
            .into_iter()
            .collect()
        })
        .clone()
}

/// Extension of `path` without dot, lowercased; `None` when absent/unsupported.
#[must_use]
pub fn extension_of(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(str::to_ascii_lowercase)
        .filter(|ext| all_supported_extensions().contains(ext.as_str()))
}