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
    /// Create a cell from the surrounding interfaces vertices
    ///
    /// # Parameters
    ///
    /// * `interfaces`: The interfaces making up the cell
    ///
    /// * `vertices`: The vertices making up the cell
    ///
    /// * `id`: The id of the cell
    pub fn new(interfaces: &[&Interface], vertices: &[&Vertex], id: usize) -> Cell {

        let shape = CellShape::from_number_of_vertices(vertices.len() as u8);
        let mut cell_faces = Vec::with_capacity(interfaces.len());

        // temporary vector of references to the actual vertices
        let mut vertex_ids = Vec::with_capacity(vertices.len());
        for vertex in vertices.iter() {
            vertex_ids.push(vertex.id());
        }
        let centre = compute_centre_of_vertices(&vertices);

        // create the cell faces
        for interface in interfaces.iter() {
            let face_id = interface.id();
            let direction = interface.compute_direction(&centre);
            cell_faces.push(CellFace{interface: face_id, direction});
        }

        let volume = match shape {
            CellShape::Triangle => triangle_area(vertices),
            CellShape::Quadrilateral => quad_area(vertices),
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
