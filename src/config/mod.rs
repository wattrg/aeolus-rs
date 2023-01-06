use std::path::{PathBuf, Path};
use serde_derive::{Serialize, Deserialize};

use crate::{gas::GasModels, solvers::Solvers};

/// Configuration for the program
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    verbosity: Verbosity,
    file_structure: FileStructure,
    gas_model: GasModels,
    solver: Solvers,
}

impl Config {
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

