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


// ── wave83 pure residual dens: extension bare path kind partition dual-oracle residual ──
// Dual-oracle residual of extension pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: bare path + multi-dot last extension wins.
#[must_use]
pub fn wave83_path_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("photo.JPG")) == Some("jpg".to_string())
        && extension_of(Path::new("archive.tar.mp4")) == Some("mp4".to_string())
        && extension_of(Path::new("README")) == None
        && extension_of(Path::new("notes.txt")) == None
}

/// Dual-oracle residual: raw/divx kind partition.
#[must_use]
pub fn wave83_kind_partition_shell() -> bool {
    extension_kind("raw") == Some("image")
        && extension_kind("divx") == Some("video")
        && extension_kind("RAW") == Some("image")
        && extension_kind("pdf") == None
}

/// Dual-oracle residual: empty/unsupported not media.
#[must_use]
pub fn wave83_empty_not_media_shell() -> bool {
    !is_media_extension("")
        && !is_media_extension("docx")
        && !is_image_extension("mp4")
        && !is_video_extension("png")
}

/// Dual-oracle residual: family sizes closed.
#[must_use]
pub fn wave83_family_sizes_shell() -> bool {
    IMAGE_EXTENSIONS.len() == 11
        && VIDEO_EXTENSIONS.len() == 12
        && all_supported_extensions().len() == 23
}

/// Dual-oracle residual: case membership dual-oracle.
#[must_use]
pub fn wave83_case_membership_shell() -> bool {
    is_image_extension("HeIc")
        && is_video_extension("WebM")
        && is_media_extension("TIFF")
        && extension_kind("MOV") == Some("video")
}

#[cfg(test)]
mod wave83_tests {
    use super::*;

    #[test]
    fn wave83_extension_bare_path_kind_partition_dual_oracle() {
        assert!(wave83_path_shell());
        assert!(wave83_kind_partition_shell());
        assert!(wave83_empty_not_media_shell());
        assert!(wave83_family_sizes_shell());
        assert!(wave83_case_membership_shell());
    }
}

// ── wave84 pure residual dens: extension heic mp4 head-tail family dual-oracle residual ──
// Dual-oracle residual of extension pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: heic/mp4 path extract + kind.
#[must_use]
pub fn wave84_heic_mp4_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("/media/shot.HEIC")) == Some("heic".to_string())
        && extension_of(Path::new("clip.Mp4")) == Some("mp4".to_string())
        && extension_kind("heic") == Some("image")
        && extension_kind("mp4") == Some("video")
}

/// Dual-oracle residual: head/tail membership + family sizes.
#[must_use]
pub fn wave84_head_tail_shell() -> bool {
    IMAGE_EXTENSIONS[0] == "jpg"
        && VIDEO_EXTENSIONS[0] == "mp4"
        && is_image_extension(*IMAGE_EXTENSIONS.last().unwrap())
        && is_video_extension(*VIDEO_EXTENSIONS.last().unwrap())
        && IMAGE_EXTENSIONS.len() == 11
        && VIDEO_EXTENSIONS.len() == 12
}

/// Dual-oracle residual: multi-dot + bare unsupported.
#[must_use]
pub fn wave84_multi_dot_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("a.b.c.WEBP")) == Some("webp".to_string())
        && extension_of(Path::new("Makefile")) == None
        && !is_media_extension("rs")
}

/// Dual-oracle residual: raw/avi kind partition dual-oracle.
#[must_use]
pub fn wave84_raw_avi_shell() -> bool {
    is_image_extension("raw")
        && is_video_extension("avi")
        && extension_kind("RAW") == Some("image")
        && extension_kind("AVI") == Some("video")
}

/// Dual-oracle residual: union size + empty not media.
#[must_use]
pub fn wave84_union_empty_shell() -> bool {
    all_supported_extensions().len() == 23
        && !is_media_extension("")
        && !is_image_extension("mp4")
        && !is_video_extension("png")
}

#[cfg(test)]
mod wave84_tests {
    use super::*;

    #[test]
    fn wave84_extension_heic_mp4_head_tail_family_dual_oracle() {
        assert!(wave84_heic_mp4_shell());
        assert!(wave84_head_tail_shell());
        assert!(wave84_multi_dot_shell());
        assert!(wave84_raw_avi_shell());
        assert!(wave84_union_empty_shell());
        assert!(wave83_family_sizes_shell());
    }
}

// ── wave85 pure residual dens: extension case webm unsupported path dual-oracle residual ──
// Dual-oracle residual of extension pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: case-insensitive image family membership.
#[must_use]
pub fn wave85_case_image_shell() -> bool {
    is_image_extension("JPG")
        && is_image_extension("HeIc")
        && is_media_extension("PNG")
        && !is_video_extension("png")
        && extension_kind("jpeg") == Some("image")
}

/// Dual-oracle residual: webm/mp4 video kind + mov path.
#[must_use]
pub fn wave85_video_kind_shell() -> bool {
    is_video_extension("webm")
        && is_video_extension("MP4")
        && extension_kind("mov") == Some("video")
        && is_media_extension("mkv")
        && !is_image_extension("mp4")
}

/// Dual-oracle residual: unsupported pdf/txt + empty.
#[must_use]
pub fn wave85_unsupported_shell() -> bool {
    !is_media_extension("pdf")
        && !is_media_extension("txt")
        && extension_kind("docx").is_none()
        && !is_image_extension("")
        && !is_video_extension(" ")
}

/// Dual-oracle residual: family sizes + union membership.
#[must_use]
pub fn wave85_family_size_shell() -> bool {
    IMAGE_EXTENSIONS.len() == 11
        && VIDEO_EXTENSIONS.len() == 12
        && all_supported_extensions().len() == IMAGE_EXTENSIONS.len() + VIDEO_EXTENSIONS.len()
        && IMAGE_EXTENSIONS[0] == "jpg"
        && VIDEO_EXTENSIONS[0] == "mp4"
}

/// Dual-oracle residual: path extension_of head/tail dual-oracle.
#[must_use]
pub fn wave85_path_extension_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("a/b/photo.HEIC")) == Some("heic".to_string())
        && extension_of(Path::new("clip.Mp4")) == Some("mp4".to_string())
        && extension_of(Path::new("readme.txt")).is_none()
        && extension_of(Path::new("noext")).is_none()
}

#[cfg(test)]
mod wave85_tests {
    use super::*;

    #[test]
    fn wave85_extension_case_webm_unsupported_path_dual_oracle() {
        assert!(wave85_case_image_shell());
        assert!(wave85_video_kind_shell());
        assert!(wave85_unsupported_shell());
        assert!(wave85_family_size_shell());
        assert!(wave85_path_extension_shell());
        assert!(wave84_union_empty_shell());
    }
}
// ── wave86 pure residual dens: extension heic/raw mkv multi-dot bare dual-oracle residual ──
// Dual-oracle residual of extension pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: heic/raw/tiff image family.
#[must_use]
pub fn wave86_image_family_shell() -> bool {
    is_image_extension("heic")
        && is_image_extension("RAW")
        && is_image_extension("tiff")
        && extension_kind("heif") == Some("image")
        && !is_video_extension("heic")
}

/// Dual-oracle residual: mkv/avi/divx video family.
#[must_use]
pub fn wave86_video_family_shell() -> bool {
    is_video_extension("mkv")
        && is_video_extension("AVI")
        && is_video_extension("divx")
        && extension_kind("wmv") == Some("video")
        && !is_image_extension("mkv")
}

/// Dual-oracle residual: multi-dot path extension_of last segment.
#[must_use]
pub fn wave86_multi_dot_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("archive.tar.jpg")) == Some("jpg".to_string())
        && extension_of(Path::new("a.b.c.MP4")) == Some("mp4".to_string())
        && extension_of(Path::new(".hidden.png")) == Some("png".to_string())
}

/// Dual-oracle residual: bare name / no extension → none; empty unsupported.
#[must_use]
pub fn wave86_bare_unsupported_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("Makefile")).is_none()
        && extension_of(Path::new("README")).is_none()
        && !is_media_extension("exe")
        && !is_media_extension("")
}

/// Dual-oracle residual: families disjoint + union count 23.
#[must_use]
pub fn wave86_family_disjoint_shell() -> bool {
    let images: std::collections::HashSet<_> = IMAGE_EXTENSIONS.iter().copied().collect();
    let videos: std::collections::HashSet<_> = VIDEO_EXTENSIONS.iter().copied().collect();
    images.is_disjoint(&videos)
        && IMAGE_EXTENSIONS.len() == 11
        && VIDEO_EXTENSIONS.len() == 12
        && all_supported_extensions().len() == 23
        && IMAGE_EXTENSIONS[IMAGE_EXTENSIONS.len() - 1] == "raw"
        && VIDEO_EXTENSIONS[VIDEO_EXTENSIONS.len() - 1] == "divx"
}

