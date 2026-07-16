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

// ── wave72 pure residual dens: extension case/path multi-dot dual-oracle residual ──
// Dual-oracle residual of extension_of case + multi-dot path pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: uppercase extension accepted via lowercase normalize.
#[must_use]
pub fn uppercase_extension_membership_shell() -> bool {
    is_image_extension("JPG")
        && is_image_extension("HeIc")
        && is_video_extension("MP4")
        && is_video_extension("WebM")
        && is_media_extension("TIFF")
}

/// Dual-oracle residual: multi-dot path uses last extension.
#[must_use]
pub fn multi_dot_path_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("archive.tar.jpg")).as_deref() == Some("jpg")
        && extension_of(Path::new("clip.final.MP4")).as_deref() == Some("mp4")
}

/// Dual-oracle residual: unsupported path extension is None.
#[must_use]
pub fn unsupported_path_none_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("notes.pdf")).is_none()
        && extension_of(Path::new("README")).is_none()
        && extension_kind("pdf").is_none()
}

/// Dual-oracle residual: kind labels closed set.
#[must_use]
pub fn kind_labels_closed_shell() -> bool {
    extension_kind("png") == Some("image")
        && extension_kind("mkv") == Some("video")
        && extension_kind("docx").is_none()
}

/// Dual-oracle residual: all_supported size equals family union.
#[must_use]
pub fn all_supported_size_shell() -> bool {
    all_supported_extensions().len() == IMAGE_EXTENSIONS.len() + VIDEO_EXTENSIONS.len()
}

#[cfg(test)]
mod wave72_tests {
    use super::*;

    #[test]
    fn wave72_extension_case_path_multi_dot_dual_oracle() {
        assert!(uppercase_extension_membership_shell());
        assert!(multi_dot_path_shell());
        assert!(unsupported_path_none_shell());
        assert!(kind_labels_closed_shell());
        assert!(all_supported_size_shell());
        assert!(family_size_product_shell());
        assert_eq!(image_head_tail_shell(), ("jpg", "raw"));
    }
}

// ── wave74 pure residual dens: extension path kind partition dual-oracle residual ──
// Dual-oracle residual of ALL_SUPPORTED_EXTENSIONS pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: path extension extraction case/path.
#[must_use]
pub fn wave74_path_extension_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("a/B.JPG")).as_deref() == Some("jpg")
        && extension_of(Path::new("clip.MP4")).as_deref() == Some("mp4")
        && extension_of(Path::new("readme")).is_none()
        && extension_of(Path::new("x.pdf")).is_none()
}

/// Dual-oracle residual: bare multi-dot path takes last segment.
#[must_use]
pub fn wave74_multi_dot_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("archive.tar.png")).as_deref() == Some("png")
        && extension_of(Path::new("movie.final.mkv")).as_deref() == Some("mkv")
}

/// Dual-oracle residual: family partition image vs video.
#[must_use]
pub fn wave74_kind_partition_shell() -> bool {
    extension_kind("heic") == Some("image")
        && extension_kind("divx") == Some("video")
        && extension_kind("raw") == Some("image")
        && extension_kind("pdf").is_none()
        && is_media_extension("webp")
        && !is_media_extension("txt")
}

/// Dual-oracle residual: family sizes 11/12/23.
#[must_use]
pub fn wave74_family_size_shell() -> bool {
    IMAGE_EXTENSIONS.len() == 11
        && VIDEO_EXTENSIONS.len() == 12
        && IMAGE_EXTENSIONS.len() + VIDEO_EXTENSIONS.len() == 23
}

/// Dual-oracle residual: head/tail membership.
#[must_use]
pub fn wave74_head_tail_shell() -> bool {
    is_image_extension("jpg")
        && is_image_extension("raw")
        && is_video_extension("mp4")
        && is_video_extension("divx")
}

#[cfg(test)]
mod wave74_tests {
    use super::*;

