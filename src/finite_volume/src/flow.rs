use common::number::Real;

#[derive(Clone)]
pub struct FlowStates {
    pub p: Vec<Real>,
    pub t: Vec<Real>,
    pub u: Vec<Real>,
    pub rho: Vec<Real>,
    pub vel_x: Vec<Real>,
    pub vel_y: Vec<Real>,
    pub vel_z: Vec<Real>,
}

impl FlowStates {
    pub fn with_capacity(capacity: usize) -> FlowStates {
        let p = Vec::with_capacity(capacity);
        let t = Vec::with_capacity(capacity);
        let u = Vec::with_capacity(capacity);
        let rho = Vec::with_capacity(capacity);
        let vel_x = Vec::with_capacity(capacity);
        let vel_y = Vec::with_capacity(capacity);
        let vel_z = Vec::with_capacity(capacity);
        FlowStates{p, t, u, rho, vel_x, vel_y, vel_z}
    }
}

pub struct ConservedQuantities {
    pub mass: Vec<Real>,
    pub momentum_x: Vec<Real>,
    pub momentum_y: Vec<Real>,
    pub momentum_z: Vec<Real>,
    pub energy: Vec<Real>,
}