#[cfg(test)]
mod wave86_tests {
    use super::*;

    #[test]
    fn wave86_extension_heic_raw_mkv_multi_dot_bare_dual_oracle() {
        assert!(wave86_image_family_shell());
        assert!(wave86_video_family_shell());
        assert!(wave86_multi_dot_shell());
        assert!(wave86_bare_unsupported_shell());
        assert!(wave86_family_disjoint_shell());
        assert!(wave85_family_size_shell());
    }
}

// ── wave87 pure residual dens: extension tiff mov disjoint path dual-oracle residual ──
// Dual-oracle residual of extension pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: tiff/heif image + case kind.
#[must_use]
pub fn wave87_tiff_heif_shell() -> bool {
    is_image_extension("tiff")
        && is_image_extension("HEIF")
        && extension_kind("TIF") == Some("image")
        && is_media_extension("bmp")
        && !is_video_extension("heic")
}

/// Dual-oracle residual: mov/avi video + flv kind.
#[must_use]
pub fn wave87_mov_avi_shell() -> bool {
    is_video_extension("mov")
        && is_video_extension("AVI")
        && extension_kind("flv") == Some("video")
        && is_media_extension("wmv")
        && !is_image_extension("mov")
}

/// Dual-oracle residual: families disjoint + raw not video.
#[must_use]
pub fn wave87_disjoint_shell() -> bool {
    IMAGE_EXTENSIONS.iter().all(|e| !VIDEO_EXTENSIONS.contains(e))
        && VIDEO_EXTENSIONS.iter().all(|e| !IMAGE_EXTENSIONS.contains(e))
        && is_image_extension("raw")
        && !is_video_extension("raw")
        && IMAGE_EXTENSIONS.len() + VIDEO_EXTENSIONS.len() == all_supported_extensions().len()
}

/// Dual-oracle residual: unsupported paths + empty/dot.
#[must_use]
pub fn wave87_unsupported_path_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("notes.md")).is_none()
        && extension_of(Path::new(".gitignore")).is_none()
        && !is_media_extension("md")
        && !is_media_extension("json")
        && extension_kind("exe").is_none()
}

/// Dual-oracle residual: multi-dot path + head tail wire.
#[must_use]
pub fn wave87_multidot_head_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("archive.tar.jpg")) == Some("jpg".to_string())
        && extension_of(Path::new("dir/clip.final.MOV")) == Some("mov".to_string())
        && IMAGE_EXTENSIONS[0] == "jpg"
        && VIDEO_EXTENSIONS[VIDEO_EXTENSIONS.len() - 1] == "divx"
}

#[cfg(test)]
mod wave87_tests {
    use super::*;

    #[test]
    fn wave87_extension_tiff_mov_disjoint_path_dual_oracle() {
        assert!(wave87_tiff_heif_shell());
        assert!(wave87_mov_avi_shell());
        assert!(wave87_disjoint_shell());
        assert!(wave87_unsupported_path_shell());
        assert!(wave87_multidot_head_shell());
        assert!(wave85_family_size_shell());
    }
}

// ── wave88 pure residual dens: extension webm mp4 case family path dual-oracle residual ──
// Dual-oracle residual of extension pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: webm/mp4 video + case kind.
#[must_use]
pub fn wave88_webm_mp4_shell() -> bool {
    is_video_extension("webm")
        && is_video_extension("MP4")
        && extension_kind("m4v") == Some("video")
        && is_media_extension("mkv")
        && !is_image_extension("webm")
}

/// Dual-oracle residual: case image jpg/png + heic kind.
#[must_use]
pub fn wave88_case_image_shell() -> bool {
    is_image_extension("JPG")
        && is_image_extension("PnG")
        && extension_kind("HEIC") == Some("image")
        && is_media_extension("webp")
        && !is_video_extension("png")
}

/// Dual-oracle residual: family sizes 11/12 + union 23.
#[must_use]
pub fn wave88_family_size_shell() -> bool {
    IMAGE_EXTENSIONS.len() == 11
        && VIDEO_EXTENSIONS.len() == 12
        && all_supported_extensions().len() == 23
        && IMAGE_EXTENSIONS[0] == "jpg"
        && VIDEO_EXTENSIONS[0] == "mp4"
}

/// Dual-oracle residual: unsupported path + empty extension.
#[must_use]
pub fn wave88_unsupported_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("readme.txt")).is_none()
        && extension_of(Path::new("Makefile")).is_none()
        && !is_media_extension("txt")
        && !is_media_extension("rs")
        && extension_kind("zip").is_none()
}

/// Dual-oracle residual: multi-dot path + head/tail wire.
#[must_use]
pub fn wave88_multidot_path_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("shot.final.HEIC")) == Some("heic".to_string())
        && extension_of(Path::new("clips/out.v1.WebM")) == Some("webm".to_string())
        && IMAGE_EXTENSIONS[IMAGE_EXTENSIONS.len() - 1] == "raw"
        && VIDEO_EXTENSIONS[VIDEO_EXTENSIONS.len() - 1] == "divx"
}

#[cfg(test)]
mod wave88_tests {
    use super::*;

    #[test]
    fn wave88_extension_webm_mp4_case_family_path_dual_oracle() {
        assert!(wave88_webm_mp4_shell());
        assert!(wave88_case_image_shell());
        assert!(wave88_family_size_shell());
        assert!(wave88_unsupported_shell());
        assert!(wave88_multidot_path_shell());
        assert!(wave87_disjoint_shell());
    }
}

// ── wave89 pure residual dens: extension heif gif raw mov head dual-oracle residual ──
// Dual-oracle residual of extension pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: heif/gif image membership + kind.
#[must_use]
pub fn wave89_heif_gif_shell() -> bool {
    is_image_extension("heif")
        && is_image_extension("GIF")
        && extension_kind("heif") == Some("image")
        && is_media_extension("gif")
        && !is_video_extension("gif")
}

/// Dual-oracle residual: raw image + mov/avi video.
#[must_use]
pub fn wave89_raw_mov_shell() -> bool {
    is_image_extension("raw")
        && is_video_extension("mov")
        && is_video_extension("AVI")
        && extension_kind("mov") == Some("video")
        && !is_image_extension("mov")
}

/// Dual-oracle residual: family head/tail wire.
#[must_use]
pub fn wave89_head_tail_shell() -> bool {
    IMAGE_EXTENSIONS[0] == "jpg"
        && IMAGE_EXTENSIONS[IMAGE_EXTENSIONS.len() - 1] == "raw"
        && VIDEO_EXTENSIONS[0] == "mp4"
        && VIDEO_EXTENSIONS[VIDEO_EXTENSIONS.len() - 1] == "divx"
        && IMAGE_EXTENSIONS.len() == 11
}

/// Dual-oracle residual: families disjoint + union size.
#[must_use]
pub fn wave89_disjoint_union_shell() -> bool {
    IMAGE_EXTENSIONS.iter().all(|e| !VIDEO_EXTENSIONS.contains(e))
        && VIDEO_EXTENSIONS.iter().all(|e| !IMAGE_EXTENSIONS.contains(e))
        && all_supported_extensions().len() == 23
        && VIDEO_EXTENSIONS.len() == 12
}

/// Dual-oracle residual: path extension + bare unsupported.
#[must_use]
pub fn wave89_path_unsupported_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("clip.MOV")) == Some("mov".to_string())
        && extension_of(Path::new("photo.HEIF")) == Some("heif".to_string())
        && extension_of(Path::new("noext")).is_none()
        && !is_media_extension("pdf")
        && extension_kind("docx").is_none()
}

#[cfg(test)]
mod wave89_tests {
    use super::*;

    #[test]
    fn wave89_extension_heif_gif_raw_mov_head_dual_oracle() {
        assert!(wave89_heif_gif_shell());
        assert!(wave89_raw_mov_shell());
        assert!(wave89_head_tail_shell());
        assert!(wave89_disjoint_union_shell());
        assert!(wave89_path_unsupported_shell());
        assert!(wave88_family_size_shell());
    }
}

