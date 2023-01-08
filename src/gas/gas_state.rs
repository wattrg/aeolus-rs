use std::fmt::Display;

use num_complex::ComplexFloat as Number;

#[allow(non_snake_case)]
#[derive(Default, PartialEq, Eq, Debug)]
pub struct GasState<Num: Number> {
    /// The pressure (Pa)
    pub p: Num,

    /// The temperature (K)
    pub T: Num,

    /// The density (kg/m^3)
    pub rho: Num,

    /// The specific energy (J/kg)
    pub u: Num,

    /// Sound speed (m/s)
    pub a: Num,
}

impl<Num: Number + Default> GasState<Num> {
    pub fn new() -> GasState<Num> {
        GasState::default()
    }
}

impl<Num> std::fmt::Display for GasState<Num>
    where Num: Number + Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let string = format!("GasState(p={}, T={}, rho={}, u={}, a={})", 
                           self.p, self.T, self.rho, self.u, self.a);
        write!(f, "{}", string)
    }
}

