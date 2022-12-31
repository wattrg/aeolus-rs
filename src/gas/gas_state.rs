use std::fmt::Display;

use num_complex::ComplexFloat as Number;
use pyo3::prelude::*;

use crate::numerical_methods::number::Real;

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

/// Python facing wrapper of a GasState, which only exposes the
/// [Real] implementation.
#[cfg(not(test))]
#[pyclass(name="GasState")]
pub struct PyGasState {
    pub inner: GasState<Real>,
}


#[cfg(not(test))]
#[pymethods]
#[allow(non_snake_case)]
impl PyGasState {
    #[new]
    fn new() -> PyGasState {
        PyGasState{inner: GasState::<Real>::default()}
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    fn __repr__(&self) -> String {
        self.inner.to_string()
    }
    
    #[getter]
    fn get_p(&self) -> Real {
        self.inner.p
    }

    #[setter]
    fn set_p(&mut self, val: Real) {
        self.inner.p = val;
    }

    #[getter]
    fn get_T(&self) -> Real {
        self.inner.T
    }

    #[setter]
    fn set_T(&mut self, val: Real) {
        self.inner.T = val;
    }

    #[getter]
    fn get_u(&self) -> Real {
        self.inner.u
    }

    #[setter]
    fn set_u(&mut self, val: Real) {
        self.inner.u = val;
    }

    #[getter]
    fn get_rho(&self) -> Real {
        self.inner.rho
    }

    #[setter]
    fn set_rho(&mut self, val: Real) {
        self.inner.rho = val;
    }
}