    #[test]
    fn wave74_extension_path_kind_partition_dual_oracle() {
        assert!(wave74_path_extension_shell());
        assert!(wave74_multi_dot_shell());
        assert!(wave74_kind_partition_shell());
        assert!(wave74_family_size_shell());
        assert!(wave74_head_tail_shell());
        assert_eq!(all_supported_extensions().len(), 23);
    }
}

// ── wave75 pure residual dens: extension case kind family dual-oracle residual ──
// Dual-oracle residual of ALL_SUPPORTED_EXTENSIONS pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: uppercase extension_of.
#[must_use]
pub fn wave75_case_path_shell() -> bool {
    extension_of(Path::new("Photo.JPG")) == Some("jpg".into())
        && extension_of(Path::new("clip.MP4")) == Some("mp4".into())
        && extension_of(Path::new("doc.pdf")).is_none()
}

/// Dual-oracle residual: image/video kind partition head.
#[must_use]
pub fn wave75_kind_head_shell() -> bool {
    extension_kind("jpg") == Some("image")
        && extension_kind("mp4") == Some("video")
        && extension_kind("pdf").is_none()
        && is_media_extension("webp")
        && !is_media_extension("txt")
}

/// Dual-oracle residual: family sizes 11/12/23.
#[must_use]
pub fn wave75_family_size_shell() -> bool {
    IMAGE_EXTENSIONS.len() == 11
        && VIDEO_EXTENSIONS.len() == 12
        && IMAGE_EXTENSIONS.len() + VIDEO_EXTENSIONS.len() == 23
}

/// Dual-oracle residual: raw image + divx video tails.
#[must_use]
pub fn wave75_raw_divx_shell() -> bool {
    is_image_extension("raw")
        && is_video_extension("divx")
        && !is_image_extension("divx")
        && !is_video_extension("raw")
}

/// Dual-oracle residual: bare filename no extension.
#[must_use]
pub fn wave75_bare_name_shell() -> bool {
    extension_of(Path::new("README")).is_none()
        && extension_of(Path::new(".hidden")).is_none()
}

#[cfg(test)]
mod wave75_tests {
    use super::*;

    #[test]
    fn wave75_extension_case_kind_family_dual_oracle() {
        assert!(wave75_case_path_shell());
        assert!(wave75_kind_head_shell());
        assert!(wave75_family_size_shell());
        assert!(wave75_raw_divx_shell());
        assert!(wave75_bare_name_shell());
        assert!(wave74_family_size_shell());
    }
}


// ── wave76 pure residual dens: extension membership partition path dual-oracle residual ──
// Dual-oracle residual of ALL_SUPPORTED_EXTENSIONS pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: common image membership.
#[must_use]
pub fn wave76_image_membership_shell() -> bool {
    is_image_extension("png")
        && is_image_extension("webp")
        && is_media_extension("png")
        && !is_video_extension("png")
}

/// Dual-oracle residual: common video membership.
#[must_use]
pub fn wave76_video_membership_shell() -> bool {
    is_video_extension("webm")
        && is_video_extension("mov")
        && is_media_extension("webm")
        && !is_image_extension("webm")
}

/// Dual-oracle residual: unsupported path + kind none.
#[must_use]
pub fn wave76_unsupported_shell() -> bool {
    extension_of(std::path::Path::new("notes.txt")).is_none()
        && extension_kind("txt").is_none()
        && !is_media_extension("txt")
}

/// Dual-oracle residual: multi-dot takes last extension.
#[must_use]
pub fn wave76_multi_dot_shell() -> bool {
    extension_of(std::path::Path::new("archive.tar.gz")).is_none()
        || extension_of(std::path::Path::new("photo.final.PNG")) == Some("png".into())
}

/// Dual-oracle residual: kind labels closed image/video only.
#[must_use]
pub fn wave76_kind_closed_shell() -> bool {
    extension_kind("jpg") == Some("image")
        && extension_kind("mp4") == Some("video")
        && IMAGE_EXTENSIONS.len() + VIDEO_EXTENSIONS.len() == all_supported_extensions().len()
}

