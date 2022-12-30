use crate::numerical_methods::number::Number;

use pyo3::prelude::*;

#[allow(non_snake_case)]
#[derive(Default, PartialEq, Debug)]
#[pyclass]
pub struct GasState {
    /// The pressure (Pa)
    #[pyo3(get, set)]
    pub p: Number,

    /// The temperature (K)
    #[pyo3(get, set)]
    pub T: Number,

    /// The density (kg/m^3)
    #[pyo3(get, set)]
    pub rho: Number,

    /// The specific energy (J/kg)
    #[pyo3(get, set)]
    pub u: Number,

    /// Sound speed (m/s)
    #[pyo3(get, set)]
    pub a: Number,
}

#[pymethods]
impl GasState {
    #[new]
    pub fn new() -> GasState {
        GasState::default()
    }

    fn __repr__(&self) -> String {
        self.to_string()
    }

    fn __str__(&self) -> String {
        self.to_string()
    }
}

impl std::fmt::Display for GasState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let string = format!("GasState(p={}, T={}, rho={}, u={}, a={})", 
                           self.p, self.T, self.rho, self.u, self.a);
        write!(f, "{}", string)
    }
}
