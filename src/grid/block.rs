use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

use pyo3::prelude::*;

use super::cell::Cell;
use super::vertex::Vertex;
use super::interface::Interface;
use crate::DynamicResult;
use super::su2::read_su2;


#[derive(Debug)]
pub struct Block {
    vertices: Vec<Vertex>,
    interfaces: Vec<Interface>,
    cells: Vec<Cell>,
    boundaries: HashMap<String, Vec<usize>>,
    dimensions: u8,
    id: usize,
}

impl Block {
    pub fn new(vertices: Vec<Vertex>, interfaces: Vec<Interface>, cells: Vec<Cell>,
               boundaries: HashMap<String, Vec<usize>>, dimensions: u8, id: usize) -> Block {
        Block{vertices, interfaces, cells, boundaries, dimensions, id}
    }
    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn interfaces(&self) -> &Vec<Interface> {
        &self.interfaces
    }

    pub fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    pub fn dimensions(&self) -> u8 {
        self.dimensions
    }

    pub fn boundaries(&self) -> &HashMap<String, Vec<usize>> {
        &self.boundaries
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

pub struct BlockIO {
    number_blocks: usize,
}

impl BlockIO {
    pub fn new() -> BlockIO {
        BlockIO { number_blocks: 0 }
    }

    pub fn create_block(&mut self, file_path: &Path) -> DynamicResult<Block> {
        let ext = GridFileType::from_file_name(&file_path)?;
        match ext {
            GridFileType::Su2 => {
                let block = read_su2(&file_path, self.number_blocks)?;
                self.number_blocks += 1;
                Ok(block)
            }
        }
    }
}


/// For handling errors associated with file types we don't know how to read
#[derive(Debug, PartialEq, Eq)]
pub struct UnknownFileType {
    name: PathBuf,
    ext: Option<String>,
}

impl std::error::Error for UnknownFileType {}

impl std::fmt::Display for UnknownFileType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.ext {
            Some(extension) => write!(f, "Unknown extension '{}' for file '{:?}'", extension, self.name),
            None => write!(f, "No extension to file: {:?}", self.name),
        }
    }
}