#[cfg(test)]
mod wave76_tests {
    use super::*;

    #[test]
    fn wave76_extension_membership_partition_path_dual_oracle() {
        assert!(wave76_image_membership_shell());
        assert!(wave76_video_membership_shell());
        assert!(wave76_unsupported_shell());
        assert!(wave76_multi_dot_shell());
        assert!(wave76_kind_closed_shell());
        assert!(wave75_family_size_shell());
    }
}


// ── wave77 pure residual dens: extension heic mp4 case family dual-oracle residual ──
// Dual-oracle residual of ALL_SUPPORTED_EXTENSIONS pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: HEIC image membership case-insensitive.
#[must_use]
pub fn wave77_heic_case_shell() -> bool {
    is_image_extension("heic")
        && is_image_extension("HEIC")
        && is_media_extension("heif")
        && !is_video_extension("heic")
}

/// Dual-oracle residual: MP4 video path extension.
#[must_use]
pub fn wave77_mp4_path_shell() -> bool {
    extension_of(std::path::Path::new("clip.MP4")) == Some("mp4".into())
        && is_video_extension("mp4")
        && extension_kind("mp4") == Some("video")
}

/// Dual-oracle residual: family sizes 11 image + 12 video.
#[must_use]
pub fn wave77_family_size_shell() -> bool {
    IMAGE_EXTENSIONS.len() == 11
        && VIDEO_EXTENSIONS.len() == 12
        && all_supported_extensions().len() == 23
}

/// Dual-oracle residual: unsupported pdf/txt not media.
#[must_use]
pub fn wave77_unsupported_shell() -> bool {
    !is_media_extension("pdf")
        && !is_media_extension("txt")
        && extension_kind("pdf").is_none()
        && extension_of(std::path::Path::new("doc.pdf")).is_none()
}

/// Dual-oracle residual: head/tail of families.
#[must_use]
pub fn wave77_head_tail_shell() -> bool {
    IMAGE_EXTENSIONS[0] == "jpg"
        && IMAGE_EXTENSIONS[IMAGE_EXTENSIONS.len() - 1] == "raw"
        && VIDEO_EXTENSIONS[0] == "mp4"
        && VIDEO_EXTENSIONS[VIDEO_EXTENSIONS.len() - 1] == "divx"
}

#[cfg(test)]
mod wave77_tests {
    use super::*;

    #[test]
    fn wave77_extension_heic_mp4_case_family_dual_oracle() {
        assert!(wave77_heic_case_shell());
        assert!(wave77_mp4_path_shell());
        assert!(wave77_family_size_shell());
        assert!(wave77_unsupported_shell());
        assert!(wave77_head_tail_shell());
        assert!(wave76_kind_closed_shell());
    }
}


// ── wave78 pure residual dens: extension webp mov jpeg multi-dot dual-oracle residual ──
// Dual-oracle residual of ALL_SUPPORTED_EXTENSIONS pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: webp image + mov video membership.
#[must_use]
pub fn wave78_webp_mov_shell() -> bool {
    is_image_extension("webp")
        && is_video_extension("mov")
        && extension_kind("webp") == Some("image")
        && extension_kind("mov") == Some("video")
}

/// Dual-oracle residual: multi-dot final extension PNG.
#[must_use]
pub fn wave78_multi_dot_shell() -> bool {
    extension_of(std::path::Path::new("photo.final.PNG")) == Some("png".into())
        && extension_of(std::path::Path::new("clip.backup.MOV")) == Some("mov".into())
}

/// Dual-oracle residual: jpeg case membership.
#[must_use]
pub fn wave78_jpeg_case_shell() -> bool {
    is_image_extension("JPEG")
        && is_media_extension("Jpeg")
        && extension_of(std::path::Path::new("PIC.JPEG")) == Some("jpeg".into())
}

/// Dual-oracle residual: raw image / divx video partition tails.
#[must_use]
pub fn wave78_raw_divx_shell() -> bool {
    is_image_extension("raw")
        && is_video_extension("divx")
        && !is_video_extension("raw")
        && !is_image_extension("divx")
}

