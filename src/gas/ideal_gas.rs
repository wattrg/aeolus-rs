use super::gas_state::GasState;
use crate::gas::gas_model::GasModel;
use num_complex::ComplexFloat as Number;


#[allow(non_snake_case)]
pub struct IdealGas<Num: Number> {
    R: Num, // J / kg / K
    Cv: Num, // J / K
    gamma: Num,
}

#[allow(non_snake_case)]
impl<Num: Number> IdealGas<Num> {
    pub fn new(R: Num, gamma: Num) -> IdealGas<Num> {
        IdealGas{R, Cv: R/(gamma-Num::one()), gamma}
    }

    fn update_sound_speed(&self, gs: &mut GasState<Num>) {
        gs.a = Num::sqrt(self.gamma * self.R * gs.T);
    }
}

#[allow(non_snake_case)]
impl <Num: Number> GasModel<Num> for IdealGas<Num> {
    fn update_from_pT(&self, gs: &mut GasState<Num>) {
        gs.rho = gs.p / (self.R * gs.T);
        gs.u = self.Cv * gs.T;
        self.update_sound_speed(gs);
    }

    fn update_from_rhoT(&self, gs: &mut GasState<Num>) {
        gs.p = gs.rho * self.R * gs.T;
        gs.u = self.Cv * gs.T;
        self.update_sound_speed(gs);
    }

    fn update_from_rhou(&self, gs: &mut GasState<Num>) {
        gs.T = gs.u / self.Cv;
        gs.p = gs.rho * self.R * gs.T;
        self.update_sound_speed(gs);
    }

    fn update_from_rhop(&self, gs: &mut GasState<Num>) {
        gs.T = gs.p / (gs.rho * self.R);
        gs.u = self.Cv * gs.T;
        self.update_sound_speed(gs);
    }

    fn Cv(&self, _gs: &GasState<Num>) -> Num {
        self.Cv
    }

    fn Cp(&self, _gs: &GasState<Num>) -> Num {
        self.Cv + self.R
    }

    fn R(&self, _gs: &GasState<Num>) -> Num {
        self.R
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
