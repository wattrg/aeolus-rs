pub mod gas_state;
pub mod gas_model;
pub mod ideal_gas;
pub mod block;

use pyo3::prelude::*;

use crate::block::{PyBlock, PyBlockIO};
use crate::gas_state::PyGasState;
use crate::ideal_gas::PyIdealGas;

// python module
#[pymodule]
pub fn aeolus_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyGasState>()?;
    m.add_class::<PyIdealGas>()?;
    m.add_class::<PyBlock>()?;
    m.add_class::<PyBlockIO>()?;
    Ok(())
}
