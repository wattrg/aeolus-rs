use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use super::cell::Cell;
use super::vertex::Vertex;
use super::interface::Interface;
use crate::DynamicResult;
use crate::grid::cell::CellShape;
use crate::util::vector3::Vector3;


#[derive(Debug)]
pub struct Block {
    vertices: Vec<Vertex>,
    interfaces: Vec<Interface>,
    cells: Vec<Cell>,
    dimensions: u8,
}

impl Block {
    /// Create a new block from a file. Currently supported file types are:
    /// * su2
    pub fn new(file_name: String) -> DynamicResult<Block> {
        let ext = GridFileType::from_file_name(&file_name)?;
        match ext {
            GridFileType::Su2 => Block::new_from_su2(file_name),
        }
    }

    fn new_from_su2(file_name: String) -> DynamicResult<Block> {
        // open the file
        let file = File::open(&file_name)?;
        let reader = BufReader::new(file);

        // we are going to iterate line by line, until we reach the end of the file.
        // If we hit a section heading, we will read that section of data. 
        // If we come across a line we don't know what to do with, we'll ignore it
        // (this is consistent with the su2 specification)
        let mut dimensions: Option<usize> = None;
        let mut n_cells: Option<usize> = None;
        let mut vertices: Vec<Vertex> = vec![];
        let mut cell_connectivity: Vec<Vec<Vec<usize>>> = vec![]; 
        let mut cell_vertices: Vec<Vec<usize>> = vec![];

        let mut line_iter = reader.lines();
        while let Some(line) = line_iter.next() {
            // clean up the line before we begin
            let line = line?;
            let line = line.trim();

            // the number of spatial dimensions
            if line.starts_with("NDIME=") {
                dimensions = Some(parse_key_value_pair(line));
            }
            
            // the position of each vertex
            else if line.starts_with("NPOIN=") {
                let dim = dimensions.expect("Number of dimension should be set before vertex coordinates");
                let n_points = parse_key_value_pair::<usize>(line);
                vertices.reserve(n_points);
                for point_i in 0 .. n_points {
                    let point_line = line_iter
                        .next()
                        .ok_or("Excepted another point, found EOF")??;
                    let coords = parse_vector_from_line_with_dim(&point_line, dim);
                    let vertex_pos = Vector3::new_from_vec(coords);
                    vertices.push(Vertex::new(vertex_pos, point_i));                                        
                }
            }

            // the interfaces and cells
            // this part works soley in id's, rather than references. This let's us read this 
            // before the definition of points if needed.
            else if line.starts_with("NELEM=") {
                let n_elem = parse_key_value_pair::<usize>(line);
                n_cells = Some(n_elem);
                cell_connectivity.reserve(n_elem);
                cell_vertices.reserve(n_elem);
                for _ in 0 .. n_elem {
                    let cell_line = line_iter.next().ok_or("Excpected another cell, found EOF")??;
                    let cell_definition = parse_vector_from_line::<usize>(&cell_line);
                    let shape = CellShape::from_su2_element_type(cell_definition[0]);
                    let this_cell_vertices = &cell_definition[1..];
                    cell_connectivity.push(shape.interfaces(this_cell_vertices));
                    cell_vertices.push(this_cell_vertices.to_vec());
                }
            }

            // boundary condition data will be read here
        }
        // now that we've read the file, we can build the interfaces and cells
        let n_cells = n_cells.expect("Could not find connectivity");
        let mut interfaces: Vec<Interface> = Vec::with_capacity(n_cells);
        let mut cells: Vec<Cell> = Vec::with_capacity(n_cells);
        for (i, cell_interfaces) in cell_connectivity.iter().enumerate() {
            let mut this_cell_interface_ids: Vec<usize> = vec![];
            for interface in cell_interfaces.iter() {
                let interface_vertices: Vec<&Vertex> = interface
                    .iter()
                    .map(|vertex_id| &vertices[*vertex_id])
                    .collect();
                let interface_id = add_interface(&mut interfaces, &interface_vertices);
                this_cell_interface_ids.push(interface_id);
            }

            let this_cell_interfaces: Vec<&Interface> = this_cell_interface_ids
                .iter()
                .map(|id| &interfaces[*id] )
                .collect();
            let this_cell_vertices: Vec<&Vertex> = cell_vertices[i]
                .iter()
                .map(|id| &vertices[*id])
                .collect();
            cells.push(Cell::new(&this_cell_interfaces, &this_cell_vertices, i));
        }

        Ok(Block{vertices, interfaces, cells, dimensions: dimensions.unwrap() as u8})
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
}

fn parse_key_value_pair<T>(pair: &str) -> T
    where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug
{
    pair.split('=')
        .last().unwrap()
        .trim()
        .parse().unwrap()
}

fn parse_vector_from_line_with_dim<T>(line: &str, dim: usize) -> Vec<T> 
    where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug
{
    line.split(' ')
        .filter(|token| !token.is_empty()) // remove empty tokens
        .take(dim) // take only the first dim tokens
        .map(|token| token.parse().unwrap()) // convert tokens to T
        .collect() // collect into a vector
}

fn parse_vector_from_line<T>(line: &str) -> Vec<T> 
    where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug
{
    line.split(' ')
        .filter(|token| !token.is_empty())
        .map(|token| token.parse().unwrap())
        .collect()
}

fn add_interface(interfaces: &mut Vec<Interface>, vertices: &[&Vertex]) -> usize {
    for interface in interfaces.iter() {
        if interface.equal_to_vertices(vertices) {
            return interface.id();
        }
    }
    interfaces.push(Interface::new_from_vertices(vertices, interfaces.len()));
    interfaces.len() - 1
}

/// For handling errors associated with file types we don't know how to read
#[derive(Debug, PartialEq, Eq)]
pub struct UnknownFileType {
    name: String,
    ext: Option<String>,
}

impl std::error::Error for UnknownFileType {}

impl std::fmt::Display for UnknownFileType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.ext {
            Some(extension) => write!(f, "Unknown extension '{}' for file '{}'", extension, self.name),
            None => write!(f, "No extension to file: {}", self.name),
        }
    }
}

impl UnknownFileType {
    pub fn new(name: String, ext: Option<String>) -> UnknownFileType {
        UnknownFileType { name, ext } 
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum GridFileType {
    Su2,
}

impl GridFileType {
    /// Convert file name to file type
    pub fn from_file_name(file_name: &str) -> Result<GridFileType, UnknownFileType> {
        let ext = file_name.split('.').last();
        match ext {
            Some("su2") => Ok(GridFileType::Su2),
            Some(unknown_ext) => Err(UnknownFileType::new(file_name.to_string(), Some(unknown_ext.to_string()))),
            None => Err(UnknownFileType::new(file_name.to_string(), None)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_file_type() {
        let file_type = GridFileType::from_file_name("grid.su2");

        assert_eq!(file_type, Ok(GridFileType::Su2));
    }

    #[test]
    fn grid_file_type_unknown() {
        let file_type = GridFileType::from_file_name("grid.su3"); 
        let err = UnknownFileType { name: "grid.su3".to_string(), ext: Some("su3".to_string())};
        assert_eq!(file_type, Err(err));
    }

    #[test]
    fn read_su2_file() {
        let block = Block::new("./tests/data/square.su2".to_string()).unwrap();    

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

        assert_eq!(block.vertices(), &vertices);
        assert_eq!(block.interfaces(), &interfaces);
        assert_eq!(block.cells().len(), 9);
    }
}
