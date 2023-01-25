use core::fmt;
use std::path::{PathBuf, Path};
use std::env;
use std::fs::{self, create_dir_all};
use std::str::FromStr;

use serde_derive::{Serialize, Deserialize};
use clap::ValueEnum;
use rlua::{UserData, Table, Value};

use crate::cli::Cli;
use crate::logging::{UserLogger, Logger};
use config::{Config, ConfigError, File};
use common::{DynamicResult, unit::RefDim};
use common::number::Real;
use grid::block::{BlockCollection, GridFileType};
use gas::gas_model::{GasModels, GasModel};
use gas::ideal_gas::IdealGas;


#[derive(Debug)]
pub struct InvalidConfig;

/// Simulation configuration
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SimSettings {
    gas_model_type: GasModels,

    reference_dimensions: RefDim,

    // these don't get written to the generic config file
    #[serde(skip)]
    gas_model: Box<dyn GasModel<Real>>,

    #[serde(skip)]
    grids: BlockCollection,
}


impl UserData for SimSettings {}

impl SimSettings { 
    pub fn from_lua_table(config: Table) -> Result<SimSettings, InvalidConfig> {
        // first check to make sure there are no invalid names in the table
        // this ensures the user doesn't misspell something, and unknowingly
        // get the default value
        let allowable_names = ["reference_values", "blocks", "gas_model_type", "gas_model"];
        for pair in config.clone().pairs::<String, Value>() {
            let (key, _) = pair.unwrap();
            if !allowable_names.contains(&key.as_str()) {
                return Err(InvalidConfig);
            }
        }

        // pull things out of the config table
        let reference_dimensions = config.get::<_, RefDim>("reference_values").unwrap();
        let grids = config.get::<_, BlockCollection>("blocks").unwrap();

        // read the gas model
        let gas_model_str = config.get::<_, String>("gas_model_type").unwrap();
        let gas_model_type = GasModels::from_str(&gas_model_str).unwrap();
        let gas_model: Box<dyn GasModel<Real>> = match gas_model_type {
            GasModels::IdealGas => Box::new(config.get::<_, IdealGas<Real>>("gas_model").unwrap()),
        };
        Ok(SimSettings{
            reference_dimensions, grids, gas_model_type, gas_model,
        })
    }

    pub fn write_config(&self, file_structure: &FileStructure) -> DynamicResult<()> {
        // write the config file
        let config_toml = toml::to_string(self).unwrap();
        fs::write(file_structure.config(), config_toml).unwrap();

        match self.gas_model_type {
            GasModels::IdealGas => {
                let ideal_gas: &IdealGas<Real> = self.gas_model.as_any().downcast_ref().unwrap();
                let ideal_gas_toml = toml::to_string(ideal_gas).unwrap();
                fs::write(file_structure.gas_model(), ideal_gas_toml).unwrap();
            }
        }

        self.write_initial_conditions(file_structure)?;


        Ok(())
    }
    
    fn write_initial_conditions(&self, file_structure: &FileStructure) -> DynamicResult<()> {
        self.write_initial_grid(file_structure) 
    }

    fn write_initial_grid(&self, file_structure: &FileStructure) -> DynamicResult<()> {
        let mut dir = file_structure.grid().to_path_buf(); 
        dir.push(&PathBuf::from_str("t0000").unwrap());
        create_dir_all(&dir)?;
        self.grids.write_blocks(&dir)?;
        Ok(())
    }
}

/// Configuration for the program
#[derive(Debug, Serialize, Deserialize)]
pub struct AeolusSettings {
    verbosity: Verbosity,
    native_grid_format: GridFileType,
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
    gas_model: PathBuf,
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

fn remove_base_folder(dir: &Path, log: &UserLogger) -> Result<(), std::io::Error> {
    let base = dir.iter().next().unwrap();
    let base_dir = Path::new(base);
    if base_dir.is_dir() {
        // the directory exists, so let's remove it. We'll pass errors
        // up the stack
        match fs::remove_dir_all(base) {
            Ok(()) => {
                log.debug(&format!("removed {}", base_dir.display()));
                Ok(())
            },
            Err(err) => Err(err),
        }
    }
    else {
        // the directory doesn't exist, so job done!
        Ok(())
    }
}

impl FileStructure {
    pub fn create_directories(&self) {
        create_parent_directory(&self.solver);
        create_parent_directory(&self.discretisation);
        create_parent_directory(&self.grid);
        create_parent_directory(&self.fluid);
        create_parent_directory(&self.gas_model);
    }

    pub fn clean(&self, log: &UserLogger) -> Result<(), std::io::Error> {
        remove_base_folder(&self.solver, log)?;
        remove_base_folder(&self.discretisation, log)?;
        remove_base_folder(&self.grid, log)?;
        remove_base_folder(&self.fluid, log)?;
        remove_base_folder(&self.gas_model, log)?;
        Ok(())
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

    pub fn gas_model(&self) -> &Path {
        &self.gas_model
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
