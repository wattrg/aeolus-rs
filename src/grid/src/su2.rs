use std::path::Path;
use std::io::{Lines, BufReader, BufRead, BufWriter, Write};
use std::fs::File;
use std::collections::HashMap;

use super::block::GridBlock;
use crate::interface::InterfaceCollection;
use crate::{vertex::GridVertex, interface::GridInterface, cell::{GridCell, CellShape}};
use crate::{Vertex, Interface, Cell, Block};
use common::vector3::Vector3;
use common::DynamicResult;

/// Read a GridBlock from and su2 file
pub fn read_su2(file_path: &Path, id: usize) -> DynamicResult<GridBlock> {
    // open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // we are going to iterate line by line, until we reach the end of the file.
    // If we hit a section heading, we will read that section of data. 
    // If we come across a line we don't know what to do with, we'll ignore it
    // (this is consistent with the su2 specification)
    let mut dimensions: Option<usize> = None;
    let mut n_cells: Option<usize> = None;
    let mut vertices: Vec<GridVertex> = vec![];
    let mut cell_connectivity: Vec<Vec<Vec<usize>>> = vec![]; 
    let mut cell_vertices: Vec<Vec<usize>> = vec![];
    let mut boundary_faces: HashMap<String, Vec<Vec<usize>>> = HashMap::new();
    let mut boundaries: HashMap<String, Vec<usize>> = HashMap::new();

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
            let dim = dimensions
                .expect("Number of dimension should be set before vertex coordinates");
            let n_points = parse_key_value_pair::<usize>(line);
            vertices.reserve(n_points);
            for point_i in 0 .. n_points {
                let point_line = next_line(&mut line_iter);
                let coords = parse_vector_from_line_with_dim(&point_line, dim);
                let vertex_pos = Vector3::new_from_vec(coords);
                vertices.push(GridVertex::new(vertex_pos, point_i));                                        
            }
        }

        // the interfaces and cells
        // this part works soley in id's, rather than references. 
        // This let's us read this before the definition of points 
        // if needed.
        else if line.starts_with("NELEM=") {
            let n_elem = parse_key_value_pair::<usize>(line);
            n_cells = Some(n_elem);
            cell_connectivity.reserve(n_elem);
            cell_vertices.reserve(n_elem);
            for _ in 0 .. n_elem {
                let cell_line = next_line(&mut line_iter);
                let cell_definition = parse_vector_from_line::<usize>(&cell_line);
                let shape = CellShape::from_su2_element_type(cell_definition[0]);
                let this_cell_vertices = &cell_definition[1..];
                cell_connectivity.push(shape.interfaces(this_cell_vertices));
                cell_vertices.push(this_cell_vertices.to_vec());
            }
        }

        // boundary conditions
        else if line.starts_with("NMARK=") {
            let n_boundaries = parse_key_value_pair(line);
            for _ in 0 .. n_boundaries {
                let (tag, bndry_faces) = read_boundary(&mut line_iter);
                boundary_faces.insert(tag, bndry_faces);
            }
        }
    }
    // now that we've read the file, we can build the interfaces and cells
    let n_cells = n_cells.expect("Could not find connectivity");
    let mut interfaces = InterfaceCollection::with_capacity(n_cells);
    let mut cells: Vec<GridCell> = Vec::with_capacity(n_cells);
    for (i, cell_interfaces) in cell_connectivity.iter().enumerate() {
        let mut this_cell_interface_ids: Vec<usize> = vec![];
        for interface in cell_interfaces.iter() {
            let interface_vertices: Vec<&GridVertex> = interface
                .iter()
                .map(|vertex_id| &vertices[*vertex_id])
                .collect();
            let interface_id = interfaces.add_or_retrieve(&interface_vertices);
            this_cell_interface_ids.push(interface_id);
        }

        let this_cell_interfaces: Vec<&GridInterface> = this_cell_interface_ids
            .iter()
            .map(|id| interfaces.interface_with_id(*id))
            .collect();
        let this_cell_vertices: Vec<&GridVertex> = cell_vertices[i]
            .iter()
            .map(|id| &vertices[*id])
            .collect();
        cells.push(GridCell::new(&this_cell_interfaces, &this_cell_vertices, i));
    }

    // now we can find the interfaces on the boundaries
    for (tag, faces_on_boundary) in boundary_faces {
        let mut interfaces_on_boundary = Vec::new();
        for vertex_ids_in_face in faces_on_boundary {
            let vertices_in_face: Vec<&GridVertex> = vertex_ids_in_face[1..]
                .iter()
                .map(|id| &vertices[*id])
                .collect();
            let interface_id = interfaces.find_interface(&vertices_in_face);
            interfaces_on_boundary.push(interface_id);
        }
        boundaries.insert(tag, interfaces_on_boundary);
    }
    Ok(GridBlock::new(vertices, interfaces.interfaces(), cells, boundaries, dimensions.unwrap() as u8, id))
}