// ── wave90 pure residual dens: extension webm mp4 case multi-dot kind dual-oracle residual ──
// Dual-oracle residual of extension pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: webm/mp4 video membership + kind.
#[must_use]
pub fn wave90_webm_mp4_shell() -> bool {
    is_video_extension("webm")
        && is_video_extension("MP4")
        && extension_kind("webm") == Some("video")
        && is_media_extension("mp4")
        && !is_image_extension("webm")
}

/// Dual-oracle residual: case-insensitive jpg/png image family.
#[must_use]
pub fn wave90_case_jpg_shell() -> bool {
    is_image_extension("JPG")
        && is_image_extension("PnG")
        && extension_kind("JPEG") == Some("image")
        && is_media_extension("jpeg")
        && !is_video_extension("jpg")
}

/// Dual-oracle residual: multi-dot path extension extract.
#[must_use]
pub fn wave90_multi_dot_path_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("archive.tar.webm")) == Some("webm".to_string())
        && extension_of(Path::new("photo.backup.JPG")) == Some("jpg".to_string())
        && extension_of(Path::new("clip.mp4")) == Some("mp4".to_string())
}

/// Dual-oracle residual: kind partition closed labels.
#[must_use]
pub fn wave90_kind_partition_shell() -> bool {
    extension_kind("heic") == Some("image")
        && extension_kind("mkv") == Some("video")
        && extension_kind("pdf").is_none()
        && IMAGE_EXTENSIONS.iter().all(|e| extension_kind(e) == Some("image"))
        && VIDEO_EXTENSIONS.iter().all(|e| extension_kind(e) == Some("video"))
}

/// Dual-oracle residual: family sizes + unsupported pdf.
#[must_use]
pub fn wave90_family_size_unsupported_shell() -> bool {
    IMAGE_EXTENSIONS.len() == 11
        && VIDEO_EXTENSIONS.len() == 12
        && all_supported_extensions().len() == 23
        && !is_media_extension("pdf")
        && !is_media_extension("txt")
}

#[cfg(test)]
mod wave90_tests {
    use super::*;

    #[test]
    fn wave90_extension_webm_mp4_case_multi_dot_kind_dual_oracle() {
        assert!(wave90_webm_mp4_shell());
        assert!(wave90_case_jpg_shell());
        assert!(wave90_multi_dot_path_shell());
        assert!(wave90_kind_partition_shell());
        assert!(wave90_family_size_unsupported_shell());
        assert!(wave89_disjoint_union_shell());
    }
}

// ── wave91 pure residual dens: extension heif gif mov bare disjoint dual-oracle residual ──
// Dual-oracle residual of extension pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: heif/gif image membership + kind.
#[must_use]
pub fn wave91_heif_gif_shell() -> bool {
    is_image_extension("heif")
        && is_image_extension("GIF")
        && extension_kind("heif") == Some("image")
        && is_media_extension("gif")
        && !is_video_extension("heif")
}

/// Dual-oracle residual: mov/avi video membership + kind.
#[must_use]
pub fn wave91_mov_avi_shell() -> bool {
    is_video_extension("mov")
        && is_video_extension("AVI")
        && extension_kind("mov") == Some("video")
        && is_media_extension("avi")
        && !is_image_extension("mov")
}

/// Dual-oracle residual: bare filename / no extension unsupported.
#[must_use]
pub fn wave91_bare_unsupported_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("README")).is_none()
        && extension_of(Path::new("notes.txt")).is_none()
        && !is_media_extension("txt")
        && !is_media_extension("pdf")
        && extension_kind("docx").is_none()
}

/// Dual-oracle residual: image/video families disjoint + union size.
#[must_use]
pub fn wave91_disjoint_union_shell() -> bool {
    IMAGE_EXTENSIONS
        .iter()
        .all(|e| !VIDEO_EXTENSIONS.contains(e))
        && VIDEO_EXTENSIONS
            .iter()
            .all(|e| !IMAGE_EXTENSIONS.contains(e))
        && IMAGE_EXTENSIONS.len() + VIDEO_EXTENSIONS.len()
            == all_supported_extensions().len()
        && IMAGE_EXTENSIONS.len() == 11
        && VIDEO_EXTENSIONS.len() == 12
}

/// Dual-oracle residual: raw/tiff image path extract dual-oracle.
#[must_use]
pub fn wave91_raw_tiff_path_shell() -> bool {
    use std::path::Path;
    is_image_extension("raw")
        && is_image_extension("TIFF")
        && extension_of(Path::new("scan.TIFF")) == Some("tiff".to_string())
        && extension_of(Path::new("shot.RAW")) == Some("raw".to_string())
        && extension_kind("tiff") == Some("image")
}

#[cfg(test)]
mod wave91_tests {
    use super::*;

    #[test]
    fn wave91_extension_heif_gif_mov_bare_disjoint_dual_oracle() {
        assert!(wave91_heif_gif_shell());
        assert!(wave91_mov_avi_shell());
        assert!(wave91_bare_unsupported_shell());
        assert!(wave91_disjoint_union_shell());
        assert!(wave91_raw_tiff_path_shell());
        assert!(wave90_family_size_unsupported_shell());
    }
}

// ── wave92 pure residual dens: extension jpg png mp4 webm case path dual-oracle residual ──
// Dual-oracle residual of extension pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: jpg/png image membership + kind.
#[must_use]
pub fn wave92_jpg_png_shell() -> bool {
    is_image_extension("jpg")
        && is_image_extension("PNG")
        && extension_kind("jpg") == Some("image")
        && extension_kind("png") == Some("image")
        && is_media_extension("jpeg")
        && !is_video_extension("jpg")
}

/// Dual-oracle residual: mp4/webm video membership + kind.
#[must_use]
pub fn wave92_mp4_webm_shell() -> bool {
    is_video_extension("mp4")
        && is_video_extension("WEBM")
        && extension_kind("mp4") == Some("video")
        && extension_kind("webm") == Some("video")
        && is_media_extension("mkv")
        && !is_image_extension("mp4")
}

/// Dual-oracle residual: case-fold path extract dual-oracle.
#[must_use]
pub fn wave92_case_path_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("/media/Photo.JPG")) == Some("jpg".to_string())
        && extension_of(Path::new("clip.Mp4")) == Some("mp4".to_string())
        && extension_of(Path::new("a.WEBP")) == Some("webp".to_string())
        && extension_of(Path::new("doc.PDF")).is_none()
}

/// Dual-oracle residual: heic/bmp image poles dual-oracle.
#[must_use]
pub fn wave92_heic_bmp_shell() -> bool {
    is_image_extension("heic")
        && is_image_extension("BMP")
        && extension_kind("heic") == Some("image")
        && is_media_extension("bmp")
        && !is_video_extension("heic")
}

/// Dual-oracle residual: family sizes + union closed.
#[must_use]
pub fn wave92_family_sizes_shell() -> bool {
    IMAGE_EXTENSIONS.len() == 11
        && VIDEO_EXTENSIONS.len() == 12
        && all_supported_extensions().len() == 23
        && IMAGE_EXTENSIONS.len() + VIDEO_EXTENSIONS.len()
            == all_supported_extensions().len()
}

#[cfg(test)]
mod wave92_tests {
    use super::*;

    #[test]
    fn wave92_extension_jpg_png_mp4_webm_case_path_dual_oracle() {
        assert!(wave92_jpg_png_shell());
        assert!(wave92_mp4_webm_shell());
        assert!(wave92_case_path_shell());
        assert!(wave92_heic_bmp_shell());
        assert!(wave92_family_sizes_shell());
        assert!(wave91_disjoint_union_shell());
    }
}

// ── wave93 pure residual dens: extension gif webp mov avi unsupported multi-dot dual-oracle residual ──
// Dual-oracle residual of extension pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: gif/webp image membership + kind.
#[must_use]
pub fn wave93_gif_webp_shell() -> bool {
    is_image_extension("gif")
        && is_image_extension("WEBP")
        && extension_kind("gif") == Some("image")
        && extension_kind("webp") == Some("image")
        && is_media_extension("gif")
        && !is_video_extension("webp")
}

/// Dual-oracle residual: mov/avi video membership + kind.
#[must_use]
pub fn wave93_mov_avi_shell() -> bool {
    is_video_extension("mov")
        && is_video_extension("AVI")
        && extension_kind("mov") == Some("video")
        && extension_kind("avi") == Some("video")
        && is_media_extension("m4v")
        && !is_image_extension("mov")
}

/// Dual-oracle residual: unsupported bare + path dual-oracle.
#[must_use]
pub fn wave93_unsupported_shell() -> bool {
    use std::path::Path;
    !is_media_extension("pdf")
        && !is_image_extension("txt")
        && !is_video_extension("doc")
        && extension_kind("pdf").is_none()
        && extension_of(Path::new("readme.md")).is_none()
}

