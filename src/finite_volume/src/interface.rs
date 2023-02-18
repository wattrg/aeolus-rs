use common::number::Real;
use common::vector3::ArrayVec3;
use crate::util::Ids;
use crate::flow::FlowStates;

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

    length: usize,
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

    pub fn len(&self) -> usize {
        self.length
    }
}
