use std::path::Path;

use common::DynamicResult;
use common::number::Real;
use common::vector3::ArrayVec3;
use grid::block::BlockCollection;
use gas::flow_state::FlowState;

use crate::boundary_conditions::BoundaryCondition;
use crate::fluid_block_io::FluidBlockIO;
use crate::interface::Interfaces;
use crate::cells::Cells;



pub struct FluidBlock {
    vertices: ArrayVec3,
    interfaces: Interfaces,
    cells: Cells,
    boundaries: Vec<BoundaryCondition>,
    id: usize,
    dimensions: u8,
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

    pub fn apply_pre_reconstruction_boundary_conditions(&mut self) {
        for boundary in self.boundaries.iter() {
            boundary.apply_pre_reconstruction_actions(&mut self.interfaces);
        }
    }

    pub fn dimensions(&self) -> u8 {
        self.dimensions
    }
}


pub struct FluidBlockCollection<'a> {
    fluid_blocks: Vec<FluidBlock>,
    fluid_block_io: Vec<FluidBlockIO<'a>>,
    time_index: usize,
}

pub type InitialCondition = fn(Real, Real, Real) -> FlowState<Real>;

impl<'a> FluidBlockCollection<'a> {
    pub fn with_constant_initial_condition(block_collection: &BlockCollection, initial_condition: FlowState<Real>) -> FluidBlockCollection {
        todo!()
    }

    pub fn with_variable_initial_condition(block_collection: &BlockCollection, initial_condition: InitialCondition) {
        todo!()
    }

    pub fn write_fluids_blocks(&mut self, path: &Path) -> DynamicResult<()> {
        self.time_index += 1;
        let mut block_path = path.to_path_buf();
        block_path.push(format!("{:0>4}", self.time_index));
        for block_io in self.fluid_block_io.iter_mut() {
            block_path.set_file_name(format!("blk{:0>4}.fluid", block_io.id()));
            block_io.write_fluid_block(&block_path)?;
        } 
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use grid::block::BlockCollection;
    use grid::Block;
    use crate::util::Ids;

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