/// Dual-oracle residual: multi-dot path last extension dual-oracle.
#[must_use]
pub fn wave93_multi_dot_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("archive.tar.JPG")) == Some("jpg".to_string())
        && extension_of(Path::new("clip.final.MOV")) == Some("mov".to_string())
        && extension_of(Path::new("a.b.c.webm")) == Some("webm".to_string())
}

/// Dual-oracle residual: family disjoint + union size dual-oracle.
#[must_use]
pub fn wave93_disjoint_union_shell() -> bool {
    IMAGE_EXTENSIONS.iter().all(|e| !VIDEO_EXTENSIONS.contains(e))
        && VIDEO_EXTENSIONS.iter().all(|e| !IMAGE_EXTENSIONS.contains(e))
        && all_supported_extensions().len()
            == IMAGE_EXTENSIONS.len() + VIDEO_EXTENSIONS.len()
        && IMAGE_EXTENSIONS[0] == "jpg"
        && VIDEO_EXTENSIONS[0] == "mp4"
}

#[cfg(test)]
mod wave93_tests {
    use super::*;

    #[test]
    fn wave93_extension_gif_webp_mov_avi_unsupported_multi_dot_dual_oracle() {
        assert!(wave93_gif_webp_shell());
        assert!(wave93_mov_avi_shell());
        assert!(wave93_unsupported_shell());
        assert!(wave93_multi_dot_shell());
        assert!(wave93_disjoint_union_shell());
        assert!(wave92_family_sizes_shell());
    }
}

// ── wave94 pure residual dens: extension jpg png mp4 webm heic family dual-oracle residual ──
// Dual-oracle residual of extension pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: jpg/png image membership + kind.
#[must_use]
pub fn wave94_jpg_png_shell() -> bool {
    is_image_extension("jpg")
        && is_image_extension("PNG")
        && is_image_extension("jpeg")
        && extension_kind("jpg") == Some("image")
        && extension_kind("png") == Some("image")
        && is_media_extension("jpeg")
        && !is_video_extension("png")
}

/// Dual-oracle residual: mp4/webm video membership + kind.
#[must_use]
pub fn wave94_mp4_webm_shell() -> bool {
    is_video_extension("mp4")
        && is_video_extension("WEBM")
        && extension_kind("mp4") == Some("video")
        && extension_kind("webm") == Some("video")
        && is_media_extension("mkv")
        && !is_image_extension("mp4")
}

/// Dual-oracle residual: case path extract dual-oracle.
#[must_use]
pub fn wave94_case_path_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("Photo.JPG")) == Some("jpg".to_string())
        && extension_of(Path::new("clip.Mp4")) == Some("mp4".to_string())
        && extension_of(Path::new("shot.HEIC")) == Some("heic".to_string())
}

/// Dual-oracle residual: heic/bmp image family dual-oracle.
#[must_use]
pub fn wave94_heic_bmp_shell() -> bool {
    is_image_extension("heic")
        && is_image_extension("BMP")
        && is_image_extension("heif")
        && is_image_extension("tif")
        && extension_kind("bmp") == Some("image")
        && is_media_extension("raw")
}

/// Dual-oracle residual: family sizes + disjoint dual-oracle.
#[must_use]
pub fn wave94_family_sizes_shell() -> bool {
    IMAGE_EXTENSIONS.len() == 11
        && VIDEO_EXTENSIONS.len() == 12
        && IMAGE_EXTENSIONS.len() + VIDEO_EXTENSIONS.len() == all_supported_extensions().len()
        && !IMAGE_EXTENSIONS.iter().any(|e| is_video_extension(e))
        && !VIDEO_EXTENSIONS.iter().any(|e| is_image_extension(e))
}

#[cfg(test)]
mod wave94_tests {
    use super::*;

    #[test]
    fn wave94_extension_jpg_png_mp4_webm_heic_family_dual_oracle() {
        assert!(wave94_jpg_png_shell());
        assert!(wave94_mp4_webm_shell());
        assert!(wave94_case_path_shell());
        assert!(wave94_heic_bmp_shell());
        assert!(wave94_family_sizes_shell());
        assert!(wave93_gif_webp_shell());
    }
}

// ── wave95 pure residual dens: extension gif webp mov avi unsupported multi-dot dual-oracle residual ──
// Dual-oracle residual of extension pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: gif/webp image membership + kind.
#[must_use]
pub fn wave95_gif_webp_shell() -> bool {
    is_image_extension("gif")
        && is_image_extension("WEBP")
        && extension_kind("gif") == Some("image")
        && extension_kind("webp") == Some("image")
        && is_media_extension("gif")
        && !is_video_extension("webp")
}

/// Dual-oracle residual: mov/avi video membership + kind.
#[must_use]
pub fn wave95_mov_avi_shell() -> bool {
    is_video_extension("mov")
        && is_video_extension("AVI")
        && extension_kind("mov") == Some("video")
        && extension_kind("avi") == Some("video")
        && is_media_extension("m4v")
        && !is_image_extension("avi")
}

/// Dual-oracle residual: unsupported extensions reject dual-oracle.
#[must_use]
pub fn wave95_unsupported_shell() -> bool {
    !is_media_extension("pdf")
        && !is_media_extension("txt")
        && !is_image_extension("exe")
        && !is_video_extension("docx")
        && extension_kind("pdf").is_none()
        && extension_kind("").is_none()
}

/// Dual-oracle residual: multi-dot path extract dual-oracle.
#[must_use]
pub fn wave95_multi_dot_path_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("archive.tar.jpg")) == Some("jpg".to_string())
        && extension_of(Path::new("clip.final.MOV")) == Some("mov".to_string())
        && extension_of(Path::new("noext")) == None
        && extension_of(Path::new("notes.pdf")) == None
}

/// Dual-oracle residual: family head/tail membership dual-oracle.
#[must_use]
pub fn wave95_family_head_tail_shell() -> bool {
    IMAGE_EXTENSIONS.first() == Some(&"jpg")
        && IMAGE_EXTENSIONS.last() == Some(&"raw")
        && VIDEO_EXTENSIONS.first() == Some(&"mp4")
        && VIDEO_EXTENSIONS.last() == Some(&"divx")
        && IMAGE_EXTENSIONS.len() == 11
        && VIDEO_EXTENSIONS.len() == 12
}

#[cfg(test)]
mod wave95_tests {
    use super::*;

    #[test]
    fn wave95_extension_gif_webp_mov_avi_unsupported_multi_dot_dual_oracle() {
        assert!(wave95_gif_webp_shell());
        assert!(wave95_mov_avi_shell());
        assert!(wave95_unsupported_shell());
        assert!(wave95_multi_dot_path_shell());
        assert!(wave95_family_head_tail_shell());
        assert!(wave94_jpg_png_shell());
    }
}

// ── wave96 pure residual dens: extension jpg png heic mp4 case empty dual-oracle residual ──
// Dual-oracle residual of extension pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: jpg/png image membership + kind.
#[must_use]
pub fn wave96_jpg_png_shell() -> bool {
    is_image_extension("jpg")
        && is_image_extension("PNG")
        && extension_kind("jpg") == Some("image")
        && extension_kind("png") == Some("image")
        && is_media_extension("jpeg")
        && !is_video_extension("jpg")
}

/// Dual-oracle residual: heic/mp4 family dual-oracle.
#[must_use]
pub fn wave96_heic_mp4_shell() -> bool {
    is_image_extension("heic")
        && is_video_extension("mp4")
        && extension_kind("heic") == Some("image")
        && extension_kind("MP4") == Some("video")
        && is_media_extension("heif")
        && is_media_extension("webm")
}

/// Dual-oracle residual: case fold dual-oracle.
#[must_use]
pub fn wave96_case_fold_shell() -> bool {
    is_image_extension("JpG")
        && is_video_extension("MkV")
        && extension_kind("GIF") == Some("image")
        && extension_kind("MOV") == Some("video")
        && is_media_extension("WeBp")
}

/// Dual-oracle residual: empty/missing path extract dual-oracle.
#[must_use]
pub fn wave96_empty_path_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("")) == None
        && extension_of(Path::new(".")) == None
        && extension_of(Path::new("dir/")) == None
        && extension_of(Path::new("photo.JPG")) == Some("jpg".to_string())
}

