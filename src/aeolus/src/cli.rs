use std::path::PathBuf;

use clap::{Parser, Subcommand};

use super::settings::Verbosity;

#[derive(Debug, Parser)]
#[command(about, version)]
pub struct Cli {
    #[arg(short, long)]
    pub verbosity: Option<Verbosity>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Prepare simulation files
    #[command(arg_required_else_help = true)]
    Prep {
        /// The file defining the simulation
        prep_file: PathBuf
    },

    /// Run a simulation
    Run{
        start_time_index: Option<usize>
    },

    /// Clean simulation files
    Clean,
}
