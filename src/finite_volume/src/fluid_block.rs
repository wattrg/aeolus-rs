use std::path::Path;

use crate::util::Ids;
use crate::flow::{FlowStates, ConservedQuantities};
use common::DynamicResult;
use common::number::Real;
use common::vector3::{ArrayVec3, Vector3};
use grid::block::{BlockCollection, GridBlock};
use grid::interface::Direction;
use grid::{Block, Vertex};
use gas::flow_state::FlowState;


pub struct Interfaces {
    vertex_ids: Ids,

    // the area of the interface
    area: Vec<Real>,

    // the unit vectors describing the orientation
    norm: ArrayVec3,
    t1: ArrayVec3,
    t2: ArrayVec3,

    left_flow_states: FlowStates,
    right_flow_states: FlowStates,

    // the centre of the interface
    centre: Vec<Real>,
}

impl Interfaces {
    pub fn vertices(&self) -> &Ids {
        &self.vertex_ids
    }

    pub fn area(&self) -> &[Real] {
        &self.area
    }

    pub fn norm(&self) -> &ArrayVec3 {
        &self.norm
    }

    pub fn t1(&self) -> &ArrayVec3 {
        &self.t1
    }

    pub fn t2(&self) -> &ArrayVec3 {
        &self.t2
    }

    pub fn centre(&self) -> &[Real] {
        &self.centre
    }
}


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

pub struct FluidBlock {
    vertices: ArrayVec3,
    interfaces: Interfaces,
    cells: Cells,
    id: usize,
}

impl FluidBlock {
    pub fn vertices(&self) -> &ArrayVec3 {
        &self.vertices
    }

    pub fn interfaces(&self) -> &Interfaces {
        &self.interfaces
    }

    pub fn cells(&self) -> &Cells {
        &self.cells
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

pub struct FluidBlockCollection {
    fluid_blocks: Vec<FluidBlock>,
    time_index: usize,
}

pub type InitialCondition = fn(Real, Real, Real) -> FlowState<Real>;

impl FluidBlockCollection {
    pub fn with_constant_initial_condition(block_collection: &BlockCollection, initial_condition: FlowState<Real>) -> FluidBlockCollection {
        for block in block_collection.blocks().iter() {
            let vertices = Self::get_vertices(block);
        }
        todo!()
    }

    pub fn with_variable_initial_condition(block_collection: &BlockCollection, initial_condition: InitialCondition) {
        todo!()
    }

    pub fn write_fluid_blocks(&mut self, dir: &Path) -> DynamicResult<()> {
        todo!() 
    }

    fn get_vertices(block: &GridBlock) -> ArrayVec3 {
        // TODO: think about re-jigging the Vertex trait to avoid the clone
        let vertices: Vec<Vector3> = block.vertices().iter().map(|vertex| vertex.pos().clone()).collect(); 
        ArrayVec3::from_vector3s(&vertices)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use grid::block::BlockCollection;
    use grid::Block;

    #[test]
    fn test_interface_ids() {
        // read a block
        let mut block_collection = BlockCollection::new(); 
        block_collection.add_block(&PathBuf::from("../grid/tests/data/square.su2")).unwrap();
        let block = block_collection.get_block(0);

        let interface_ids = Ids::from_interfaces(block.interfaces());
        assert_eq!(interface_ids[0], [0, 1]);
        assert_eq!(interface_ids[1], [1, 5]);
        assert_eq!(interface_ids[2], [5, 4]);
        assert_eq!(interface_ids[3], [4, 0]);
        assert_eq!(interface_ids[4], [1, 2]);
        assert_eq!(interface_ids[5], [2, 6]);
        assert_eq!(interface_ids[12], [8, 4]);
        assert_eq!(interface_ids[23], [15, 14]);
    }

    #[test]
    fn test_cell_ids() {
        // read a block
        let mut block_collection = BlockCollection::new(); 
        block_collection.add_block(&PathBuf::from("../grid/tests/data/square.su2")).unwrap();
        let block = block_collection.get_block(0);

        let (vertex_ids, interface_ids) = Ids::from_cells(block.cells());
        assert_eq!(vertex_ids[0], [0, 1, 5, 4]);
        assert_eq!(vertex_ids[5], [6, 7, 11, 10]);

        assert_eq!(interface_ids[0], [0, 1, 2, 3]);
        assert_eq!(interface_ids[5], [9, 15, 16, 13]);
    }
}