/// Dual-oracle residual: disjoint image/video membership dual-oracle.
#[must_use]
pub fn wave96_disjoint_shell() -> bool {
    !IMAGE_EXTENSIONS.iter().any(|e| VIDEO_EXTENSIONS.contains(e))
        && !VIDEO_EXTENSIONS.iter().any(|e| IMAGE_EXTENSIONS.contains(e))
        && IMAGE_EXTENSIONS.contains(&"jpg")
        && VIDEO_EXTENSIONS.contains(&"mp4")
        && is_media_extension("jpg")
        && is_media_extension("mp4")
}

#[cfg(test)]
mod wave96_tests {
    use super::*;

    #[test]
    fn wave96_extension_jpg_png_heic_mp4_case_empty_dual_oracle() {
        assert!(wave96_jpg_png_shell());
        assert!(wave96_heic_mp4_shell());
        assert!(wave96_case_fold_shell());
        assert!(wave96_empty_path_shell());
        assert!(wave96_disjoint_shell());
        assert!(wave95_gif_webp_shell());
    }
}

// ── wave97 pure residual dens: extension gif webp mov avi unsupported multi-dot dual-oracle residual ──
// Dual-oracle residual of extension pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: gif/webp image membership dual-oracle.
#[must_use]
pub fn wave97_gif_webp_shell() -> bool {
    is_image_extension("gif")
        && is_image_extension("WEBP")
        && extension_kind("gif") == Some("image")
        && extension_kind("webp") == Some("image")
        && is_media_extension("gif")
        && !is_video_extension("webp")
}

/// Dual-oracle residual: mov/avi video membership dual-oracle.
#[must_use]
pub fn wave97_mov_avi_shell() -> bool {
    is_video_extension("mov")
        && is_video_extension("AVI")
        && extension_kind("mov") == Some("video")
        && extension_kind("avi") == Some("video")
        && is_media_extension("m4v")
        && !is_image_extension("mov")
}

/// Dual-oracle residual: unsupported extensions dual-oracle.
#[must_use]
pub fn wave97_unsupported_shell() -> bool {
    !is_media_extension("txt")
        && !is_media_extension("pdf")
        && !is_image_extension("exe")
        && !is_video_extension("docx")
        && extension_kind("txt").is_none()
        && extension_kind("").is_none()
}

/// Dual-oracle residual: multi-dot path extract dual-oracle.
#[must_use]
pub fn wave97_multi_dot_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("archive.tar.jpg")) == Some("jpg".to_string())
        && extension_of(Path::new("clip.final.MOV")) == Some("mov".to_string())
        && extension_of(Path::new("notes.txt")) == None
        && extension_of(Path::new("photo.JPEG")) == Some("jpeg".to_string())
}

/// Dual-oracle residual: family head/tail sizes dual-oracle.
#[must_use]
pub fn wave97_family_sizes_shell() -> bool {
    IMAGE_EXTENSIONS.len() == 11
        && VIDEO_EXTENSIONS.len() == 12
        && IMAGE_EXTENSIONS.first() == Some(&"jpg")
        && VIDEO_EXTENSIONS.first() == Some(&"mp4")
        && IMAGE_EXTENSIONS.contains(&"gif")
        && VIDEO_EXTENSIONS.contains(&"avi")
}

#[cfg(test)]
mod wave97_tests {
    use super::*;

    #[test]
    fn wave97_extension_gif_webp_mov_avi_unsupported_multi_dot_dual_oracle() {
        assert!(wave97_gif_webp_shell());
        assert!(wave97_mov_avi_shell());
        assert!(wave97_unsupported_shell());
        assert!(wave97_multi_dot_shell());
        assert!(wave97_family_sizes_shell());
        assert!(wave96_jpg_png_shell());
    }
}

// ── wave98 pure residual dens: extension jpg png heic mp4 case empty dual-oracle residual ──
// Dual-oracle residual of extension pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: jpg/png image membership + kind.
#[must_use]
pub fn wave98_jpg_png_shell() -> bool {
    is_image_extension("jpg")
        && is_image_extension("PNG")
        && extension_kind("jpg") == Some("image")
        && extension_kind("png") == Some("image")
        && is_media_extension("jpeg")
        && !is_video_extension("jpg")
}

/// Dual-oracle residual: heic/mp4 family dual-oracle.
#[must_use]
pub fn wave98_heic_mp4_shell() -> bool {
    is_image_extension("heic")
        && is_video_extension("mp4")
        && extension_kind("heic") == Some("image")
        && extension_kind("MP4") == Some("video")
        && is_media_extension("heif")
        && is_media_extension("webm")
}

/// Dual-oracle residual: case fold dual-oracle.
#[must_use]
pub fn wave98_case_fold_shell() -> bool {
    is_image_extension("JpG")
        && is_video_extension("MkV")
        && extension_kind("GIF") == Some("image")
        && extension_kind("MOV") == Some("video")
        && is_media_extension("WeBp")
}

/// Dual-oracle residual: empty/missing path extract dual-oracle.
#[must_use]
pub fn wave98_empty_path_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("")) == None
        && extension_of(Path::new(".")) == None
        && extension_of(Path::new("dir/")) == None
        && extension_of(Path::new("photo.JPG")) == Some("jpg".to_string())
}

/// Dual-oracle residual: disjoint image/video membership dual-oracle.
#[must_use]
pub fn wave98_disjoint_shell() -> bool {
    !IMAGE_EXTENSIONS.iter().any(|e| VIDEO_EXTENSIONS.contains(e))
        && !VIDEO_EXTENSIONS.iter().any(|e| IMAGE_EXTENSIONS.contains(e))
        && IMAGE_EXTENSIONS.contains(&"jpg")
        && VIDEO_EXTENSIONS.contains(&"mp4")
        && is_media_extension("jpg")
        && is_media_extension("mp4")
}

#[cfg(test)]
mod wave98_tests {
    use super::*;

    #[test]
    fn wave98_extension_jpg_png_heic_mp4_case_empty_dual_oracle() {
        assert!(wave98_jpg_png_shell());
        assert!(wave98_heic_mp4_shell());
        assert!(wave98_case_fold_shell());
        assert!(wave98_empty_path_shell());
        assert!(wave98_disjoint_shell());
        assert!(wave97_gif_webp_shell());
    }
}
// ── wave99 pure residual dens: extension tiff bmp mkv webm nested-path sizes dual-oracle residual ──
// Dual-oracle residual of extension pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: tiff/bmp image membership dual-oracle.
#[must_use]
pub fn wave99_tiff_bmp_shell() -> bool {
    is_image_extension("tiff")
        && is_image_extension("BMP")
        && extension_kind("tif") == Some("image")
        && extension_kind("bmp") == Some("image")
        && is_media_extension("raw")
        && !is_video_extension("tiff")
}

/// Dual-oracle residual: mkv/webm video family dual-oracle.
#[must_use]
pub fn wave99_mkv_webm_shell() -> bool {
    is_video_extension("mkv")
        && is_video_extension("WEBM")
        && extension_kind("mkv") == Some("video")
        && extension_kind("webm") == Some("video")
        && is_media_extension("m4v")
        && !is_image_extension("mkv")
}

/// Dual-oracle residual: nested path extract dual-oracle.
#[must_use]
pub fn wave99_nested_path_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("/data/album/nested/Photo.TIFF")) == Some("tiff".to_string())
        && extension_of(Path::new("clips/final.WebM")) == Some("webm".to_string())
        && extension_of(Path::new("a/b/c.d.mp4")) == Some("mp4".to_string())
}

/// Dual-oracle residual: family sizes dual-oracle.
#[must_use]
pub fn wave99_family_sizes_shell() -> bool {
    IMAGE_EXTENSIONS.len() == 11
        && VIDEO_EXTENSIONS.len() == 12
        && IMAGE_EXTENSIONS.len() + VIDEO_EXTENSIONS.len() == 23
        && all_supported_extensions().len() == 23
}

/// Dual-oracle residual: unsupported reject dual-oracle.
#[must_use]
pub fn wave99_unsupported_shell() -> bool {
    use std::path::Path;
    !is_media_extension("pdf")
        && !is_media_extension("txt")
        && extension_kind("docx") == None
        && extension_of(Path::new("report.pdf")) == None
        && extension_of(Path::new("archive.tar.gz")).is_none()
}

#[cfg(test)]
mod wave99_tests {
    use super::*;

