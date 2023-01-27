use std::ops::Index;

use common::number::Real;
use grid::interface::GridInterface;
use grid::cell::GridCell;
use grid::{Cell, Interface};

pub struct ArrayVec3 {
    pub x: Vec<Real>,
    pub y: Vec<Real>,
    pub z: Vec<Real>,
}

/// Keep track of the ids of objects forming another object.
/// For example, the id's of the interfaces surrounding a cell.
/// We store it dynamically since we don't know how many interfaces
/// may be surrounding the cell (we don't know the shape of the cell
/// at compile time, and we allow different shaped cells in the
/// same grid).
pub struct Ids {
    ids: Vec<usize>,
    offsets: Vec<usize>,
}

impl Index<usize> for Ids {
    type Output = [usize];

    fn index(&self, index: usize) -> &Self::Output {
        &self.ids[self.offsets[index] .. self.offsets[index+1]]
    }
}

impl Ids {
    pub fn from_interfaces(interfaces: &Vec<GridInterface>) -> Ids {
        let capacity = interfaces.len();
        let mut offsets: Vec<usize> = Vec::with_capacity(capacity);
        let mut ids: Vec<usize> = Vec::new();
        for interface in interfaces.iter() {
            offsets.push(ids.len());
            ids.extend(interface.vertex_ids());
        };
        offsets.push(ids.len());
        Ids {ids, offsets}
    }

    pub fn from_cells(cells: &Vec<GridCell>) -> (Ids, Ids) {
        let capacity = cells.len();
        let mut interface_offsets: Vec<usize> = Vec::with_capacity(capacity);
        let mut vertex_offsets: Vec<usize> = Vec::with_capacity(capacity);
        let mut interface_ids: Vec<usize> = Vec::new();
        let mut vertex_ids: Vec<usize> = Vec::new();
        for cell in cells.iter() {
            interface_offsets.push(interface_ids.len());
            vertex_offsets.push(vertex_ids.len());
            interface_ids.extend(cell.interface_ids());
            vertex_ids.extend(cell.vertex_ids());
        };
        interface_offsets.push(interface_ids.len());
        vertex_offsets.push(interface_ids.len());
        (
            Ids {ids: vertex_ids, offsets: vertex_offsets}, 
            Ids {ids: interface_ids, offsets: interface_offsets}
        )
    }
}

pub struct Interfaces {
    vertex_ids: Ids,

    // the area of the interface
    area: Vec<Real>,

    // the unit vectors describing the orientation
    norm: ArrayVec3,
    t1: ArrayVec3,
    t2: ArrayVec3,

    // the centre of the interface
    centre: Vec<Real>,
}

impl Interfaces {
    pub fn vertices(&self) -> &Ids {
        &self.vertex_ids
    }

    pub fn area(&self) -> &Vec<Real> {
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

    pub fn centre(&self) -> &Vec<Real> {
        &self.centre
    }
}

pub struct Cells {
    vertices: Ids,
    interfaces: Ids,

    volume: Vec<Real>,
    centre: Vec<Real>,
}

impl Cells {
    pub fn vertices(&self) -> &Ids {
        &self.vertices
    }

    pub fn interfaces(&self) -> &Ids {
        &self.interfaces
    }

    pub fn volume(&self) -> &Vec<Real> {
        &self.volume
    }

    pub fn centre(&self) -> &Vec<Real> {
        &self.centre
    }
}

pub struct FluidBlock {
    vertices: ArrayVec3,
    interfaces: Interfaces,
    cells: Cells,
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
}
