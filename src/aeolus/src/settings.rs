use core::fmt;
use std::path::{PathBuf, Path};
use std::env;
use std::fs;
use std::str::FromStr;

use serde_derive::{Serialize, Deserialize};
use clap::ValueEnum;
use rlua::{UserData, Table, Value};

use crate::cli::Cli;
use config::{Config, ConfigError, File};
use common::{DynamicResult, unit::RefDim};
use grid::block::BlockCollection;

#[derive(Debug)]
pub struct InvalidConfig;

/// Simulation configuration
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SimSettings {
    reference_dimensions: RefDim,

    #[serde(skip)]
    grids: BlockCollection,
}

impl UserData for SimSettings {}

impl SimSettings { 
    pub fn from_lua_table(config: Table) -> Result<SimSettings, InvalidConfig> {
        // first check to make sure there are no invalid names in the table
        // this ensures the user doesn't misspell something, and unknowingly
        // get the default value
        let allowable_names = ["reference_values", "blocks"];
        for pair in config.clone().pairs::<String, Value>() {
            let (key, _) = pair.unwrap();
            if !allowable_names.contains(&key.as_str()) {
                return Err(InvalidConfig);
            }
        }

        // read the configuration
        let reference_dimensions = config.get::<_, RefDim>("reference_values").unwrap();

        // the grid
        let grids = config.get::<_, BlockCollection>("blocks").unwrap();

        Ok(SimSettings{
            reference_dimensions, grids,
        })
    }

    pub fn write_config(&self, file_structure: &FileStructure) -> DynamicResult<()> {
        // write the config file
        let config_toml = toml::to_string(self).unwrap();
        fs::write(file_structure.config(), config_toml).unwrap();

        // write the initial conditions
        self.write_initial_conditions(file_structure)?;

        Ok(())
    }

    fn write_initial_conditions(&self, file_structure: &FileStructure) -> DynamicResult<()> {
        self.write_initial_grid(file_structure) 
    }

    fn write_initial_grid(&self, file_structure: &FileStructure) -> DynamicResult<()> {
        let mut dir = file_structure.grid().to_path_buf(); 
        dir.push(&PathBuf::from_str("t0000").unwrap());
        create_parent_directory(&dir);
        self.grids.write_blocks(&dir)?;
        Ok(())
    }
}

/// Configuration for the program
#[derive(Debug, Serialize, Deserialize)]
pub struct AeolusSettings {
    verbosity: Verbosity,
    file_structure: FileStructure,
}

impl AeolusSettings {
    pub fn new(args: &Cli) -> Result<AeolusSettings, ConfigError> {
        // where to look for default config
        let aeolus_home = env::var("AEOLUS_HOME").unwrap_or_else(|_| ".".into());
        let aeolus_default_path = format!("{}/resources/defaults/aeolus_defaults.toml", aeolus_home);
        // begin configuring from files
        let s = Config::builder()
            .add_source(File::from(Path::new(&aeolus_default_path)))
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
    config: PathBuf,
    solver: PathBuf,
    discretisation: PathBuf,
    grid: PathBuf,
    fluid:  PathBuf,
}

fn create_parent_directory(dir: &Path) {
    fs::create_dir_all(
        dir.parent()
            .unwrap()
            .as_os_str()
    ).unwrap();
}

impl FileStructure {
    pub fn create_directories(&self) {
        create_parent_directory(&self.solver);
        create_parent_directory(&self.discretisation);
        create_parent_directory(&self.grid);
        create_parent_directory(&self.fluid);
    }

    pub fn solver(&self) -> &Path {
        &self.solver
    }

    pub fn discretisation(&self) -> &Path {
        &self.discretisation
    }

    pub fn config(&self) -> &Path {
        &self.config
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
