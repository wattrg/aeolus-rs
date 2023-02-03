use crate::interface::Interfaces;

pub struct BoundaryCondition {
    tag: String,
    convective_flux_computed_in_boundary: bool,
    has_ghost_cells: bool,

    // the interfaces on the boundary
    interfaces: Vec<usize>,

    pre_reconstruction_actions: Vec<Box<dyn PreReconstructionAction>>,
}

impl BoundaryCondition {
    pub fn apply_pre_reconstruction_actions(&self, interfaces: &mut Interfaces) {
        for pre_reconstruction_action in self.pre_reconstruction_actions.iter() {
            pre_reconstruction_action.apply_pre_reconstruction_action(&self.interfaces, interfaces);
        }
    }

    pub fn has_ghost_cells(&self) -> bool {
        self.has_ghost_cells
    }

    pub fn convective_flux_computed_in_boundary(&self) -> bool {
        self.convective_flux_computed_in_boundary
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }
}

pub trait PreReconstructionAction {
    fn apply_pre_reconstruction_action(&self, boundary_faces: &[usize], interfaces: &mut Interfaces);
}
