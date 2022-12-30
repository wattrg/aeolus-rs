use crate::gas::gas_state::GasState;

#[allow(non_snake_case)]
pub trait GasModel {
    fn update_from_pT(&self, gs: &mut GasState);
    fn update_from_rhoT(&self, gs: &mut GasState);
    fn update_from_rhou(&self, gs: &mut GasState);
    fn update_from_rhop(&self, gs: &mut GasState);
}
