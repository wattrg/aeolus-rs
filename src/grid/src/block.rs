use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

use rlua::{UserData, UserDataMethods};
use serde_derive::{Serialize, Deserialize};

use crate::Block;
use crate::Cell;
use crate::Interface;
use crate::Vertex;

use super::cell::GridCell;
use super::su2::write_su2;
use super::vertex::GridVertex;
use super::interface::GridInterface;
use common::DynamicResult;
use super::su2::read_su2;


#[derive(Debug, Clone)]
pub struct GridBlock {
    vertices: Vec<GridVertex>,
    interfaces: Vec<GridInterface>,
    cells: Vec<GridCell>,
    boundaries: HashMap<String, Vec<usize>>,
    dimensions: u8,
    id: usize,
}

impl UserData for GridBlock {}
impl UserData for &GridBlock {}

impl GridBlock {
    pub fn new(vertices: Vec<GridVertex>, mut interfaces: Vec<GridInterface>, cells: Vec<GridCell>,
               boundaries: HashMap<String, Vec<usize>>, dimensions: u8, id: usize) -> GridBlock {
        for cell in cells.iter() {
            cell.attach_cell_to_interfaces(&mut interfaces);
        }
        GridBlock{vertices, interfaces, cells, boundaries, dimensions, id}
    }
}

impl Block<GridVertex, GridInterface, GridCell> for GridBlock  {
    fn vertices(&self) -> &Vec<GridVertex> {
        &self.vertices
    }

    fn interfaces(&self) -> &Vec<GridInterface> {
        &self.interfaces
    }

    fn cells(&self) -> &Vec<GridCell> {
        &self.cells
    }

    fn dimensions(&self) -> u8 {
        self.dimensions
    }

    fn boundaries(&self) -> &HashMap<String, Vec<usize>> {
        &self.boundaries
    }

    fn id(&self) -> usize {
        self.id
    }
}

/// A collection of blocks
#[derive(Default, Debug, Clone)]
pub struct BlockCollection {
    blocks: Vec<GridBlock>,
}

impl BlockCollection {
    pub fn new() -> BlockCollection {
        BlockCollection { blocks: Vec::new() }
    }

    pub fn add_block(&mut self, file_path: &Path) -> DynamicResult<()> {
        let ext = GridFileType::from_file_name(file_path)?;
        let number_blocks = self.blocks.len();
        let block = match ext {
            GridFileType::Native | GridFileType::Su2 => read_su2(file_path, number_blocks)?,
        };
        self.blocks.push(block);
        Ok(())
    }

    pub fn get_block(&self, id: usize) -> &GridBlock {
        &self.blocks[id]
    }

    pub fn blocks(&self) -> &Vec<GridBlock> {
        &self.blocks
    }

    /// write the blocks out in native format
    pub fn write_blocks(&self, grid_dir: &Path) -> DynamicResult<()> {
        let mut file_name = grid_dir.to_path_buf();
        let ext = GridFileType::Native.extension();
        file_name.push("block");
        for block in self.blocks.iter() {
            file_name.set_file_name(format!("block_{:04}.{}", block.id(), ext));
            write_block(block, &file_name)?;
        }
        Ok(())
    }
}

impl UserData for BlockCollection {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("add_block", |_, block_collection, file_name: String| {
            let path = PathBuf::from_str(&file_name).unwrap();
            block_collection.add_block(&path).unwrap(); 
            Ok(())
        });
    }
}

pub fn write_block<V, I, C, B>(block: &B, file_name: &Path) -> DynamicResult<()> 
    where B: Block<V, I, C>, V: Vertex, I: Interface + Clone, C: Cell
{
    let file_type = GridFileType::from_file_name(file_name)?; 
    match file_type {
        GridFileType::Native | GridFileType::Su2 => write_su2(file_name, block),
    }
    Ok(())
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

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GridFileType {
    Native, Su2,
}

impl GridFileType {
    /// Convert file name to file type
    pub fn from_file_name(file_path: &Path) -> Result<GridFileType, UnknownFileType> {
        let ext = file_path.extension().and_then(OsStr::to_str);
        match ext {
            Some("su2") => Ok(GridFileType::Su2),
            Some("grid") => Ok(GridFileType::Native),
            Some(unknown_ext) => Err(UnknownFileType::new(file_path.to_owned(), Some(unknown_ext.to_string()))),
            None => Err(UnknownFileType::new(file_path.to_owned(), None)),
        }
    }

    pub fn extension(&self) -> &str {
        match &self {
            GridFileType::Native => "grid",
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

