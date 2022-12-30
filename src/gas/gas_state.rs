use crate::numerical_methods::number::Number;

#[allow(non_snake_case)]
#[derive(Default, PartialEq, Debug)]
pub struct GasState {
    /// The pressure (Pa)
    pub p: Number,

    /// The temperature (K)
    pub T: Number,

    /// The density (kg/m^3)
    pub rho: Number,

    /// The specific energy (J/kg)
    pub u: Number,

    /// Sound speed (m/s)
    pub a: Number,
}
