use std::str::FromStr;

use crate::{gas_state::GasState, ideal_gas::IdealGas};
use common::number::Real;

use num_complex::ComplexFloat as Number;
use serde_derive::{Serialize, Deserialize};

#[allow(non_snake_case)]
pub trait GasModel<Num: Number + Clone>: std::fmt::Debug{
    // thermodyanmics methods
    fn update_from_pT(&self, gs: &mut GasState<Num>);
    fn update_from_rhoT(&self, gs: &mut GasState<Num>);
    fn update_from_rhou(&self, gs: &mut GasState<Num>);
    fn update_from_rhop(&self, gs: &mut GasState<Num>);
    fn Cv(&self, gs: &GasState<Num>) -> Num;
    fn Cp(&self, gs: &GasState<Num>) -> Num;
    fn R(&self, gs: &GasState<Num>) -> Num;

    fn as_any(&self) -> &dyn std::any::Any;
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum GasModels { IdealGas, }

#[derive(Debug)]
pub struct InvalidGasModel;

impl FromStr for GasModels {
    type Err = InvalidGasModel;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ideal_gas" => Ok(GasModels::IdealGas),
            _ => Err(InvalidGasModel),
        }
    }
}

impl Default for GasModels {
    fn default() -> Self {
        GasModels::IdealGas
    }
}

impl Default for Box<dyn GasModel<Real>> {
    fn default() -> Self {
        Box::new(IdealGas::new(287.1, 1.4))  
    }
}
