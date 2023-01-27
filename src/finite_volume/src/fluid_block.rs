
use std::ops::Index;

use common::number::Real;

pub struct ArrayVec3 {
    pub x: Vec<Real>,
    pub y: Vec<Real>,
    pub z: Vec<Real>,
}

pub struct Ids {
    vertex_ids: Vec<usize>,
    offsets: Vec<usize>,
}

impl Index<usize> for Ids {
    type Output = [usize];

    fn index(&self, index: usize) -> &Self::Output {
        &self.vertex_ids[self.offsets[index] .. self.offsets[index+1]]
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
