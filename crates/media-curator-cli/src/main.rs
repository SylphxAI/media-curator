//! media-curator-cli binary — S0 `health` subcommand + `--version`.

use std::io::{self, Write};
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use media_curator_cli::health_json;

#[derive(Parser)]
#[command(name = "media-curator-cli", version, about = "Media Curator Rust CLI (ADR-168 S0 health + version)")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Dependency-free health probe (S0).
    Health,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.command {
        Some(Command::Health) => {
            if let Err(error) = writeln!(io::stdout(), "{}", health_json()) {
                eprintln!("write health: {error}");
                return ExitCode::from(1);
            }
            ExitCode::SUCCESS
        }
        None => {
            eprintln!("media-curator-cli: no subcommand specified; use `health` or `--help`");
            ExitCode::from(2)
        }
    }
}
