use std::ops::Index;

use grid::cell::GridCell;
use grid::interface::GridInterface;
use grid::{Cell, Interface};

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
