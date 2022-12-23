use crate::numerical_methods::number::Number;
use crate::util::vector3::Vector3;

use super::interface::Interface;
use super::vertex::Vertex;
use super::interface::Direction;
use super::geom_calc::{compute_centre_of_vertices, quad_area, triangle_area};

/// The shape of the cell
pub enum CellShape {
    Triangle,
    Quadrilateral,
}

impl CellShape {
    /// Convert number of vertices to cell shape
    pub fn from_number_of_vertices(n_vertices: u8) -> CellShape {
        match n_vertices {
            0 | 1 | 2 => panic!("Not enough vertices to form a cell: {n_vertices}"),
            3 => CellShape::Triangle,
            4 => CellShape::Quadrilateral,
            _ => panic!("Unsupported number of vertices for cell: {n_vertices}"),
        }
    }
}

/// Encodes information about the interface
/// and whether it is inwards or outwards facing
pub struct CellFace {
    interface: usize,
    direction: Direction,
}

impl CellFace {
    pub fn interface(&self) -> usize {
        self.interface
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}

/// Encodes geometric data about a cell
pub struct Cell{
    vertex_ids: Vec<usize>,
    interfaces: Vec<CellFace>,
    shape: CellShape,
    volume: Number,
    centre: Vector3,
    id: usize,
}

impl Cell {
    /// Create a cell from the surrounding interfaces
    ///
    /// # Parameters
    ///
    /// * `interfaces`: all of the interfaces in the grid
    ///
    /// * `vertices`: all of the vertices in the grid
    ///
    /// * `face_ids`: the id's of the interfaces of the cell
    ///
    /// * `vertex_ids`: the ids of the vertices in the cell
    ///
    /// * `id`: The id of the cell
    pub fn new(interfaces: &[Interface], vertices: &[Vertex], 
               face_ids: Vec<usize>, vertex_ids: Vec<usize>, id: usize) -> Cell {

        let shape = CellShape::from_number_of_vertices(vertices.len() as u8);
        let mut cell_faces = Vec::with_capacity(face_ids.len());

        // temporary vector of references to the actual vertices
        let mut vs: Vec<&Vertex> = Vec::with_capacity(vertex_ids.len());
        for vertex in vertex_ids.iter() {
            vs.push(&vertices[*vertex]);
        }
        let centre = compute_centre_of_vertices(&vs);

        // create the cell faces
        for interface in face_ids.iter() {
            let iface = &interfaces[*interface];
            let direction = iface.compute_direction(&centre);
            cell_faces.push(CellFace{interface: *interface, direction});
        }

        let volume = match shape {
            CellShape::Triangle => triangle_area(&vs),
            CellShape::Quadrilateral => quad_area(&vs),
        };
    
        Cell {
            vertex_ids,
            interfaces: cell_faces,
            shape,
            volume,
            centre,
            id,
        }
    }
    
    /// Access the interfaces surrounding the cell
    pub fn cell_faces(&self) -> &Vec<CellFace> {
        &self.interfaces
    }

    /// Access the shape of the cell
    pub fn shape(&self) -> &CellShape {
        &self.shape
    }

    /// Access the volume of the cell
    pub fn volume(&self) -> Number {
        self.volume
    }
    
    pub fn vertex_ids(&self) -> &Vec<usize> {
        &self.vertex_ids
    }
    
    pub fn centre(&self) -> &Vector3 {
        &self.centre
    }
    
    pub fn id(&self) -> usize {
        self.id
    }
}
