use std::path::{PathBuf, Path};
use std::env;

use serde_derive::{Serialize, Deserialize};
use clap::ArgMatches;

use config::{Config, ConfigError, File};

/// Configuration for the program
#[derive(Debug, Serialize, Deserialize)]
pub struct AeolusSettings {
    verbosity: Verbosity,
    file_structure: FileStructure,
}

impl AeolusSettings {
    pub fn new(cli: &ArgMatches) -> Result<AeolusSettings, ConfigError> {
        // where to look for default config
        let aeolus_home = env::var("AEOLUS_HOME").unwrap_or_else(|_| "src".into());

        // begin configuring from files
        let s = Config::builder()
            .add_source(File::with_name(&format!("{}/config/default.toml", aeolus_home)))
            .add_source(File::with_name("local.toml").required(false));

        // override config with command line options
        let verbosity = cli.get_one::<String>("verbosity");
        let s = match verbosity {
            Some(verb) => s.set_override("verbosity", verb.to_string())?,
            None => s,
        };

        // we're done
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

#[derive(Debug, Serialize, Deserialize)]
pub enum Verbosity {
    Error, Warning, Debug 
}
