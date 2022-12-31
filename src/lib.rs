use pyo3::prelude::*;

pub mod util;
pub mod numerical_methods;
pub mod grid;
pub mod gas;

/// Short hand for returning a result with some generic `Ok` type
/// and a dynamic `Err` type
pub type DynamicResult<T> = Result<T, Box<dyn std::error::Error>>;

// python module
#[cfg(not(test))]
#[pymodule]
fn aeolus_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<gas::gas_state::PyGasState>()?;
    m.add_class::<gas::ideal_gas::PyIdealGas>()?;
    Ok(())
}