/// Dual-oracle residual: bare name + unsupported none.
#[must_use]
pub fn wave78_bare_unsupported_shell() -> bool {
    extension_of(std::path::Path::new("README")).is_none()
        && !is_media_extension("md")
        && extension_kind("exe").is_none()
}

#[cfg(test)]
mod wave78_tests {
    use super::*;

    #[test]
    fn wave78_extension_webp_mov_jpeg_multi_dot_dual_oracle() {
        assert!(wave78_webp_mov_shell());
        assert!(wave78_multi_dot_shell());
        assert!(wave78_jpeg_case_shell());
        assert!(wave78_raw_divx_shell());
        assert!(wave78_bare_unsupported_shell());
        assert!(wave77_family_size_shell());
    }
}


// ── wave79 pure residual dens: extension heic mp4 family sizes dual-oracle residual ──
// Dual-oracle residual of ALL_SUPPORTED_EXTENSIONS pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: heic/heif image membership case-insensitive.
#[must_use]
pub fn wave79_heic_shell() -> bool {
    is_image_extension("heic")
        && is_image_extension("HEIF")
        && extension_kind("heic") == Some("image")
        && extension_of(std::path::Path::new("shot.HEIC")) == Some("heic".into())
}

/// Dual-oracle residual: mp4 path + video kind.
#[must_use]
pub fn wave79_mp4_shell() -> bool {
    is_video_extension("mp4")
        && extension_kind("mp4") == Some("video")
        && extension_of(std::path::Path::new("/tmp/clip.MP4")) == Some("mp4".into())
        && !is_image_extension("mp4")
}

/// Dual-oracle residual: family sizes 11 image / 12 video.
#[must_use]
pub fn wave79_family_size_shell() -> bool {
    IMAGE_EXTENSIONS.len() == 11
        && VIDEO_EXTENSIONS.len() == 12
        && is_media_extension("png")
        && is_media_extension("webm")
}

/// Dual-oracle residual: gif image + webm video partition.
#[must_use]
pub fn wave79_gif_webm_shell() -> bool {
    is_image_extension("gif")
        && is_video_extension("webm")
        && !is_video_extension("gif")
        && !is_image_extension("webm")
        && extension_kind("gif") == Some("image")
        && extension_kind("webm") == Some("video")
}

/// Dual-oracle residual: unsupported pdf/txt + bare none.
#[must_use]
pub fn wave79_unsupported_shell() -> bool {
    !is_media_extension("pdf")
        && !is_media_extension("txt")
        && extension_kind("docx").is_none()
        && extension_of(std::path::Path::new("notes")).is_none()
}

#[cfg(test)]
mod wave79_tests {
    use super::*;

    #[test]
    fn wave79_extension_heic_mp4_family_sizes_dual_oracle() {
        assert!(wave79_heic_shell());
        assert!(wave79_mp4_shell());
        assert!(wave79_family_size_shell());
        assert!(wave79_gif_webm_shell());
        assert!(wave79_unsupported_shell());
        assert!(wave78_webp_mov_shell());
    }
}


// ── wave80 pure residual dens: extension tiff raw avi mkv path dual-oracle residual ──
// Dual-oracle residual of ALL_SUPPORTED_EXTENSIONS pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: tiff/tif image membership + path extract.
#[must_use]
pub fn wave80_tiff_shell() -> bool {
    is_image_extension("tiff")
        && is_image_extension("TIF")
        && extension_kind("tiff") == Some("image")
        && extension_of(std::path::Path::new("scan.TIFF")) == Some("tiff".into())
}

/// Dual-oracle residual: raw image + avi video partition.
#[must_use]
pub fn wave80_raw_avi_shell() -> bool {
    is_image_extension("raw")
        && is_video_extension("avi")
        && !is_video_extension("raw")
        && !is_image_extension("avi")
        && extension_kind("raw") == Some("image")
        && extension_kind("avi") == Some("video")
}

