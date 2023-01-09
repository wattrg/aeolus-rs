use pyo3::prelude::*;

use common::number::Real;
use gas::gas_state::GasState;

/// Python facing wrapper of a GasState, which only exposes the
/// [Real] implementation.
#[pyclass(name="GasState")]
pub struct PyGasState {
    pub inner: GasState<Real>,
}

#[allow(non_snake_case)]
#[pymethods]
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