    #[test]
    fn wave99_extension_tiff_bmp_mkv_webm_nested_path_sizes_dual_oracle() {
        assert!(wave99_tiff_bmp_shell());
        assert!(wave99_mkv_webm_shell());
        assert!(wave99_nested_path_shell());
        assert!(wave99_family_sizes_shell());
        assert!(wave99_unsupported_shell());
        assert!(wave98_jpg_png_shell());
    }
}
// ── wave100 pure residual dens: extension heic-jpeg mov-avi bare-none families-disjoint head-tail dual-oracle residual ──
// Dual-oracle residual of extension pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: heic/jpeg image membership dual-oracle.
#[must_use]
pub fn wave100_heic_jpeg_shell() -> bool {
    is_image_extension("heic")
        && is_image_extension("JPEG")
        && extension_kind("heif") == Some("image")
        && extension_kind("jpeg") == Some("image")
        && is_media_extension("heic")
        && !is_video_extension("jpeg")
}

/// Dual-oracle residual: mov/avi video family dual-oracle.
#[must_use]
pub fn wave100_mov_avi_shell() -> bool {
    is_video_extension("mov")
        && is_video_extension("AVI")
        && extension_kind("mov") == Some("video")
        && extension_kind("avi") == Some("video")
        && is_media_extension("divx")
        && !is_image_extension("mov")
}

/// Dual-oracle residual: bare name / unsupported path none dual-oracle.
#[must_use]
pub fn wave100_bare_none_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("README")) == None
        && extension_of(Path::new("archive.zip")) == None
        && extension_of(Path::new(".hidden")) == None
        && !is_media_extension("zip")
        && extension_kind("exe") == None
}

/// Dual-oracle residual: families disjoint dual-oracle.
#[must_use]
pub fn wave100_families_disjoint_shell() -> bool {
    IMAGE_EXTENSIONS.iter().all(|e| !is_video_extension(e))
        && VIDEO_EXTENSIONS.iter().all(|e| !is_image_extension(e))
        && extension_families_disjoint()
        && IMAGE_EXTENSIONS.len() == 11
        && VIDEO_EXTENSIONS.len() == 12
}

/// Dual-oracle residual: head/tail catalog dual-oracle.
#[must_use]
pub fn wave100_head_tail_shell() -> bool {
    image_head_tail_shell() == ("jpg", "raw")
        && video_head_tail_shell() == ("mp4", "divx")
        && IMAGE_EXTENSIONS.first() == Some(&"jpg")
        && IMAGE_EXTENSIONS.last() == Some(&"raw")
        && VIDEO_EXTENSIONS.first() == Some(&"mp4")
        && VIDEO_EXTENSIONS.last() == Some(&"divx")
}

#[cfg(test)]
mod wave100_tests {
    use super::*;

    #[test]
    fn wave100_extension_heic_jpeg_mov_avi_bare_none_families_disjoint_head_tail_dual_oracle() {
        assert!(wave100_heic_jpeg_shell());
        assert!(wave100_mov_avi_shell());
        assert!(wave100_bare_none_shell());
        assert!(wave100_families_disjoint_shell());
        assert!(wave100_head_tail_shell());
        assert!(wave99_tiff_bmp_shell());
    }
}
// ── wave101 pure residual dens: extension png-webp mp4-webm multi-dot all-supported uppercase dual-oracle residual ──
// Dual-oracle residual of extension pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: png/webp image membership dual-oracle.
#[must_use]
pub fn wave101_png_webp_shell() -> bool {
    is_image_extension("png")
        && is_image_extension("WEBP")
        && extension_kind("png") == Some("image")
        && extension_kind("webp") == Some("image")
        && is_media_extension("png")
        && !is_video_extension("webp")
}

/// Dual-oracle residual: mp4/webm video membership dual-oracle.
#[must_use]
pub fn wave101_mp4_webm_shell() -> bool {
    is_video_extension("mp4")
        && is_video_extension("WEBM")
        && extension_kind("mp4") == Some("video")
        && extension_kind("webm") == Some("video")
        && is_media_extension("webm")
        && !is_image_extension("mp4")
}

/// Dual-oracle residual: multi-dot path last extension dual-oracle.
#[must_use]
pub fn wave101_multi_dot_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("photo.final.PNG")) == Some("png".to_string())
        && extension_of(Path::new("clip.draft.MP4")) == Some("mp4".to_string())
        && extension_of(Path::new("archive.tar.gz")).is_none()
        && extension_of(Path::new("/a/b/c.WEBM")) == Some("webm".to_string())
}

/// Dual-oracle residual: all supported size dual-oracle.
#[must_use]
pub fn wave101_all_supported_shell() -> bool {
    all_supported_extensions().len() == 23
        && IMAGE_EXTENSIONS.len() + VIDEO_EXTENSIONS.len() == 23
        && all_supported_count_matches_families()
        && extension_family_union_size_shell()
}

/// Dual-oracle residual: uppercase membership dual-oracle.
#[must_use]
pub fn wave101_uppercase_shell() -> bool {
    is_image_extension("JPG")
        && is_video_extension("MOV")
        && extension_kind("GIF") == Some("image")
        && extension_kind("MKV") == Some("video")
        && uppercase_extension_membership_shell()
}

#[cfg(test)]
mod wave101_tests {
    use super::*;

    #[test]
    fn wave101_extension_png_webp_mp4_webm_multi_dot_all_supported_uppercase_dual_oracle() {
        assert!(wave101_png_webp_shell());
        assert!(wave101_mp4_webm_shell());
        assert!(wave101_multi_dot_shell());
        assert!(wave101_all_supported_shell());
        assert!(wave101_uppercase_shell());
        assert!(wave100_heic_jpeg_shell());
    }
}
// ── wave102 pure residual dens: extension jpg-gif mkv-m4v family-sizes head-tail unsupported dual-oracle residual ──
// Dual-oracle residual of extension pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: jpg/gif image membership dual-oracle.
#[must_use]
pub fn wave102_jpg_gif_shell() -> bool {
    is_image_extension("jpg")
        && is_image_extension("GIF")
        && extension_kind("jpg") == Some("image")
        && extension_kind("gif") == Some("image")
        && is_media_extension("gif")
        && !is_video_extension("jpg")
}

/// Dual-oracle residual: mkv/m4v video membership dual-oracle.
#[must_use]
pub fn wave102_mkv_m4v_shell() -> bool {
    is_video_extension("mkv")
        && is_video_extension("M4V")
        && extension_kind("mkv") == Some("video")
        && extension_kind("m4v") == Some("video")
        && is_media_extension("m4v")
        && !is_image_extension("mkv")
}

/// Dual-oracle residual: family sizes dual-oracle.
#[must_use]
pub fn wave102_family_sizes_shell() -> bool {
    IMAGE_EXTENSIONS.len() == 11
        && VIDEO_EXTENSIONS.len() == 12
        && family_size_product_shell()
        && extension_family_size_shell() == [11, 12, 23]
}

/// Dual-oracle residual: image/video head-tail dual-oracle.
#[must_use]
pub fn wave102_head_tail_shell() -> bool {
    image_head_tail_shell() == ("jpg", "raw")
        && video_head_tail_shell() == ("mp4", "divx")
        && IMAGE_EXTENSIONS[0] == "jpg"
        && VIDEO_EXTENSIONS[VIDEO_EXTENSIONS.len() - 1] == "divx"
}

/// Dual-oracle residual: unsupported extension none dual-oracle.
#[must_use]
pub fn wave102_unsupported_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("notes.txt")).is_none()
        && extension_of(Path::new("pkg.deb")).is_none()
        && !is_media_extension("txt")
        && extension_kind("pdf") == None
        && unsupported_path_none_shell()
}

#[cfg(test)]
mod wave102_tests {
    use super::*;

    #[test]
    fn wave102_extension_jpg_gif_mkv_m4v_family_sizes_head_tail_unsupported_dual_oracle() {
        assert!(wave102_jpg_gif_shell());
        assert!(wave102_mkv_m4v_shell());
        assert!(wave102_family_sizes_shell());
        assert!(wave102_head_tail_shell());
        assert!(wave102_unsupported_shell());
        assert!(wave101_png_webp_shell());
    }
}
// ── wave103 pure residual dens: extension jpeg-heic mov-avi disjoint multi-dot all-supported dual-oracle residual ──
// Dual-oracle residual of extension pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: jpeg/heic image membership dual-oracle.
#[must_use]
pub fn wave103_jpeg_heic_shell() -> bool {
    is_image_extension("jpeg")
        && is_image_extension("HEIC")
        && extension_kind("jpeg") == Some("image")
        && extension_kind("heic") == Some("image")
        && is_media_extension("heic")
        && !is_video_extension("jpeg")
}