/// Write a [`Block`] trait object to a su2 file
pub fn write_su2<V, I, C, B>(file_path: &Path, block: &B)
    where B: Block<V, I, C>, C: Cell, I: Interface, V: Vertex
{
    let file = File::create(file_path).unwrap(); 
    let mut buffer = BufWriter::new(file);

    // the number of dimensions
    writeln!(buffer, "NDIME={}", block.dimensions()).unwrap();

    // the position of the vertices
    writeln!(buffer, "NPOIN={}", block.vertices().len()).unwrap();
    for vertex in block.vertices().iter() {
        write!(buffer, "{}", vertex.pos().x).unwrap();
        write!(buffer, " {}", vertex.pos().y).unwrap();
        if block.dimensions() == 3 {
            write!(buffer, " {}", vertex.pos().z).unwrap();
        }
        writeln!(buffer).unwrap();
    }

    // the connectivity
    writeln!(buffer, "NELEM={}", block.cells().len()).unwrap();
    for cell in block.cells().iter() {
        let element_type = cell.shape().to_su2_element_type();
        write!(buffer, "{}", element_type).unwrap();
        for vertex_id in cell.vertex_ids().iter() {
            write!(buffer, " {}", vertex_id).unwrap();
        }
        writeln!(buffer).unwrap();
    }

    // boundaries
    let interfaces = block.interfaces();
    writeln!(buffer, "NMARK={}", block.boundaries().len()).unwrap();
    for (tag, bndry_interfaces) in block.boundaries().iter() {
        writeln!(buffer, "MARKER_TAG={}", tag).unwrap();
        writeln!(buffer, "MARKER_ELEMS={}", bndry_interfaces.len()).unwrap();
        for interface in bndry_interfaces.iter() {
            let iface = &interfaces[*interface];
            let shape = iface.shape().to_su2_element_type();
            write!(buffer, "{}", shape).unwrap();
            for vertex_id in iface.vertex_ids().iter() {
                write!(buffer, " {}", vertex_id).unwrap();
            }
            writeln!(buffer).unwrap();
        }
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

fn read_boundary(line_iter: &mut Lines<BufReader<File>>) -> (String, Vec<Vec<usize>>) {
    let bndry_line = next_line(line_iter);
    assert!(bndry_line.starts_with("MARKER_TAG"));
    let tag = bndry_line.split_once('=').unwrap().1.to_string();
    let bndry_line = next_line(line_iter);
    assert!(bndry_line.starts_with("MARKER_ELEMS"));
    let number_interfaces = parse_key_value_pair::<usize>(&bndry_line);
    let mut bndry_interfaces: Vec<Vec<usize>> = Vec::with_capacity(number_interfaces);
    for _ in 0 .. number_interfaces {
        let bndry_line = next_line(line_iter);
        bndry_interfaces.push(parse_vector_from_line(&bndry_line));
    }
    (tag, bndry_interfaces)
}

fn next_line(line_iter: &mut Lines<BufReader<File>>) -> String {
    line_iter.next()
        .unwrap()
        .unwrap()
        .trim()
        .to_string()
}
