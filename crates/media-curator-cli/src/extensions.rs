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

// ── wave69 pure residual dens: extension family size ladder dual-oracle residual ──
// Dual-oracle residual of image/video extension family sizes pure half.
// Filesystem walk I/O residual retained. dens ≠ flip.

/// Dual-oracle residual: family size shell (image, video, total).
#[must_use]
pub fn extension_family_size_shell() -> [usize; 3] {
    [
        IMAGE_EXTENSIONS.len(),
        VIDEO_EXTENSIONS.len(),
        IMAGE_EXTENSIONS.len() + VIDEO_EXTENSIONS.len(),
    ]
}

/// Dual-oracle residual: kind probe ladder.
#[must_use]
pub fn extension_kind_probe_ladder() -> [Option<&'static str>; 4] {
    [
        extension_kind("jpeg"),
        extension_kind("mov"),
        extension_kind("heic"),
        extension_kind("pdf"),
    ]
}

/// Dual-oracle residual: case-insensitive media probes.
#[must_use]
pub fn extension_case_probe_ok() -> bool {
    is_image_extension("JPG")
        && is_image_extension("Png")
        && is_video_extension("MP4")
        && is_video_extension("MkV")
        && is_media_extension("WEBP")
        && !is_media_extension("PDF")
}

/// Dual-oracle residual: families ⊆ all_supported.
#[must_use]
pub fn families_subset_of_all_supported() -> bool {
    let all = all_supported_extensions();
    IMAGE_EXTENSIONS
        .iter()
        .chain(VIDEO_EXTENSIONS.iter())
        .all(|e| all.contains(e))
}

/// Dual-oracle residual: all_supported count matches family sum.
#[must_use]
pub fn all_supported_count_matches_families() -> bool {
    all_supported_extensions().len() == IMAGE_EXTENSIONS.len() + VIDEO_EXTENSIONS.len()
}

#[cfg(test)]
mod wave69_tests {
    use super::*;

    #[test]
    fn wave69_extension_family_size_ladder_dual_oracle() {
        assert_eq!(extension_family_size_shell(), [11, 12, 23]);
        assert_eq!(
            extension_kind_probe_ladder(),
            [Some("image"), Some("video"), Some("image"), None]
        );
        assert!(extension_case_probe_ok());
        assert!(families_subset_of_all_supported());
        assert!(all_supported_count_matches_families());
        assert!(!is_image_extension("mp4"));
        assert!(!is_video_extension("png"));
    }
}


// ── wave70 pure residual dens: extension family union + path extract dual-oracle residual ──
// Dual-oracle residual of utils ALL_SUPPORTED + image/video partition pure halves.
// Filesystem walk / transfer I/O residual retained. dens ≠ flip.

/// Dual-oracle residual: image+video count equals all_supported size.
#[must_use]
pub fn extension_family_union_size_shell() -> bool {
    let all = all_supported_extensions();
    IMAGE_EXTENSIONS.len() + VIDEO_EXTENSIONS.len() == all.len()
        && IMAGE_EXTENSIONS.len() + VIDEO_EXTENSIONS.len() == 23
}

/// Dual-oracle residual: families are disjoint.
#[must_use]
pub fn extension_families_disjoint() -> bool {
    IMAGE_EXTENSIONS.iter().all(|e| !VIDEO_EXTENSIONS.contains(e))
}

/// Dual-oracle residual: path extension extract for known media.
#[must_use]
pub fn extension_of_path_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("/a/b/Photo.JPG")).as_deref() == Some("jpg")
        && extension_of(Path::new("clip.MP4")).as_deref() == Some("mp4")
        && extension_of(Path::new("notes.txt")).is_none()
        && extension_of(Path::new("noext")).is_none()
}

/// Dual-oracle residual: kind partition covers all supported.
#[must_use]
pub fn kind_covers_supported_shell() -> bool {
    IMAGE_EXTENSIONS.iter().all(|e| extension_kind(e) == Some("image"))
        && VIDEO_EXTENSIONS.iter().all(|e| extension_kind(e) == Some("video"))
}

#[cfg(test)]
mod wave70_tests {
    use super::*;

    #[test]
    fn wave70_extension_union_path_dual_oracle() {
        assert!(extension_family_union_size_shell());
        assert!(extension_families_disjoint());
        assert!(extension_of_path_shell());
        assert!(kind_covers_supported_shell());
        assert_eq!(IMAGE_EXTENSIONS.len(), 11);
        assert_eq!(VIDEO_EXTENSIONS.len(), 12);
        assert!(is_media_extension("webp"));
        assert!(!is_media_extension("pdf"));
    }
}


// ── wave71 pure residual dens: extension family head/tail + case dual-oracle residual ──
// Dual-oracle residual of IMAGE/VIDEO extension catalogs pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: image head/tail extensions.
#[must_use]
pub fn image_head_tail_shell() -> (&'static str, &'static str) {
    (IMAGE_EXTENSIONS[0], IMAGE_EXTENSIONS[IMAGE_EXTENSIONS.len() - 1])
}

/// Dual-oracle residual: video head/tail extensions.
#[must_use]
pub fn video_head_tail_shell() -> (&'static str, &'static str) {
    (VIDEO_EXTENSIONS[0], VIDEO_EXTENSIONS[VIDEO_EXTENSIONS.len() - 1])
}

/// Dual-oracle residual: lowercase membership for common types.
#[must_use]
pub fn common_media_membership_shell() -> bool {
    is_image_extension("jpg")
        && is_image_extension("png")
        && is_video_extension("mp4")
        && is_video_extension("webm")
        && !is_image_extension("mp4")
        && !is_video_extension("jpg")
}

/// Dual-oracle residual: family sizes fixed product catalog.
#[must_use]
pub fn family_size_product_shell() -> bool {
    IMAGE_EXTENSIONS.len() == 11 && VIDEO_EXTENSIONS.len() == 12
}

#[cfg(test)]
mod wave71_tests {
    use super::*;

    #[test]
    fn wave71_extension_head_tail_membership_dual_oracle() {
        assert_eq!(image_head_tail_shell(), ("jpg", "raw"));
        assert_eq!(video_head_tail_shell(), ("mp4", "divx"));
        assert!(common_media_membership_shell());
        assert!(family_size_product_shell());
        assert!(extension_families_disjoint());
        assert!(is_media_extension("heic"));
        assert!(!is_media_extension("docx"));
    }
}