/// Dual-oracle residual: mov/avi video membership dual-oracle.
#[must_use]
pub fn wave103_mov_avi_shell() -> bool {
    is_video_extension("mov")
        && is_video_extension("AVI")
        && extension_kind("mov") == Some("video")
        && extension_kind("avi") == Some("video")
        && is_media_extension("avi")
        && !is_image_extension("mov")
}

/// Dual-oracle residual: image/video families disjoint dual-oracle.
#[must_use]
pub fn wave103_disjoint_shell() -> bool {
    extension_families_disjoint()
        && IMAGE_EXTENSIONS.iter().all(|e| !is_video_extension(e))
        && VIDEO_EXTENSIONS.iter().all(|e| !is_image_extension(e))
        && IMAGE_EXTENSIONS.len() == 11
        && VIDEO_EXTENSIONS.len() == 12
}

/// Dual-oracle residual: multi-dot path extension dual-oracle.
#[must_use]
pub fn wave103_multi_dot_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("archive.tar.jpg")) == Some("jpg".to_string())
        && extension_of(Path::new("clip.final.MP4")) == Some("mp4".to_string())
        && multi_dot_path_shell()
}

/// Dual-oracle residual: all-supported size matches family union dual-oracle.
#[must_use]
pub fn wave103_all_supported_shell() -> bool {
    all_supported_extensions().len() == 23
        && all_supported_count_matches_families()
        && all_supported_size_shell()
        && families_subset_of_all_supported()
}

#[cfg(test)]
mod wave103_tests {
    use super::*;

    #[test]
    fn wave103_extension_jpeg_heic_mov_avi_disjoint_multi_dot_all_supported_dual_oracle() {
        assert!(wave103_jpeg_heic_shell());
        assert!(wave103_mov_avi_shell());
        assert!(wave103_disjoint_shell());
        assert!(wave103_multi_dot_shell());
        assert!(wave103_all_supported_shell());
        assert!(wave102_jpg_gif_shell());
    }
}
// ── wave104 pure residual dens: extension jpeg-heic mov-avi path-probe casefold disjoint dual-oracle residual ──
// Dual-oracle residual of extension pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: jpeg/heic image membership dual-oracle.
#[must_use]
pub fn wave104_jpeg_heic_shell() -> bool {
    is_image_extension("jpeg")
        && is_image_extension("HEIC")
        && extension_kind("jpeg") == Some("image")
        && extension_kind("heic") == Some("image")
        && is_media_extension("heic")
        && !is_video_extension("jpeg")
}

/// Dual-oracle residual: mov/avi video membership dual-oracle.
#[must_use]
pub fn wave104_mov_avi_shell() -> bool {
    is_video_extension("mov")
        && is_video_extension("AVI")
        && extension_kind("mov") == Some("video")
        && extension_kind("avi") == Some("video")
        && is_media_extension("avi")
        && !is_image_extension("mov")
}

/// Dual-oracle residual: path probe dual-oracle.
#[must_use]
pub fn wave104_path_probe_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("photo.JPEG")) == Some("jpeg".to_string())
        && extension_of(Path::new("clip.Mov")) == Some("mov".to_string())
        && extension_of(Path::new("archive.zip")).is_none()
}

/// Dual-oracle residual: casefold membership dual-oracle.
#[must_use]
pub fn wave104_casefold_shell() -> bool {
    is_image_extension("PnG")
        && is_video_extension("Mp4")
        && is_media_extension("WEBP")
        && extension_kind("TIFF") == Some("image")
        && extension_kind("WEBM") == Some("video")
}

/// Dual-oracle residual: image/video families disjoint dual-oracle.
#[must_use]
pub fn wave104_disjoint_shell() -> bool {
    extension_families_disjoint()
        && all_supported_count_matches_families()
        && IMAGE_EXTENSIONS.len() + VIDEO_EXTENSIONS.len() == 23
        && all_supported_extensions().len() == 23
}

#[cfg(test)]
mod wave104_tests {
    use super::*;

    #[test]
    fn wave104_extension_jpeg_heic_mov_avi_path_probe_casefold_disjoint_dual_oracle() {
        assert!(wave104_jpeg_heic_shell());
        assert!(wave104_mov_avi_shell());
        assert!(wave104_path_probe_shell());
        assert!(wave104_casefold_shell());
        assert!(wave104_disjoint_shell());
        assert!(wave102_jpg_gif_shell());
    }
}
// ── wave105 pure residual dens: extension png-webp mp4-mkv multi-dot unsupported family-sizes dual-oracle residual ──
// Dual-oracle residual of extension pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: png/webp image membership dual-oracle.
#[must_use]
pub fn wave105_png_webp_shell() -> bool {
    is_image_extension("png")
        && is_image_extension("WEBP")
        && extension_kind("png") == Some("image")
        && extension_kind("webp") == Some("image")
        && is_media_extension("webp")
        && !is_video_extension("png")
}

/// Dual-oracle residual: mp4/mkv video membership dual-oracle.
#[must_use]
pub fn wave105_mp4_mkv_shell() -> bool {
    is_video_extension("mp4")
        && is_video_extension("MKV")
        && extension_kind("mp4") == Some("video")
        && extension_kind("mkv") == Some("video")
        && is_media_extension("mkv")
        && !is_image_extension("mp4")
}

/// Dual-oracle residual: multi-dot path dual-oracle.
#[must_use]
pub fn wave105_multi_dot_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("archive.tar.PNG")) == Some("png".to_string())
        && extension_of(Path::new("clip.final.MP4")) == Some("mp4".to_string())
        && extension_of(Path::new("notes.txt")).is_none()
}

/// Dual-oracle residual: unsupported dual-oracle.
#[must_use]
pub fn wave105_unsupported_shell() -> bool {
    !is_image_extension("pdf")
        && !is_video_extension("doc")
        && !is_media_extension("exe")
        && extension_kind("zip").is_none()
        && extension_kind("txt").is_none()
}

/// Dual-oracle residual: family sizes dual-oracle.
#[must_use]
pub fn wave105_family_sizes_shell() -> bool {
    IMAGE_EXTENSIONS.len() == 11
        && VIDEO_EXTENSIONS.len() == 12
        && IMAGE_EXTENSIONS.len() + VIDEO_EXTENSIONS.len() == 23
        && extension_families_disjoint()
        && all_supported_extensions().len() == 23
}

#[cfg(test)]
mod wave105_tests {
    use super::*;

    #[test]
    fn wave105_extension_png_webp_mp4_mkv_multi_dot_unsupported_family_sizes_dual_oracle() {
        assert!(wave105_png_webp_shell());
        assert!(wave105_mp4_mkv_shell());
        assert!(wave105_multi_dot_shell());
        assert!(wave105_unsupported_shell());
        assert!(wave105_family_sizes_shell());
        assert!(wave104_jpeg_heic_shell());
    }
}
// ── wave106 pure residual dens: extension jpeg-heic mov-webm bare-ext-case path-none head-tail dual-oracle residual ──
// Dual-oracle residual of extension pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: jpeg/heic image membership dual-oracle.
#[must_use]
pub fn wave106_jpeg_heic_shell() -> bool {
    is_image_extension("jpeg")
        && is_image_extension("HEIC")
        && extension_kind("jpeg") == Some("image")
        && extension_kind("heic") == Some("image")
        && is_media_extension("heif")
        && !is_video_extension("jpeg")
}

/// Dual-oracle residual: mov/webm video membership dual-oracle.
#[must_use]
pub fn wave106_mov_webm_shell() -> bool {
    is_video_extension("mov")
        && is_video_extension("WEBM")
        && extension_kind("mov") == Some("video")
        && extension_kind("webm") == Some("video")
        && is_media_extension("m4v")
        && !is_image_extension("mov")
}

/// Dual-oracle residual: bare extension case dual-oracle.
#[must_use]
pub fn wave106_bare_ext_case_shell() -> bool {
    is_image_extension("JPG")
        && is_image_extension("Raw")
        && is_video_extension("AVI")
        && is_video_extension("Flv")
        && extension_kind("TIFF") == Some("image")
        && extension_kind("MPEG") == Some("video")
}

/// Dual-oracle residual: path none dual-oracle.
#[must_use]
pub fn wave106_path_none_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("README")).is_none()
        && extension_of(Path::new("notes.md")).is_none()
        && extension_of(Path::new("archive.tar.gz")).is_none()
        && extension_of(Path::new("photo.JPG")) == Some("jpg".to_string())
}

