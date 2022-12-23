use crate::numerical_methods::number::Number;
use crate::geometry::interface::Interface;
use crate::geometry::vertex::Vertex;
use crate::util::vector3::Vector3;

use super::geom_calc::{compute_centre_of_vertices, quad_area, triangle_area};
use super::interface::Direction;

/// The shape of the cell
pub enum CellShape {
    Triangle,
    Quadrilateral,
}

impl CellShape {
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
pub struct CellFace<'a> {
    interface: &'a Interface<'a>,
    direction: Direction,
}

impl <'a> CellFace<'a> {
    pub fn interface(&self) -> &'a Interface<'a> {
        self.interface
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}

/// Encodes geometric data about a cell
pub struct Cell<'a> {
    vertices: Vec<&'a Vertex>,
    interfaces: Vec<CellFace<'a>>,
    shape: CellShape,
    volume: Number,
    centre: Vector3,
}

impl<'a> Cell<'a> {
    /// Create a cell from the surrounding interfaces
    pub fn new(interfaces: Vec<&'a Interface<'a>>, vertices: Vec<&'a Vertex>) -> Cell<'a> {
        let shape = CellShape::from_number_of_vertices(vertices.len() as u8);
        let centre = compute_centre_of_vertices(&vertices);
        let mut cell_faces = Vec::with_capacity(interfaces.len());
        for interface in interfaces.iter() {
            let direction = interface.compute_direction(&centre);
            cell_faces.push(CellFace{interface, direction});
        }
        let volume = match shape {
            CellShape::Triangle => triangle_area(&vertices),
            CellShape::Quadrilateral => quad_area(&vertices),
        };
    
        Cell {
            vertices,
            interfaces: cell_faces,
            shape,
            volume,
            centre,
        }
    }
    
    /// Access the interfaces surrounding the cell
    pub fn cell_faces(&self) -> &Vec<CellFace<'a>> {
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
    
    pub fn vertices(&self) -> &Vec<&Vertex> {
        &self.vertices
    }
    
    pub fn centre(&self) -> &Vector3 {
        &self.centre
    }
}
