// src/main.rs
/*
 * Main executable for CognitiveWeave
 */

use clap::Parser;
use cognitiveweave::{Result, run};

#[derive(Parser)]
#[command(version, about = "CognitiveWeave - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
