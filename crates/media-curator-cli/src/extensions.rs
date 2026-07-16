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

// ── wave62 pure residual dens: image/video extension kind dual-oracle ──
// Dual-oracle residual of product extension families (utils ALL_SUPPORTED).
// Filesystem walk I/O residual retained. dens ≠ flip.

/// Dual-oracle residual: image extension family (lowercase, no dot).
pub const IMAGE_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "gif", "webp", "heic", "heif", "tif", "tiff", "bmp", "raw",
];

/// Dual-oracle residual: video extension family (lowercase, no dot).
pub const VIDEO_EXTENSIONS: &[&str] = &[
    "mp4", "mov", "m4v", "mkv", "avi", "webm", "mpg", "mpeg", "3gp", "wmv", "flv", "divx",
];

/// Dual-oracle residual: image family membership.
#[must_use]
pub fn is_image_extension(ext: &str) -> bool {
    IMAGE_EXTENSIONS.contains(&ext.to_ascii_lowercase().as_str())
}

/// Dual-oracle residual: video family membership.
#[must_use]
pub fn is_video_extension(ext: &str) -> bool {
    VIDEO_EXTENSIONS.contains(&ext.to_ascii_lowercase().as_str())
}

/// Dual-oracle residual: supported image or video.
#[must_use]
pub fn is_media_extension(ext: &str) -> bool {
    is_image_extension(ext) || is_video_extension(ext)
}

/// Dual-oracle residual: kind label for extension.
#[must_use]
pub fn extension_kind(ext: &str) -> Option<&'static str> {
    if is_image_extension(ext) {
        Some("image")
    } else if is_video_extension(ext) {
        Some("video")
    } else {
        None
    }
}

#[cfg(test)]
mod wave62_tests {
    use super::*;

    #[test]
    fn wave62_extension_kind_dual_oracle() {
        assert_eq!(IMAGE_EXTENSIONS.len(), 11);
        assert_eq!(VIDEO_EXTENSIONS.len(), 12);
        assert!(is_image_extension("JPG"));
        assert!(is_image_extension("png"));
        assert!(!is_image_extension("mp4"));
        assert!(is_video_extension("MP4"));
        assert!(is_video_extension("mkv"));
        assert!(!is_video_extension("png"));
        assert!(is_media_extension("heic"));
        assert!(is_media_extension("webm"));
        assert!(!is_media_extension("pdf"));
        assert_eq!(extension_kind("jpeg"), Some("image"));
        assert_eq!(extension_kind("mov"), Some("video"));
        assert_eq!(extension_kind("txt"), None);
        // dual-oracle: families ⊆ all_supported_extensions
        let all = all_supported_extensions();
        for e in IMAGE_EXTENSIONS.iter().chain(VIDEO_EXTENSIONS.iter()) {
            assert!(all.contains(e), "missing {e}");
        }
    }
}
