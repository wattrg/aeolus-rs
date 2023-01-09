use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

use super::cell::Cell;
use super::su2::write_su2;
use super::vertex::Vertex;
use super::interface::Interface;
use common::DynamicResult;
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

    pub fn write_block(&self, block: &Block, file_type: GridFileType, mut file_path: PathBuf) {
        let ext = file_type.extension();
        file_path.set_extension(ext);

        match file_type {
            GridFileType::Su2 => write_su2(&file_path, block),
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
            Some(extension) => write!(
                f, "Unknown extension '{}' for file '{:?}'", extension, self.name
            ),
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

    pub fn extension(&self) -> &str {
        match &self {
            GridFileType::Su2 => "su2",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;


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
}

