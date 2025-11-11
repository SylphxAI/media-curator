<div align="center">

# Media Curator 📸

**Intelligent media library management - LSH-based deduplication with blazing-fast processing**

[![npm version](https://img.shields.io/npm/v/@sylphlab/media-curator?style=flat-square)](https://www.npmjs.com/package/@sylphlab/media-curator)
[![CI Status](https://img.shields.io/github/actions/workflow/status/SylphxAI/media-curator/ci.yml?style=flat-square)](https://github.com/SylphxAI/media-curator/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/badge/coverage-100%25-brightgreen?style=flat-square)](https://github.com/SylphxAI/media-curator)
[![License](https://img.shields.io/badge/License-MIT-blue?style=flat-square)](https://opensource.org/licenses/MIT)

**LSH deduplication** • **WASM performance** • **Parallel processing** • **Metadata extraction**

[Quick Start](#-quick-start) • [Installation](#-installation) • [Features](#-key-features)

</div>

---

## 🚀 Overview

Intelligently curate, organize, and deduplicate your digital photo and video collection. Built with performance, scalability, and robustness in mind using modern TypeScript, WebAssembly, and native libraries.

**The Problem:**
```
Managing large media libraries:
- Scattered files, no organization ❌
- Duplicate accumulation (storage waste) ❌
- Manual organization (time-consuming) ❌
- Simple hash matching (misses similar files) ❌
```

**The Solution:**
```
Media Curator:
- Auto-organize by date/camera/location ✅
- LSH-based visual similarity detection ✅
- Customizable folder structure ✅
- Perceptual hashing (finds variants) ✅
```

**Result: Organized, deduplicated media library with intelligent visual similarity detection.**

---

## ⚡ Performance Advantages

### Speed & Efficiency

| Feature | Traditional Tools | Media Curator |
|---------|------------------|---------------|
| **Deduplication** | ❌ Hash-based only | ✅ LSH perceptual hashing |
| **Similarity Detection** | ❌ Exact matches | ✅ Visual similarity (variants) |
| **Performance** | ⚠️ Single-threaded | ✅ Parallel processing (workerpool) |
| **Metadata** | ⚠️ In-memory | ✅ SQLite (millions of files) |
| **Caching** | ❌ Re-process every run | ✅ LMDB pause/resume |
| **Calculations** | ⚠️ JavaScript | ✅ WebAssembly (Hamming distance) |

### Technology Performance

- **Native Libraries** - Sharp (libvips) + FFmpeg for maximum speed
- **WebAssembly** - Optimized Hamming distance calculations
- **Worker Pools** - Parallel pHash processing (multi-core)
- **SQLite** - Fast metadata queries for millions of files
- **LMDB Cache** - Persistent intermediate results

---

## 🎯 Key Features

### Intelligent Organization

**Smart Folder Structure:**
- **Date-based** - EXIF date (falls back to file date)
- **Camera model** - Group by device
- **Geolocation** - GPS-tagged photos
- **File type** - Separate images/videos
- **Custom formats** - Flexible placeholder system

**Example Format String:**
```bash
# Organize: Year > Month > Type > Filename
--format "{D.YYYY}/{D.MMMM}/{TYPE}/{NAME}_{RND}{EXT}"

# Result: 2023/April/Image/IMG_1234_a1b2c3d4.jpg
```

### Advanced Deduplication

**Beyond Simple Hashing:**
- **Perceptual Hashing (pHash)** - Detects visually similar files
- **LSH (Locality-Sensitive Hashing)** - Efficient similarity search
- **Configurable Thresholds** - Control sensitivity
- **Multi-format Support** - Images and videos

**Detects:**
- Exact duplicates (same hash)
- Resized versions
- Minor edits (crop, filter, compression)
- Different formats (JPG vs PNG)
- Video frame similarity

### Scalable Architecture

**Database-Centric Design:**
- **SQLite** - Metadata + LSH hashes for millions of files
- **LMDB** - Fast key-value cache for intermediate results
- **Low Memory** - No need to load entire library into RAM
- **Pause/Resume** - Cache enables quick restarts

**Performance Optimization:**
- **Workerpool** - Parallel pHash computation
- **WASM** - Fast Hamming distance (AssemblyScript)
- **Batch Processing** - Efficient file handling
- **Concurrent Workers** - Customizable worker count

---

## 📦 Installation

### Global Installation (Recommended)

```bash
# Install via Bun
bun install --global @sylphlab/media-curator

# Install via npm
npm install --global @sylphlab/media-curator

# Verify installation
media-curator --help
```

### Prerequisites

- **Node.js** ≥18.0.0 or **Bun** ≥0.5.0
- **FFmpeg** - For video processing
- **ExifTool** - For metadata extraction (optional, bundled)

**Install FFmpeg:**

```bash
# macOS
brew install ffmpeg

# Ubuntu/Debian
sudo apt-get install ffmpeg

# Windows
# Download from https://ffmpeg.org/download.html
```

---

## 🚀 Quick Start

### Basic Organization

Organize photos from one directory to another:

```bash
media-curator /media/photos /library/organized
```

### With Deduplication

Organize and separate duplicates:

```bash
media-curator /media/photos /library/organized \
  -d /library/duplicates \
  -e /library/errors
```

### Custom Format String

Organize by year and month:

```bash
media-curator /media/photos /library/organized \
  --format "{D.YYYY}/{D.MM}/{NAME}{EXT}"
```

### Full Example

Complete workflow with all options:

```bash
media-curator /media/photos /media/downloads /library/organized \
  -d /library/duplicates \
  -e /library/errors \
  --move \
  --resolution 128 \
  --image-similarity-threshold 0.95 \
  --video-similarity-threshold 0.90 \
  --format "{D.YYYY}/{D.MMMM}/{TYPE}/{NAME}_{RND}{EXT}" \
  --concurrency 8 \
  --verbose
```

---

## 🛠️ CLI Options

### Core Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `<source...>` | ✅ | Source directories or files (multiple allowed) |
| `<destination>` | ✅ | Destination directory for organized files |

### Essential Options

| Option | Default | Description |
|--------|---------|-------------|
| `-d, --duplicate <path>` | None | Directory for duplicate files |
| `-e, --error <path>` | None | Directory for files with processing errors |
| `-m, --move` | `false` | Move files instead of copying |
| `-v, --verbose` | `false` | Enable detailed logging |

### Deduplication Options

| Option | Default | Description |
|--------|---------|-------------|
| `--image-similarity-threshold <n>` | `0.99` | Image similarity threshold (0-1) |
| `--video-similarity-threshold <n>` | `0.93` | Video similarity threshold (0-1) |
| `--image-video-similarity-threshold <n>` | `0.93` | Cross-type similarity threshold |
| `-r, --resolution <n>` | `64` | pHash resolution (higher = more accurate) |

### Performance Options

| Option | Default | Description |
|--------|---------|-------------|
| `-c, --concurrency <n>` | CPU cores - 1 | Number of worker processes |
| `--max-chunk-size <n>` | `2MB` | Maximum file processing chunk size |

### Video Processing Options

| Option | Default | Description |
|--------|---------|-------------|
| `--target-fps <n>` | `2` | Target FPS for video frame extraction |
| `--min-frames <n>` | `5` | Minimum frames to extract |
| `--max-scene-frames <n>` | `100` | Maximum frames per scene |
| `--scene-change-threshold <n>` | `0.01` | Scene change detection threshold |
| `-w, --window-size <n>` | `5` | Frame clustering window size |
| `-p, --step-size <n>` | `1` | Frame clustering step size |

### Organization Options

| Option | Default | Description |
|--------|---------|-------------|
| `-F, --format <string>` | (see below) | Destination path format string |
| `--debug <path>` | None | Directory for debug reports |

---

## 📝 Format String Placeholders

### Date Placeholders

**Prefixes:**
- `I.` - Image Date (from EXIF)
- `F.` - File Creation Date
- `D.` - Mixed Date (prefers EXIF, falls back to file)

**Patterns:**

| Placeholder | Example | Description |
|-------------|---------|-------------|
| `{?.YYYY}` | `2023` | 4-digit year |
| `{?.YY}` | `23` | 2-digit year |
| `{?.MMMM}` | `January` | Full month name |
| `{?.MMM}` | `Jan` | Short month name |
| `{?.MM}` | `01` | Month (zero-padded) |
| `{?.M}` | `1` | Month (no padding) |
| `{?.DD}` | `05` | Day (zero-padded) |
| `{?.D}` | `5` | Day (no padding) |
| `{?.DDDD}` | `Sunday` | Full weekday name |
| `{?.DDD}` | `Sun` | Short weekday name |
| `{?.HH}` | `14` | 24-hour (zero-padded) |
| `{?.hh}` | `02` | 12-hour (zero-padded) |
| `{?.mm}` | `08` | Minute (zero-padded) |
| `{?.ss}` | `09` | Second (zero-padded) |
| `{?.a}` | `am` | Lowercase am/pm |
| `{?.A}` | `AM` | Uppercase AM/PM |
| `{?.WW}` | `01` | Week number (01-53) |

### Filename Placeholders

| Placeholder | Example | Description |
|-------------|---------|-------------|
| `{NAME}` | `IMG_1234` | Original filename (no extension) |
| `{NAME.L}` | `img_1234` | Lowercase filename |
| `{NAME.U}` | `IMG_1234` | Uppercase filename |
| `{EXT}` | `.jpg` | File extension (with dot) |
| `{RND}` | `a1b2c3d4` | Random 8-char hex (prevents collisions) |

### Metadata Placeholders

| Placeholder | Example | Description |
|-------------|---------|-------------|
| `{GEO}` | `34.05_-118.24` | GPS coordinates (if available) |
| `{CAM}` | `iPhone 14 Pro` | Camera model (if available) |
| `{TYPE}` | `Image` or `Video` | File type |

### Conditional Placeholders

| Placeholder | Values | Description |
|-------------|--------|-------------|
| `{HAS.GEO}` | `GeoTagged` or `NoGeo` | Has GPS data? |
| `{HAS.CAM}` | `WithCamera` or `NoCamera` | Has camera metadata? |
| `{HAS.DATE}` | `Dated` or `NoDate` | Has EXIF date? |

### Format Examples

```bash
# By year and month
"{D.YYYY}/{D.MM}/{NAME}{EXT}"
# → 2023/04/IMG_1234.jpg

# By camera model
"{CAM}/{D.YYYY}/{NAME}{EXT}"
# → iPhone 14 Pro/2023/IMG_1234.jpg

# With geolocation
"{HAS.GEO}/{GEO}/{D.YYYY}-{D.MM}/{NAME}{EXT}"
# → GeoTagged/34.05_-118.24/2023-04/IMG_1234.jpg

# Prevent collisions
"{D.YYYY}/{D.MMMM}/{TYPE}/{NAME}_{RND}{EXT}"
# → 2023/April/Image/IMG_1234_a1b2c3d4.jpg
```

---

## 💡 Advanced Usage Examples

### 1. Dry Run (Debug Mode)

Test organization without moving files:

```bash
media-curator /media/photos /library/organized \
  --debug /tmp/curator_debug \
  --format "{D.YYYY}-{D.MM}/{TYPE}/{NAME}{EXT}"
```

No files are moved/copied, but a report is generated showing:
- What would happen
- Potential duplicates
- Metadata extraction results

### 2. High-Sensitivity Deduplication

For archival use cases, increase sensitivity:

```bash
media-curator /archive_source /library/organized \
  -d /library/duplicates \
  --move \
  --resolution 128 \
  --image-similarity-threshold 0.95 \
  --video-similarity-threshold 0.90 \
  --verbose
```

### 3. Organize by Camera and Date

Group photos by camera model:

```bash
media-curator /camera_roll /library/by_camera \
  --format "{HAS.CAM}/{CAM}/{D.YYYY}-{D.MM}/{NAME}_{RND}{EXT}" \
  --verbose
```

**Result:**
```
WithCamera/iPhone 14 Pro/2023-10/IMG_001_abc123ef.jpg
NoCamera/Unknown/2024-01/video_clip_xyz98765.mp4
```

### 4. Process Specific File Types

Using shell globbing to filter:

```bash
# Only JPG files
media-curator /media/photos/**/*.jpg /library/organized_jpgs

# Only MP4 videos
media-curator /media/videos/**/*.mp4 /library/organized_mp4s
```

### 5. Maximum Performance

Utilize all CPU cores with higher resolution:

```bash
media-curator /massive_library /organized \
  -d /duplicates \
  --move \
  --concurrency 16 \
  --resolution 128 \
  --target-fps 4 \
  --verbose
```

---

## 🏗️ Architecture

### Technology Stack

| Component | Technology | Purpose |
|-----------|-----------|---------|
| **Language** | TypeScript | Type-safe development |
| **Runtime** | Node.js / Bun | Execution environment |
| **Image Processing** | Sharp (libvips) | Fast image operations |
| **Video Processing** | FFmpeg | Video frame extraction |
| **Metadata** | ExifTool | EXIF/GPS extraction |
| **Database** | SQLite (better-sqlite3) | Metadata + LSH storage |
| **Cache** | LMDB | Fast key-value cache |
| **Optimization** | WebAssembly (AssemblyScript) | Hamming distance |
| **Concurrency** | workerpool | Parallel processing |
| **Error Handling** | neverthrow | Result types |

### Pipeline Architecture

```
┌─────────────────────────────────────────────────────────┐
│ 1. Discovery                                            │
│    • Scan source directories                           │
│    • Collect file paths                                │
└─────────────────┬───────────────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────────────┐
│ 2. Gatherer (Parallel Processing)                      │
│    • Extract metadata (EXIF, GPS, camera)              │
│    • Generate pHash (via workerpool)                   │
│    • Store in SQLite + LMDB cache                      │
│    • Tools: Sharp, FFmpeg, ExifTool, WASM             │
└─────────────────┬───────────────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────────────┐
│ 3. Deduplicator (LSH-based)                            │
│    • Query SQLite for similarity candidates            │
│    • Calculate Hamming distance (WASM)                 │
│    • Group duplicate sets                              │
│    • Identify unique files                             │
└─────────────────┬───────────────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────────────┐
│ 4. Transfer                                             │
│    • Organize files by format string                   │
│    • Move/copy unique files to destination             │
│    • Move duplicates to duplicate directory            │
│    • Generate debug reports                            │
└─────────────────────────────────────────────────────────┘
```

**Key Design Principles:**
- **Functional Programming** - Pure functions, immutability, composition
- **Manual Dependency Injection** - Testable, maintainable architecture
- **Result Types** - Explicit error handling via `neverthrow`
- **Minimal Dependencies** - Prefer built-in APIs

---

## 🎯 Use Cases

### Personal Photo Libraries
- **Vacation photos** - Organize by date and location
- **Family events** - Group by camera (different devices)
- **Digital cleanup** - Remove duplicate photos from phone backups
- **Archival** - High-sensitivity deduplication before long-term storage

### Professional Photography
- **Client sessions** - Organize by date and camera model
- **Event coverage** - Deduplicate similar shots (burst mode)
- **Portfolio management** - Find and remove similar images
- **Backup deduplication** - Clean up redundant backups

### Video Collections
- **Video library** - Organize by date and metadata
- **Duplicate detection** - Find visually similar video clips
- **Frame-based similarity** - Detect re-encoded videos
- **Storage optimization** - Remove duplicate/similar videos

---

## 🔧 Development

### Setup

```bash
# Clone repository
git clone https://github.com/SylphxAI/media-curator.git
cd media-curator

# Install dependencies
bun install

# Build
bun run build
```

### Quality Checks

```bash
# Lint
bun run lint

# Format
bun run format

# Type check
bun run typecheck

# Test
bun test

# Test with coverage
bun run test:cov

# Validate all
bun run validate
```

### Run Locally

```bash
# Development mode
bun run start

# Production build
bun run build
bun run start:node
```

---

## 📊 Performance & Quality

### Test Coverage

- **100% coverage** enforced via CI
- Unit tests for all core modules
- Integration tests for pipeline stages
- Mock-based testing for external tools

### Code Quality

- **ESLint** - Strict rules for consistency
- **Prettier** - Automated formatting
- **TypeScript** - Strict mode type safety
- **Husky** - Pre-commit hooks

### Performance Characteristics

**Tested with:**
- Large libraries (10,000+ files)
- Mixed photo/video collections
- Multiple source directories
- Various file formats

**Optimizations:**
- Worker pool parallelism
- WASM-accelerated calculations
- SQLite indexing for fast queries
- LMDB caching for pause/resume

---

## 🗺️ Roadmap

**✅ Completed**
- [x] LSH-based perceptual hashing
- [x] SQLite metadata storage
- [x] LMDB caching
- [x] WebAssembly optimization
- [x] Parallel processing (workerpool)
- [x] FFmpeg + Sharp integration
- [x] Customizable format strings
- [x] CLI progress indicators

**🚀 Planned**
- [ ] Performance benchmarks (quantified metrics)
- [ ] Web UI for visual duplicate review
- [ ] Cloud storage integration (S3, Google Photos)
- [ ] Machine learning-based similarity (neural embeddings)
- [ ] Incremental indexing (watch mode)
- [ ] Face detection grouping
- [ ] Advanced filtering options

---

## 🤝 Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

**Development Guidelines:**
1. **Open an issue** - Discuss changes before implementing
2. **Fork the repository**
3. **Create a feature branch** - `git checkout -b feature/my-feature`
4. **Follow code standards** - Run `bun run validate`
5. **Write tests** - Maintain 100% coverage
6. **Submit a pull request**

**Note:** Some tests may fail under `bun test` due to complex mocking. See `memory-bank/progress.md` for details.

---

## 🤝 Support

[![npm](https://img.shields.io/npm/v/@sylphlab/media-curator?style=flat-square)](https://www.npmjs.com/package/@sylphlab/media-curator)
[![GitHub Issues](https://img.shields.io/github/issues/SylphxAI/media-curator?style=flat-square)](https://github.com/SylphxAI/media-curator/issues)

- 🐛 [Bug Reports](https://github.com/SylphxAI/media-curator/issues)
- 💬 [Discussions](https://github.com/SylphxAI/media-curator/discussions)
- 📧 [Email](mailto:hi@sylphx.com)

**Show Your Support:**
⭐ Star • 👀 Watch • 🐛 Report bugs • 💡 Suggest features • 🔀 Contribute

---

## 📄 License

MIT © [Sylphx](https://sylphx.com)

---

## 🙏 Credits

Built with:
- [Sharp](https://sharp.pixelplumbing.com/) - High-performance image processing (libvips)
- [FFmpeg](https://ffmpeg.org/) - Video frame extraction
- [SQLite](https://www.sqlite.org/) - Metadata storage (better-sqlite3)
- [LMDB](http://www.lmdb.tech/) - Fast key-value cache
- [ExifTool](https://exiftool.org/) - Metadata extraction
- [WebAssembly](https://webassembly.org/) - Optimized calculations
- [TypeScript](https://typescriptlang.org) - Type safety

Special thanks to the open source community ❤️

---

## 📚 Additional Resources

- [ARCHITECTURE.md](ARCHITECTURE.md) - Detailed architecture documentation
- [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines
- [memory-bank/progress.md](memory-bank/progress.md) - Development progress

---

<p align="center">
  <strong>Organize. Deduplicate. Optimize.</strong>
  <br>
  <sub>Intelligent media library management with visual similarity detection</sub>
  <br><br>
  <a href="https://sylphx.com">sylphx.com</a> •
  <a href="https://x.com/SylphxAI">@SylphxAI</a> •
  <a href="mailto:hi@sylphx.com">hi@sylphx.com</a>
</p>
