use core::fmt;
use std::path::{PathBuf, Path};
use std::env;

use serde_derive::{Serialize, Deserialize};
use clap::ValueEnum;

use crate::config::cli::Cli;
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
        let aeolus_home = env::var("AEOLUS_HOME").unwrap_or_else(|_| "src".into());

        // begin configuring from files
        let s = Config::builder()
            .add_source(File::from(Path::new(&format!("{}/config/default.toml", aeolus_home))))
            .add_source(File::from(Path::new("local.toml")).required(false));

        // configure from command line
        // there is a method 'set_override_option' which would allow
        // stringing this onto the above methods, but it doesn't seem
        // to support enum values.
        let s = match &args.verbosity {
            Some(verb) => s.set_override("verbosity", verb.to_string())?,
            None => s,
        };

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
