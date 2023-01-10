
/// Generate a python interface to the GasModel trait for
/// a concrete type implementing the trait
macro_rules! create_gas_model_python_interface {
    ($inner_name: ident, $wrapper_name: ident, $python_name: literal) => {
        use pyo3::{pyclass, pymethods};
        use crate::gas_state::PyGasState;
        use gas::gas_model::GasModel;

        #[pyclass]
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