impl UnknownFileType {
    pub fn new(name: PathBuf, ext: Option<String>) -> UnknownFileType {
        UnknownFileType { name, ext } 
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum GridFileType {
    Su2,
}

impl GridFileType {
    /// Convert file name to file type
    pub fn from_file_name(file_path: &Path) -> Result<GridFileType, UnknownFileType> {
        let ext = file_path.extension().and_then(OsStr::to_str);
        match ext {
            Some("su2") => Ok(GridFileType::Su2),
            Some(unknown_ext) => Err(UnknownFileType::new(file_path.to_owned(), Some(unknown_ext.to_string()))),
            None => Err(UnknownFileType::new(file_path.to_owned(), None)),
        }
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



#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::{vertex::Vertex, interface::Interface, cell::Cell};
    use crate::util::vector3::Vector3;

    #[test]
    fn grid_file_type() {
        let file_type = GridFileType::from_file_name(&PathBuf::from("grid.su2"));

        assert_eq!(file_type, Ok(GridFileType::Su2));
    }

    #[test]
    fn grid_file_type_unknown() {
        let file_type = GridFileType::from_file_name(&PathBuf::from("grid.su3")); 
        let err = UnknownFileType { name: PathBuf::from("grid.su3"), ext: Some("su3".to_string())};
        assert_eq!(file_type, Err(err));
    }

    #[test]
    fn read_su2_file() {
        let mut block_io = BlockIO::new();
        let block = block_io.create_block(&PathBuf::from("./tests/data/square.su2")).unwrap();    

        let vertices = vec![
            Vertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 0),
            Vertex::new(Vector3{x: 1.0, y: 0.0, z: 0.0}, 1),
            Vertex::new(Vector3{x: 2.0, y: 0.0, z: 0.0}, 2),
            Vertex::new(Vector3{x: 3.0, y: 0.0, z: 0.0}, 3),
            Vertex::new(Vector3{x: 0.0, y: 1.0, z: 0.0}, 4),
            Vertex::new(Vector3{x: 1.0, y: 1.0, z: 0.0}, 5),
            Vertex::new(Vector3{x: 2.0, y: 1.0, z: 0.0}, 6),
            Vertex::new(Vector3{x: 3.0, y: 1.0, z: 0.0}, 7),
            Vertex::new(Vector3{x: 0.0, y: 2.0, z: 0.0}, 8),
            Vertex::new(Vector3{x: 1.0, y: 2.0, z: 0.0}, 9),
            Vertex::new(Vector3{x: 2.0, y: 2.0, z: 0.0}, 10),
            Vertex::new(Vector3{x: 3.0, y: 2.0, z: 0.0}, 11),
            Vertex::new(Vector3{x: 0.0, y: 3.0, z: 0.0}, 12),
            Vertex::new(Vector3{x: 1.0, y: 3.0, z: 0.0}, 13),
            Vertex::new(Vector3{x: 2.0, y: 3.0, z: 0.0}, 14),
            Vertex::new(Vector3{x: 3.0, y: 3.0, z: 0.0}, 15),
        ];

        let interfaces = vec![
            Interface::new_from_vertices(&[&vertices[0], &vertices[1]], 0), 
            Interface::new_from_vertices(&[&vertices[1], &vertices[5]], 1),
            Interface::new_from_vertices(&[&vertices[5], &vertices[4]], 2),
            Interface::new_from_vertices(&[&vertices[4], &vertices[0]], 3),
            Interface::new_from_vertices(&[&vertices[1], &vertices[2]], 4),
            Interface::new_from_vertices(&[&vertices[2], &vertices[6]], 5),
            Interface::new_from_vertices(&[&vertices[6], &vertices[5]], 6),
            Interface::new_from_vertices(&[&vertices[2], &vertices[3]], 7),
            Interface::new_from_vertices(&[&vertices[3], &vertices[7]], 8),
            Interface::new_from_vertices(&[&vertices[7], &vertices[6]], 9),
            Interface::new_from_vertices(&[&vertices[5], &vertices[9]], 10),
            Interface::new_from_vertices(&[&vertices[9], &vertices[8]], 11),
            Interface::new_from_vertices(&[&vertices[8], &vertices[4]], 12),
            Interface::new_from_vertices(&[&vertices[6], &vertices[10]], 13), 
            Interface::new_from_vertices(&[&vertices[10], &vertices[9]], 14),
            Interface::new_from_vertices(&[&vertices[7], &vertices[11]], 15), 
            Interface::new_from_vertices(&[&vertices[11], &vertices[10]], 16), 
            Interface::new_from_vertices(&[&vertices[9], &vertices[13]], 17),
            Interface::new_from_vertices(&[&vertices[13], &vertices[12]], 18), 
            Interface::new_from_vertices(&[&vertices[12], &vertices[8]], 19),
            Interface::new_from_vertices(&[&vertices[10], &vertices[14]], 20),
            Interface::new_from_vertices(&[&vertices[14], &vertices[13]], 21),
            Interface::new_from_vertices(&[&vertices[11], &vertices[15]], 22),
            Interface::new_from_vertices(&[&vertices[15], &vertices[14]], 23),
        ];

        let cells = vec![
            Cell::new(&[&interfaces[0], &interfaces[1], &interfaces[2], &interfaces[3]], 
                      &[&vertices[0], &vertices[1], &vertices[5], &vertices[4]], 0),
            Cell::new(&[&interfaces[4], &interfaces[5], &interfaces[6], &interfaces[1]], 
                      &[&vertices[1], &vertices[2], &vertices[6], &vertices[5]], 1),
            Cell::new(&[&interfaces[7], &interfaces[8], &interfaces[9], &interfaces[5]], 
                      &[&vertices[2], &vertices[3], &vertices[7], &vertices[6]], 2),
            Cell::new(&[&interfaces[2], &interfaces[10], &interfaces[11], &interfaces[12]], 
                      &[&vertices[4], &vertices[5], &vertices[9], &vertices[8]], 3),
            Cell::new(&[&interfaces[6], &interfaces[13], &interfaces[14], &interfaces[10]], 
                      &[&vertices[5], &vertices[6], &vertices[10], &vertices[9]], 4),
            Cell::new(&[&interfaces[9], &interfaces[15], &interfaces[16], &interfaces[13]], 
                      &[&vertices[6], &vertices[7], &vertices[11], &vertices[10]], 5),
            Cell::new(&[&interfaces[11], &interfaces[17], &interfaces[18], &interfaces[19]], 
                      &[&vertices[8], &vertices[9], &vertices[13], &vertices[12]], 6),
            Cell::new(&[&interfaces[14], &interfaces[20], &interfaces[21], &interfaces[17]], 
                      &[&vertices[9], &vertices[10], &vertices[14], &vertices[13]], 7),
            Cell::new(&[&interfaces[16], &interfaces[22], &interfaces[23], &interfaces[20]], 
                      &[&vertices[10], &vertices[11], &vertices[15], &vertices[14]], 8),
        ];

        let boundaries = HashMap::from([
            ("slip_wall_bottom".to_string(), vec![0, 4, 7]),
            ("outflow".to_string(), vec![8, 15, 22]),
            ("slip_wall_top".to_string(), vec![18, 21, 23]),
            ("inflow".to_string(), vec![3, 12, 19]),
        ]);

        assert_eq!(block.vertices(), &vertices);
        assert_eq!(block.interfaces(), &interfaces);
        assert_eq!(block.cells(), &cells);
        assert_eq!(block.boundaries(), &boundaries);
        assert_eq!(block.dimensions(), 2);
    }
}
