//! media-curator-cli binary — health + pure-core subcommands.

use std::io::{self, Write};
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use media_curator_cli::hamming::hamming_distance;
use media_curator_cli::health_json;

#[derive(Parser)]
#[command(
    name = "media-curator-cli",
    version,
    about = "Media Curator Rust CLI (ADR-168 health + pure cores)"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Dependency-free health probe (S0).
    Health,
    /// Hamming distance between two hex-encoded hashes.
    Hamming {
        /// First hash as hex.
        a: String,
        /// Second hash as hex.
        b: String,
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
        None => {
            eprintln!("media-curator-cli: no subcommand specified; use `health`, `hamming`, or `--help`");
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
    if s.len() % 2 != 0 {
        return Err(());
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|_| ()))
        .collect()
}
