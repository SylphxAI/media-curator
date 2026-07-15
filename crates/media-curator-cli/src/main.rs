//! media-curator-cli binary — production subcommands for health + pure cores.

use std::io::{self, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use media_curator_cli::discovery::{discover_files_json, DiscoverOptions};
use media_curator_cli::file_stats::file_stats_for_path;
use media_curator_cli::hamming::hamming_distance;
use media_curator_cli::dedup::cluster_exact_by_phash;
use media_curator_cli::health_json;

#[derive(Parser)]
#[command(
    name = "media-curator-cli",
    version,
    about = "Media Curator Rust CLI (ADR-168 production authority)"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Health probe (Rust authority).
    Health,
    /// Hamming distance between two hex-encoded hashes.
    Hamming {
        /// First hash as hex.
        a: String,
        /// Second hash as hex.
        b: String,
    },
    /// File stats (size + MD5) for a path.
    #[command(name = "file-stats")]
    FileStats {
        /// File path.
        path: PathBuf,
    },
    /// Discover media files under source directories.
    Discover {
        /// Source directories (repeatable).
        #[arg(long = "source", required = true)]
        sources: Vec<PathBuf>,
        /// Directory scan concurrency.
        #[arg(long, default_value_t = 4)]
        concurrency: usize,
    },
    /// Exact-duplicate clusters by identical pHash hex (JSON lines on stdin or --entry path=hex).
    #[command(name = "exact-dup")]
    ExactDup {
        /// Entry as path=phashHex (repeatable). Empty phash via path= 
        #[arg(long = "entry")]
        entries: Vec<String>,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.command {
        Some(Command::Health) => write_stdout(&health_json()),
        Some(Command::Hamming { a, b }) => match (hex_decode(&a), hex_decode(&b)) {
            (Ok(ba), Ok(bb)) => {
                let d = hamming_distance(&ba, &bb);
                write_stdout(&format!(r#"{{"distance":{d}}}"#))
            }
            _ => {
                eprintln!("invalid hex hash");
                ExitCode::from(1)
            }
        },
        Some(Command::FileStats { path }) => match file_stats_for_path(&path) {
            Ok(stats) => match serde_json::to_string(&stats) {
                Ok(json) => write_stdout(&json),
                Err(error) => {
                    eprintln!("serialize file-stats: {error}");
                    ExitCode::from(1)
                }
            },
            Err(error) => {
                eprintln!("file-stats: {error}");
                ExitCode::from(1)
            }
        },
        Some(Command::Discover {
            sources,
            concurrency,
        }) => match discover_files_json(DiscoverOptions {
            source_dirs: sources,
            concurrency,
        }) {
            Ok(json) => write_stdout(&json),
            Err(error) => {
                eprintln!("discover: {error}");
                ExitCode::from(1)
            }
        },
        Some(Command::ExactDup { entries }) => {
            let parsed: Vec<(String, Option<String>)> = entries
                .into_iter()
                .map(|raw| {
                    if let Some((path, hash)) = raw.split_once('=') {
                        let hash = hash.trim();
                        (
                            path.to_string(),
                            if hash.is_empty() {
                                None
                            } else {
                                Some(hash.to_string())
                            },
                        )
                    } else {
                        (raw, None)
                    }
                })
                .collect();
            let result = cluster_exact_by_phash(&parsed);
            match serde_json::to_string(&result) {
                Ok(json) => write_stdout(&json),
                Err(error) => {
                    eprintln!("exact-dup serialize: {error}");
                    ExitCode::from(1)
                }
            }
        }
        None => {
            eprintln!(
                "media-curator-cli: no subcommand specified; use `health`, `hamming`, `file-stats`, `discover`, `exact-dup`, or `--help`"
            );
            ExitCode::from(2)
        }
    }
}

fn write_stdout(payload: &str) -> ExitCode {
    if let Err(error) = writeln!(io::stdout(), "{payload}") {
        eprintln!("write stdout: {error}");
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}

fn hex_decode(s: &str) -> Result<Vec<u8>, ()> {
    if !s.len().is_multiple_of(2) {
        return Err(());
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|_| ()))
        .collect()
}
