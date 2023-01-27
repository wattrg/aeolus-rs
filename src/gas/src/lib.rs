use serde_derive::{Serialize, Deserialize};

/// Store the state of a blob of gas
pub mod gas_state;

pub mod flow_state;

/// Model the behaviour of a gas
pub mod gas_model;

/// Ideal gas
pub mod ideal_gas;

#[derive(Debug, Serialize, Deserialize)]
pub enum GasModels {
    IdealGas,
}
