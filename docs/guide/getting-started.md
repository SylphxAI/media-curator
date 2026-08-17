# Quick Start Guide

This guide walks you through the basic usage of MediaCurator (`media-curator`) to organize your media files and handle duplicates.

## Understanding the Command

The basic command structure is:

```bash
MediaCurator <source> <destination> [options]
```

- `<source>`: The source directory to admit into the library for this run.
- `<destination>`: The main directory where your organized, unique media files will be placed.
- `[options]`: Flags to customize the behavior, such as how files are named/structured (`--format`), how duplicates are handled (`-d`, `--move`), and processing sensitivity (e.g., `--image-similarity-threshold`).

## Example 1: Simple Organization (Copy Mode)

Let's say you have photos in `/media/incoming/photos`. You want to organize them into `/library/main_collection` based on the year and month they were created.

```bash
MediaCurator /media/incoming/photos /library/main_collection --format "{D.YYYY}/{D.MM}/{NAME}{EXT}"
```

**What happens?**

1.  **Discovery:** MediaCurator scans the source directories for media files.
2.  **Gathering:** It extracts metadata (like dates) and calculates perceptual hashes for each file.
3.  **Deduplication:** It checks for duplicates based on default similarity thresholds.
4.  **Transfer (Copy):**
    - For each _unique_ file, it determines the target path using the `--format` string (e.g., `/library/main_collection/2023/04/MyPhoto.jpg`).
    - It **copies** the unique file from the source to the calculated destination path. The original file remains in the source directory.
    - Duplicate files are identified but **ignored** (not copied to the destination) by default.

**Key Points:**

- By default, `media-curator` operates in **copy mode**. Originals are left untouched in the source directories.
- The `{D.YYYY}` and `{D.MM}` placeholders use the EXIF date if available, falling back to the file's creation date otherwise.
- Duplicates (based on default settings) are skipped and not copied to the destination.

## Review Before Applying

For a first pass over a valuable library, export the recommendations before
changing any media. The plan contains the selected organization actions,
duplicate and error routing, the reason for each recommendation, and a Rust
fingerprint of every source file.

```bash
MediaCurator /media/incoming/photos /library/main_collection \
    --format "{D.YYYY}/{D.MM}/{NAME}{EXT}" \
    -d /library/duplicates \
    -e /library/errors \
    --plan /private/media-curator-plan.json
```

Inspect the JSON plan and set `review.approved` to `true` only after the
recommendations are accepted. Then apply it:

```bash
MediaCurator --apply-plan /private/media-curator-plan.json
```

Applying is fail-closed: it refuses an unapproved plan, a changed source, a
path outside the declared roots, or a destination containing different bytes.
Progress is recorded in a sibling `.journal.json` file, so rerunning
`--apply-plan` safely resumes an interrupted copy or move.

## Example 2: Organizing and Moving Duplicates

Now, let's organize the same source, but this time we want to:

- **Move** the unique files to the destination instead of copying them.
- Move all identified **duplicates** to a separate `/library/duplicates` folder.

```bash
MediaCurator /media/incoming/photos /library/main_collection \
    --format "{D.YYYY}/{D.MM}/{NAME}{EXT}" \
    -d /library/duplicates \
    --move
```

**What happens differently?**

1.  Discovery, Gathering, and Deduplication proceed as before.
2.  **Transfer (Move):**
    - Unique files are **moved** from their source directory to the calculated destination path (e.g., `/library/main_collection/2023/04/MyPhoto.jpg`). They are removed from the source.
    - Duplicate files (all copies except the one chosen as the 'original' for the destination) are **moved** from their source directory to the specified duplicates directory (`/library/duplicates`). They are also removed from the source.

**Key Points:**

- The `--move` flag changes the behavior from copying to moving for _both_ unique and duplicate files.
- The `-d /library/duplicates` flag tells `media-curator` where to put the identified duplicates. Without `-d`, duplicates would simply be left in the source (if copying) or deleted (if moving unique files, which is generally not recommended without `-d`).

## Next Steps

You've seen the basics! Now you can:

- Dive deeper into the powerful [Organization Format String](./format-string.md) options.
- Understand the nuances of the [Deduplication Strategy](./deduplication.md) and how to adjust its sensitivity.
- Explore all available command-line options by running `media-curator --help` in your terminal.
