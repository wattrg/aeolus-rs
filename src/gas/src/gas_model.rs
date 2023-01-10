use crate::gas_state::GasState;
use num_complex::ComplexFloat as Number;

#[allow(non_snake_case)]
pub trait GasModel<Num: Number> {
    fn update_from_pT(&self, gs: &mut GasState<Num>);
    fn update_from_rhoT(&self, gs: &mut GasState<Num>);
    fn update_from_rhou(&self, gs: &mut GasState<Num>);
    fn update_from_rhop(&self, gs: &mut GasState<Num>);
    fn Cv(&self, gs: &GasState<Num>) -> Num;
    fn Cp(&self, gs: &GasState<Num>) -> Num;
    fn R(&self, gs: &GasState<Num>) -> Num;
}
