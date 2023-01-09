use core::fmt;
use std::path::{PathBuf, Path};
use std::env;

use serde_derive::{Serialize, Deserialize};
use clap::ValueEnum;

use crate::cli::Cli;
use config::{Config, ConfigError, File};

/// Configuration for the program
#[derive(Debug, Serialize, Deserialize)]
pub struct AeolusSettings {
    verbosity: Verbosity,
    file_structure: FileStructure,
}

impl AeolusSettings {
    pub fn new(args: &Cli) -> Result<AeolusSettings, ConfigError> {
        // where to look for default config
        let aeolus_home = env::var("AEOLUS_HOME").unwrap_or_else(|_| "aeolus/src".into());

        // begin configuring from files
        let s = Config::builder()
            .add_source(File::from(Path::new(&format!("{}/default.toml", aeolus_home))))
            .add_source(File::from(Path::new("local.toml")).required(false))
            .set_override_option("verbosity", args.verbosity.as_ref().map(|v| v.to_string()))?;

        // Attempt to read the configuration
        s.build()?.try_deserialize()
    }

    pub fn verbosity(&self) -> &Verbosity {
        &self.verbosity
    }

    pub fn file_structure(&self) -> &FileStructure {
        &self.file_structure
    }
}

/// The location the program should look
/// for different parts of the configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct FileStructure {
    solver: PathBuf,
    discretisation: PathBuf,
    gas_model: PathBuf,
    units: PathBuf,
    grid: PathBuf,
    fluid:  PathBuf,
}

impl FileStructure {
    pub fn solver(&self) -> &Path {
        &self.solver
    }

    pub fn discretisation(&self) -> &Path {
        &self.discretisation
    }

    pub fn gas_model(&self) -> &Path {
        &self.gas_model
    }

    pub fn grid(&self) -> &Path {
        &self.grid
    }

    pub fn fluid(&self) -> &Path {
        &self.fluid
    }

    pub fn units(&self) -> &Path {
        &self.units
    }
}

#[derive(Debug, Serialize, Deserialize, ValueEnum, Clone)]
pub enum Verbosity {
    Error, Warning, Debug 
}

impl std::fmt::Display for Verbosity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Verbosity::Error => write!(f, "Error"),
            Verbosity::Warning => write!(f, "Warning"),
            Verbosity::Debug => write!(f, "Debug"),
        }
    }
}
