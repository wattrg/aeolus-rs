use pyo3::prelude::*;

use std::path::PathBuf;
use grid::block::{Block, BlockIO};

/// Python facing wrapper for a Block
#[pyclass(name="Block")]
pub struct PyBlock {
    pub inner: Block,
}

#[pyclass(name="BlockIO")]
pub struct PyBlockIO {
    pub block_io: BlockIO,
    pub blocks: Vec<PyBlock>,
}

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
