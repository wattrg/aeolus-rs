use std::path::PathBuf;
use std::fs::read_to_string;

use pyo3::prelude::*;

use common::DynamicResult;
use crate::settings::AeolusSettings;

pub fn prep_sim(sim: &mut PathBuf, _settings: &AeolusSettings) -> DynamicResult<()> {
    // if no extension was given, we'll add one
    if let None = sim.extension() {
        sim.set_extension("py");
    }
    
    let py_contents = read_to_string(&sim).unwrap();
    let py_file_name = sim.file_name().unwrap().to_str().unwrap();
    Python::with_gil(|py| {
        let _prep_module = PyModule::from_code(py, &py_contents, py_file_name, "prep").unwrap();
    });

    Ok(())
}
