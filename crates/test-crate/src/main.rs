//! test-crate CLI
#![forbid(unsafe_code)]

use clap::Parser;
use test_crate::{Cli, Commands, commands};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Change directory if requested
    if let Some(ref dir) = cli.chdir {
        std::env::set_current_dir(dir)?;
    }

    // Execute command
    match cli.command {
        Commands::Info(args) => commands::info::cmd_info(args),
    }
}
