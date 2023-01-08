use std::path::PathBuf;

use pyo3::prelude::*;

use crate::grid::block::{Block, BlockIO};
use crate::gas::{gas_model::GasModel, ideal_gas::IdealGas, gas_state::GasState};
use crate::numerical_methods::number::Real;

// python module
#[cfg(not(test))]
#[pymodule]
fn aeolus_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyGasState>()?;
    m.add_class::<PyIdealGas>()?;
    m.add_class::<PyBlock>()?;
    m.add_class::<PyBlockIO>()?;
    Ok(())
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

/// Generate a python interface to the GasModel trait for
/// a concrete type implementing the trait
macro_rules! create_gas_model_python_interface {
    ($inner_name: ident, $wrapper_name: ident, $python_name: literal) => {
        use pyo3::{pyclass, pymethods};

        #[pyclass(name=$python_name)]
        pub struct $wrapper_name {
            inner: $inner_name<Real>
        }

        #[allow(non_snake_case)]
        #[pymethods]
        impl $wrapper_name {
            fn update_from_pT(&self, gs: &mut PyGasState) {
                self.inner.update_from_pT(&mut gs.inner);
            }

            fn update_from_rhoT(&self, gs: &mut PyGasState) {
                self.inner.update_from_rhoT(&mut gs.inner);
            }
            
            fn update_from_rhou(&self, gs: &mut PyGasState) {
                self.inner.update_from_rhou(&mut gs.inner);
            }

            fn update_from_rhop(&self, gs: &mut PyGasState) {
                self.inner.update_from_rhop(&mut gs.inner);
            }

            fn Cv(&self, gs: &PyGasState) -> Real {
                self.inner.Cv(&gs.inner)
            }

            fn Cp(&self, gs: &PyGasState) -> Real {
                self.inner.Cp(&gs.inner)
            }

            fn R(&self, gs: &PyGasState) -> Real {
                self.inner.R(&gs.inner)
            }
        }
    };
}
pub(crate) use create_gas_model_python_interface;

#[cfg(not(test))]
create_gas_model_python_interface!(IdealGas, PyIdealGas, "IdealGas");

#[cfg(not(test))]
#[allow(non_snake_case)]
#[pymethods]
impl PyIdealGas {
    #[new]
    fn new(R: Real, gamma: Real) -> PyIdealGas {
        PyIdealGas{inner: IdealGas::new(R, gamma)}
    }
}

/// Python facing wrapper for a Block
#[cfg(not(test))]
#[pyclass(name="Block")]
pub struct PyBlock {
    pub inner: Block,
}

#[cfg(not(test))]
#[pyclass(name="BlockIO")]
pub struct PyBlockIO {
    pub block_io: BlockIO,
    pub blocks: Vec<PyBlock>,
}

#[cfg(not(test))]
#[pymethods]
impl PyBlockIO {
    #[new]
    fn new() -> PyBlockIO {
        PyBlockIO{ block_io: BlockIO::new(), blocks: Vec::new() }
    }

    fn add_block(&mut self, file_path: &str) {
        let block = self.block_io
            .create_block(&PathBuf::from(file_path))
            .unwrap();
        self.blocks.push( PyBlock{ inner: block } );
    }
}