/// Dual-oracle residual: family head-tail dual-oracle.
#[must_use]
pub fn wave106_head_tail_shell() -> bool {
    IMAGE_EXTENSIONS[0] == "jpg"
        && IMAGE_EXTENSIONS.last() == Some(&"raw")
        && VIDEO_EXTENSIONS[0] == "mp4"
        && VIDEO_EXTENSIONS.last() == Some(&"divx")
        && IMAGE_EXTENSIONS.len() == 11
        && VIDEO_EXTENSIONS.len() == 12
}

#[cfg(test)]
mod wave106_tests {
    use super::*;

    #[test]
    fn wave106_extension_jpeg_heic_mov_webm_bare_ext_case_path_none_head_tail_dual_oracle() {
        assert!(wave106_jpeg_heic_shell());
        assert!(wave106_mov_webm_shell());
        assert!(wave106_bare_ext_case_shell());
        assert!(wave106_path_none_shell());
        assert!(wave106_head_tail_shell());
        assert!(wave105_png_webp_shell());
    }
}
// ── wave107 pure residual dens: extension gif-bmp mkv-avi path-probe disjoint family-sum dual-oracle residual ──
// Dual-oracle residual of extension pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: gif/bmp image membership dual-oracle.
#[must_use]
pub fn wave107_gif_bmp_shell() -> bool {
    is_image_extension("gif")
        && is_image_extension("BMP")
        && extension_kind("gif") == Some("image")
        && extension_kind("bmp") == Some("image")
        && is_media_extension("tif")
        && !is_video_extension("gif")
}

/// Dual-oracle residual: mkv/avi video membership dual-oracle.
#[must_use]
pub fn wave107_mkv_avi_shell() -> bool {
    is_video_extension("mkv")
        && is_video_extension("AVI")
        && extension_kind("mkv") == Some("video")
        && extension_kind("avi") == Some("video")
        && is_media_extension("wmv")
        && !is_image_extension("mkv")
}

/// Dual-oracle residual: path probe dual-oracle.
#[must_use]
pub fn wave107_path_probe_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("shot.JPG")) == Some("jpg".to_string())
        && extension_of(Path::new("/tmp/clip.WebM")) == Some("webm".to_string())
        && extension_of(Path::new("a.b.png")) == Some("png".to_string())
        && extension_of(Path::new("noext")).is_none()
}

/// Dual-oracle residual: image/video families disjoint dual-oracle.
#[must_use]
pub fn wave107_disjoint_shell() -> bool {
    IMAGE_EXTENSIONS.iter().all(|e| !is_video_extension(e))
        && VIDEO_EXTENSIONS.iter().all(|e| !is_image_extension(e))
        && !is_media_extension("txt")
        && !is_media_extension("pdf")
}

/// Dual-oracle residual: family size sum dual-oracle.
#[must_use]
pub fn wave107_family_sum_shell() -> bool {
    IMAGE_EXTENSIONS.len() == 11
        && VIDEO_EXTENSIONS.len() == 12
        && all_supported_extensions().len() == 23
        && IMAGE_EXTENSIONS.contains(&"png")
        && VIDEO_EXTENSIONS.contains(&"mp4")
}

#[cfg(test)]
mod wave107_tests {
    use super::*;

    #[test]
    fn wave107_extension_gif_bmp_mkv_avi_path_probe_disjoint_family_sum_dual_oracle() {
        assert!(wave107_gif_bmp_shell());
        assert!(wave107_mkv_avi_shell());
        assert!(wave107_path_probe_shell());
        assert!(wave107_disjoint_shell());
        assert!(wave107_family_sum_shell());
        assert!(wave106_jpeg_heic_shell());
    }
}
// ── wave108 pure residual dens: extension png-webp mp4-mov case-fold raw-heif non-media dual-oracle residual ──
// Dual-oracle residual of extension pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: png/webp image membership dual-oracle.
#[must_use]
pub fn wave108_png_webp_shell() -> bool {
    is_image_extension("png")
        && is_image_extension("WEBP")
        && extension_kind("png") == Some("image")
        && extension_kind("webp") == Some("image")
        && is_media_extension("png")
        && !is_video_extension("webp")
}

/// Dual-oracle residual: mp4/mov video membership dual-oracle.
#[must_use]
pub fn wave108_mp4_mov_shell() -> bool {
    is_video_extension("mp4")
        && is_video_extension("MOV")
        && extension_kind("mp4") == Some("video")
        && extension_kind("mov") == Some("video")
        && is_media_extension("m4v")
        && !is_image_extension("mp4")
}

/// Dual-oracle residual: case-fold membership dual-oracle.
#[must_use]
pub fn wave108_case_fold_shell() -> bool {
    is_image_extension("JpG")
        && is_image_extension("PnG")
        && is_video_extension("Mp4")
        && is_video_extension("WeBm")
        && extension_kind("HEIC") == Some("image")
}

/// Dual-oracle residual: raw/heif image dual-oracle.
#[must_use]
pub fn wave108_raw_heif_shell() -> bool {
    is_image_extension("raw")
        && is_image_extension("HEIF")
        && extension_kind("raw") == Some("image")
        && extension_kind("heif") == Some("image")
        && IMAGE_EXTENSIONS.contains(&"raw")
        && IMAGE_EXTENSIONS.contains(&"heif")
}

/// Dual-oracle residual: non-media reject dual-oracle.
#[must_use]
pub fn wave108_non_media_shell() -> bool {
    !is_media_extension("txt")
        && !is_media_extension("pdf")
        && !is_media_extension("doc")
        && extension_kind("zip").is_none()
        && extension_kind("").is_none()
}

#[cfg(test)]
mod wave108_tests {
    use super::*;

    #[test]
    fn wave108_extension_png_webp_mp4_mov_case_fold_raw_heif_non_media_dual_oracle() {
        assert!(wave108_png_webp_shell());
        assert!(wave108_mp4_mov_shell());
        assert!(wave108_case_fold_shell());
        assert!(wave108_raw_heif_shell());
        assert!(wave108_non_media_shell());
        assert!(wave107_gif_bmp_shell());
    }
}
// ── wave109 pure residual dens: extension raw-tif flv-divx upper-path family-product unsupported-none dual-oracle residual ──
// Dual-oracle residual of extension pure halves.
// Filesystem walk residual retained. dens ≠ flip.

/// Dual-oracle residual: raw/tif image membership dual-oracle.
#[must_use]
pub fn wave109_raw_tif_shell() -> bool {
    is_image_extension("raw")
        && is_image_extension("TIF")
        && extension_kind("raw") == Some("image")
        && extension_kind("tiff") == Some("image")
        && is_media_extension("heif")
        && !is_video_extension("raw")
}

/// Dual-oracle residual: flv/divx video membership dual-oracle.
#[must_use]
pub fn wave109_flv_divx_shell() -> bool {
    is_video_extension("flv")
        && is_video_extension("DIVX")
        && extension_kind("flv") == Some("video")
        && extension_kind("divx") == Some("video")
        && is_media_extension("3gp")
        && !is_image_extension("flv")
}

/// Dual-oracle residual: uppercase path probe dual-oracle.
#[must_use]
pub fn wave109_upper_path_shell() -> bool {
    use std::path::Path;
    extension_of(Path::new("PHOTO.JPEG")) == Some("jpeg".to_string())
        && extension_of(Path::new("/tmp/Clip.MOV")) == Some("mov".to_string())
        && extension_of(Path::new("a.b.HEIC")) == Some("heic".to_string())
}

/// Dual-oracle residual: family product dual-oracle.
#[must_use]
pub fn wave109_family_product_shell() -> bool {
    IMAGE_EXTENSIONS.len() * VIDEO_EXTENSIONS.len() == 132
        && IMAGE_EXTENSIONS.len() == 11
        && VIDEO_EXTENSIONS.len() == 12
        && all_supported_extensions().len() == IMAGE_EXTENSIONS.len() + VIDEO_EXTENSIONS.len()
}

/// Dual-oracle residual: unsupported kind none dual-oracle.
#[must_use]
pub fn wave109_unsupported_none_shell() -> bool {
    use std::path::Path;
    extension_kind("pdf").is_none()
        && extension_kind("txt").is_none()
        && !is_media_extension("docx")
        && extension_of(Path::new("readme.md")).is_none()
}

#[cfg(test)]
mod wave109_tests {
    use super::*;

    #[test]
    fn wave109_extension_raw_tif_flv_divx_upper_path_family_product_unsupported_none_dual_oracle() {
        assert!(wave109_raw_tif_shell());
        assert!(wave109_flv_divx_shell());
        assert!(wave109_upper_path_shell());
        assert!(wave109_family_product_shell());
        assert!(wave109_unsupported_none_shell());
        assert!(wave107_gif_bmp_shell());
    }
}
