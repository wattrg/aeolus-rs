use crate::numerical_methods::number::Real;
use crate::util::vector3::Vector3;

use super::interface::Interface;
use super::vertex::Vertex;
use super::interface::Direction;
use super::geom_calc::{compute_centre_of_vertices, quad_area, triangle_area};

/// The shape of the cell
#[derive(Debug, PartialEq, Eq)]
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

    /// return the number of vertices
    pub fn number_of_vertices(&self) -> usize {
        match &self {
            CellShape::Triangle => 3,
            CellShape::Quadrilateral => 4,
        }
    }

    /// Convert SU2 element type to cell shape
    pub fn from_su2_element_type(elem_type: usize) -> CellShape {
        match elem_type {
            5 => CellShape::Triangle,
            9 => CellShape::Quadrilateral,
            _ => panic!("Invalid, or unsupported su2 element type"),
        }
    }

    pub fn to_su2_element_type(&self) -> usize {
        match &self {
            CellShape::Triangle => 5,
            CellShape::Quadrilateral => 9,
        }
    }

    /// Determine the id's of each of the vertices in each interface
    pub fn interfaces(&self, vertices: &[usize]) -> Vec<Vec<usize>> {
        match &self {
            CellShape::Triangle => {
                vec![
                    vec![vertices[0], vertices[1]],
                    vec![vertices[1], vertices[2]],
                    vec![vertices[2], vertices[0]],
                ]
            }
            CellShape::Quadrilateral => {
                vec![
                    vec![vertices[0], vertices[1]],
                    vec![vertices[1], vertices[2]],
                    vec![vertices[2], vertices[3]],
                    vec![vertices[3], vertices[0]],
                ]
            }
        }
    }

    /// Calculate the volume of the shape given a set of vertices
    pub fn volume(&self, vertices: &[&Vertex]) -> Real {
        match &self {
            CellShape::Triangle => triangle_area(vertices),
            CellShape::Quadrilateral => quad_area(vertices),
        }
    }
}

/// Encodes information about the interface
/// and whether it is inwards or outwards facing
#[derive(Debug, PartialEq, Eq)]
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
#[derive(Debug, PartialEq)]
pub struct Cell{
    vertex_ids: Vec<usize>,
    interfaces: Vec<CellFace>,
    shape: CellShape,
    volume: Real,
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
        let centre = compute_centre_of_vertices(vertices);

        // create the cell faces
        for interface in interfaces.iter() {
            let face_id = interface.id();
            let direction = interface.compute_direction(&centre);
            cell_faces.push(CellFace{interface: face_id, direction});
        }

        let volume = shape.volume(vertices);
    
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
    pub fn volume(&self) -> Real {
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


#[cfg(test)]
mod tests {
    use super::*;

    fn setup_quad() -> (Vec<Vertex>, Vec<Interface>, Cell) {
        let vertices = vec![
            Vertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 0),
            Vertex::new(Vector3{x: 1.0, y: 0.0, z: 0.0}, 1),
            Vertex::new(Vector3{x: 1.0, y: 1.0, z: 0.0}, 2),
            Vertex::new(Vector3{x: 0.0, y: 1.0, z: 0.0}, 3),
        ];

        let interfaces = vec![
            Interface::new_from_vertices(&[&vertices[0], &vertices[1]], 0),
            Interface::new_from_vertices(&[&vertices[2], &vertices[1]], 1),
            Interface::new_from_vertices(&[&vertices[2], &vertices[3]], 2),
            Interface::new_from_vertices(&[&vertices[3], &vertices[0]], 3),
        ];
        let cell = Cell::new(&[&interfaces[0], &interfaces[1], &interfaces[2], &interfaces[3]], 
                             &[&vertices[0], &vertices[1], &vertices[2], &vertices[3]], 
                             0); 

        (vertices, interfaces, cell)
    }

    fn setup_tri() -> (Vec<Vertex>, Vec<Interface>, Cell) {
        let vertices = vec![
            Vertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 0),
            Vertex::new(Vector3{x: 1.0, y: 0.0, z: 0.0}, 1),
            Vertex::new(Vector3{x: 0.5, y: 1.0, z: 0.0}, 2),
        ];

        let interfaces = vec![
            Interface::new_from_vertices(&[&vertices[0], &vertices[1]], 0),
            Interface::new_from_vertices(&[&vertices[2], &vertices[1]], 1),
            Interface::new_from_vertices(&[&vertices[2], &vertices[0]], 2),
        ];
        let cell = Cell::new(&[&interfaces[0], &interfaces[1], &interfaces[2]], 
                             &[&vertices[0], &vertices[1], &vertices[2]], 
                             0); 

        (vertices, interfaces, cell)
    }

    #[test]
    fn cell_faces_quad() {
        let (_vertices, _interfaces, cell) = setup_quad();
        let result = vec![
            CellFace{interface: 0, direction: Direction::Outwards},
            CellFace{interface: 1, direction: Direction::Inwards},
            CellFace{interface: 2, direction: Direction::Outwards},
            CellFace{interface: 3, direction: Direction::Outwards},
        ];

        assert_eq!(cell.cell_faces(), &result);
    }

    #[test]
    fn cell_faces_tri() {
        let (_vertices, _interfaces, cell) = setup_tri();
        let result = vec![
            CellFace{interface: 0, direction: Direction::Outwards},
            CellFace{interface: 1, direction: Direction::Inwards},
            CellFace{interface: 2, direction: Direction::Outwards},
        ];

        assert_eq!(cell.cell_faces(), &result);
    }

    #[test]
    fn shape_quad() {
        let (_vertices, _interfaces, cell) = setup_quad();

        assert_eq!(cell.shape(), &CellShape::Quadrilateral);
    }

    #[test]
    fn shape_tri() {
        let (_vertices, _interfaces, cell) = setup_tri();

        assert_eq!(cell.shape(), &CellShape::Triangle);
    }

    #[test]
    fn quad_volume() {
        let (_vertices, _interfaces, cell) = setup_quad();

        assert_eq!(cell.volume(), 1.0);
    }

    #[test]
    fn tri_volume() {
        let (_vertices, _interfaces, cell) = setup_tri();

        assert_eq!(cell.volume(), 0.5);
    }

    #[test]
    fn vertex_ids() {
        let (_vertices, _interaces, cell) = setup_quad();

        assert_eq!(cell.vertex_ids(), &vec![0, 1, 2, 3]);
    }

    #[test]
    fn centre_tri() {
        let (_vertices, _interfaces, cell) = setup_tri();

        assert_eq!(cell.centre(), &Vector3{x: 0.5, y: 1./3., z: 0.0});
    }

    #[test]
    fn centre_quad() {
        let (_vertices, _interfaces, cell) = setup_quad();
        
        assert_eq!(cell.centre(), &Vector3{x: 0.5, y: 0.5, z: 0.0});
    }
}
