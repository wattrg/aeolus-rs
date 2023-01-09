use gas::ideal_gas::IdealGas;
use crate::gas_model::create_gas_model_python_interface;
use common::number::Real;

create_gas_model_python_interface!(IdealGas, PyIdealGas, "IdealGas");

#[allow(non_snake_case)]
#[pymethods]
impl PyIdealGas {
    #[new]
    fn new(R: Real, gamma: Real) -> PyIdealGas {
        PyIdealGas{inner: IdealGas::new(R, gamma)}
    }
}
