use std::path::{PathBuf, Path};
use serde_derive::{Serialize, Deserialize};
use config::{Config, ConfigError, File};
use std::env;

/// Configuration for the program
#[derive(Debug, Serialize, Deserialize)]
pub struct AeolusSettings {
    verbosity: Verbosity,
    file_structure: FileStructure,
}

impl AeolusSettings {
    pub fn new() -> Result<AeolusSettings, ConfigError> {
        let aeolus_home = env::var("AEOLUS_HOME").unwrap_or_else(|_| "src".into());
        let s = Config::builder()
            .add_source(File::with_name(&format!("{}/config/default.toml", aeolus_home)))
            .add_source(File::with_name("local.toml").required(false))
            .build()?;
        s.try_deserialize()
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
