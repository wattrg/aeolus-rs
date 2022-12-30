use super::gas_state::GasState;
use crate::numerical_methods::number::Number;
use crate::gas::gas_model::GasModel;

use pyo3::pyclass;

#[allow(non_snake_case)]
#[pyclass]
pub struct IdealGas {
    R: Number, // J / kg / K
    Cv: Number, // J / K
    gamma: Number,
}

#[allow(non_snake_case)]
impl IdealGas {
    pub fn new(R: Number, gamma: Number) -> IdealGas {
        IdealGas{R, Cv: R/(gamma-1.), gamma}
    }

    fn update_sound_speed(&self, gs: &mut GasState) {
        gs.a = Number::sqrt(self.gamma * self.R * gs.T);
    }
}

#[allow(non_snake_case)]
impl GasModel for IdealGas {
    fn update_from_pT(&self, gs: &mut GasState) {
        gs.rho = gs.p / (self.R * gs.T);
        gs.u = self.Cv * gs.T;
        self.update_sound_speed(gs);
    }

    fn update_from_rhoT(&self, gs: &mut GasState) {
        gs.p = gs.rho * self.R * gs.T;
        gs.u = self.Cv * gs.T;
        self.update_sound_speed(gs);
    }

    fn update_from_rhou(&self, gs: &mut GasState) {
        gs.T = gs.u / self.Cv;
        gs.p = gs.rho * self.R * gs.T;
        self.update_sound_speed(gs);
    }

    fn update_from_rhop(&self, gs: &mut GasState) {
        gs.T = gs.p / (gs.rho * self.R);
        gs.u = self.Cv * gs.T;
        self.update_sound_speed(gs);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn update_from_pT() {
        let gm = IdealGas::new(287.05, 1.4);
        let mut gs = GasState::default();
        gs.p = 101325.0;
        gs.T = 300.0;
        gm.update_from_pT(&mut gs);

        let result = GasState{
            p: 101325., 
            T: 300.0, 
            rho: 1.176624281484062, 
            u: 215287.50000000006, 
            a: 347.2189510957027,
        }; 

        assert_eq!(gs, result);
    }

    #[test]
    #[allow(non_snake_case)]
    fn update_from_rhoT() {
        let gm = IdealGas::new(287.05, 1.4);
        let mut gs = GasState::default();
        gs.rho = 1.176624281484062;
        gs.T = 300.0;

        gm.update_from_rhoT(&mut gs);
        let result = GasState{
            p: 101325., 
            T: 300.0, 
            rho: 1.176624281484062, 
            u: 215287.50000000006, 
            a: 347.2189510957027,
        }; 

        assert_eq!(gs, result);
    }

    #[test]
    fn update_from_rhou() {
        let gm = IdealGas::new(287.05, 1.4);
        let mut gs = GasState::default();
        gs.rho = 1.176624281484062;
        gs.u = 215287.50000000006;

        gm.update_from_rhou(&mut gs);
        let result = GasState{
            p: 101325., 
            T: 300.0, 
            rho: 1.176624281484062, 
            u: 215287.50000000006, 
            a: 347.2189510957027,
        }; 

        assert_eq!(gs, result);
    }

    #[test]
    fn update_from_rhop() {
        let gm = IdealGas::new(287.05, 1.4);
        let mut gs = GasState::default();
        gs.rho = 1.176624281484062;
        gs.p = 101325.0;

        gm.update_from_rhop(&mut gs);
        let result = GasState{
            p: 101325., 
            T: 300.0, 
            rho: 1.176624281484062, 
            u: 215287.50000000006, 
            a: 347.2189510957027,
        }; 

        assert_eq!(gs, result);
    }
}
