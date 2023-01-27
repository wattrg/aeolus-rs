use common::vector3::Vector3;
use num_complex::ComplexFloat as Number;

use crate::gas_state::GasState;

pub struct FlowState<Num: Number> {
    gas_state: GasState<Num>,
    velocity: Vector3,
}

impl<Num: Number> FlowState<Num> {
    pub fn gas_state(&self) -> &GasState<Num> {
        &self.gas_state
    }

    pub fn gas_state_mut(&mut self) -> &mut GasState<Num> {
        &mut self.gas_state
    }

    pub fn velocity(&self) -> &Vector3 {
        &self.velocity
    }

    pub fn velocity_mut(&mut self) -> &mut Vector3 {
        &mut self.velocity
    }
}
