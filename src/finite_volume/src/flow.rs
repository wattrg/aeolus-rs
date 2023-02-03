use common::number::Real;

pub struct FlowStates {
    pub p: Vec<Real>,
    pub temp: Vec<Real>,
    pub u: Vec<Real>,
    pub rho: Vec<Real>,
    pub vel_x: Vec<Real>,
    pub vel_y: Vec<Real>,
    pub vel_z: Vec<Real>,
}

pub struct ConservedQuantities {
    pub mass: Vec<Real>,
    pub momentum_x: Vec<Real>,
    pub momentum_y: Vec<Real>,
    pub momentum_z: Vec<Real>,
    pub energy: Vec<Real>,
}
