use common::number::Real;
use grid::interface::Direction;

use crate::util::Ids;
use crate::flow::{FlowStates, ConservedQuantities};

pub struct Cells {
    // geometric information
    vertices: Ids,
    interfaces: Ids,
    interface_directions: Vec<Direction>,
    volume: Vec<Real>,
    centre: Vec<Real>,

    flow_states: FlowStates,
    conserved_quantities: ConservedQuantities,
    residuals: ConservedQuantities,
}

impl Cells {
    pub fn vertices(&self) -> &Ids {
        &self.vertices
    }

    pub fn interfaces(&self) -> &Ids {
        &self.interfaces
    }

    pub fn interface_directions(&self) -> &[Direction] {
        &self.interface_directions
    }

    pub fn volume(&self) -> &[Real] {
        &self.volume
    }

    pub fn centre(&self) -> &[Real] {
        &self.centre
    }
}