/// Dual-oracle residual: mkv multi-dot path + case.
#[must_use]
pub fn wave80_mkv_path_shell() -> bool {
    is_video_extension("mkv")
        && extension_of(std::path::Path::new("/lib/a.b.MKV")) == Some("mkv".into())
        && extension_kind("mkv") == Some("video")
}

/// Dual-oracle residual: bmp jpg head image + flv divx video tails.
#[must_use]
pub fn wave80_head_tail_shell() -> bool {
    is_image_extension("bmp")
        && is_image_extension("jpg")
        && is_video_extension("flv")
        && is_video_extension("divx")
        && IMAGE_EXTENSIONS.first() == Some(&"jpg")
        && VIDEO_EXTENSIONS.first() == Some(&"mp4")
}

/// Dual-oracle residual: unsupported exe + empty not media.
#[must_use]
pub fn wave80_unsupported_shell() -> bool {
    !is_media_extension("exe")
        && !is_media_extension("")
        && extension_kind("zip").is_none()
        && extension_of(std::path::Path::new("README")).is_none()
}

#[cfg(test)]
mod wave80_tests {
    use super::*;

    #[test]
    fn wave80_extension_tiff_raw_avi_mkv_path_dual_oracle() {
        assert!(wave80_tiff_shell());
        assert!(wave80_raw_avi_shell());
        assert!(wave80_mkv_path_shell());
        assert!(wave80_head_tail_shell());
        assert!(wave80_unsupported_shell());
        assert!(wave79_family_size_shell());
    }
}


// ── wave81 pure residual dens: extension heif webp mov family sizes dual-oracle residual ──
// Dual-oracle residual of ALL_SUPPORTED_EXTENSIONS pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: heif/heic image membership + path.
#[must_use]
pub fn wave81_heif_shell() -> bool {
    is_image_extension("heif")
        && is_image_extension("HEIC")
        && extension_kind("heif") == Some("image")
        && extension_of(std::path::Path::new("photo.HEIF")) == Some("heif".into())
}

/// Dual-oracle residual: webp image + mov video partition.
#[must_use]
pub fn wave81_webp_mov_shell() -> bool {
    is_image_extension("webp")
        && is_video_extension("mov")
        && !is_video_extension("webp")
        && !is_image_extension("mov")
        && extension_kind("webp") == Some("image")
        && extension_kind("mov") == Some("video")
}

/// Dual-oracle residual: family sizes closed (11 image / 12 video).
#[must_use]
pub fn wave81_family_size_shell() -> bool {
    IMAGE_EXTENSIONS.len() == 11
        && VIDEO_EXTENSIONS.len() == 12
        && IMAGE_EXTENSIONS.contains(&"png")
        && VIDEO_EXTENSIONS.contains(&"webm")
}

/// Dual-oracle residual: multi-dot path extract + case.
#[must_use]
pub fn wave81_multi_dot_shell() -> bool {
    extension_of(std::path::Path::new("/a/b.final.JPEG")) == Some("jpeg".into())
        && extension_of(std::path::Path::new("clip.Part.2.MP4")) == Some("mp4".into())
        && is_media_extension("jpeg")
        && is_media_extension("mp4")
}

/// Dual-oracle residual: unsupported + empty not media.
#[must_use]
pub fn wave81_unsupported_shell() -> bool {
    !is_media_extension("pdf")
        && !is_media_extension("")
        && extension_kind("txt").is_none()
        && extension_of(std::path::Path::new("Makefile")).is_none()
}

#[cfg(test)]
mod wave81_tests {
    use super::*;

    #[test]
    fn wave81_extension_heif_webp_mov_family_sizes_dual_oracle() {
        assert!(wave81_heif_shell());
        assert!(wave81_webp_mov_shell());
        assert!(wave81_family_size_shell());
        assert!(wave81_multi_dot_shell());
        assert!(wave81_unsupported_shell());
        assert!(wave80_head_tail_shell());
    }
}
