use std::collections::HashMap;

use crate::vertex::GridVertex;
use common::vector3::Vector3;
use common::number::Real;
use crate::geom_calc::compute_centre_of_vertices;
use crate::{Interface, Id};

/// Allowable interface shapes
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InterfaceShape {
    Line,
}

impl InterfaceShape {
    /// Convert number of vertices to cell shape
    pub fn from_number_of_vertices(n_vertices: u8) -> InterfaceShape {
        match n_vertices {
            0 | 1 => panic!("Not enough vertices to form an interface: {n_vertices}"),
            2 => InterfaceShape::Line,
            _ => panic!("Unsupported number of vertices in interface: {n_vertices}"),
        }
    }

    /// Calculate the area of a shape given a set of vertices
    pub fn area(&self, vertices: &[&GridVertex]) -> Real {
        match &self {
            InterfaceShape::Line => vertices[0].vector_to(vertices[1])
                                               .length(),
        }
    }

    pub fn from_su2_element_type(elem_type: usize) -> InterfaceShape {
        match elem_type {
            3 => InterfaceShape::Line,
            _ => panic!("Invalid or unsupported su2 interface shape"),
        }
    }

    pub fn to_su2_element_type(&self) -> usize {
        match &self {
            InterfaceShape::Line => 3,
        }
    }
}

/// Describes if the interface is point inwards
/// or outwards for a particular cell
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Direction {
    Inwards, Outwards,
}

/// A geometric interface
#[derive(Debug, Clone)]
pub struct GridInterface {
    vertex_ids: Vec<usize>,
    area: Real,
    n: Vector3,
    t1: Vector3,
    t2: Vector3,
    centre: Vector3,
    shape: InterfaceShape,
    id: usize,
}

impl GridInterface {
    /// Create an interface from a vector of vertices
    /// 
    /// # Parameters
    /// * `vertices`: The vertices making up the interface
    ///
    /// * `id`: The id of the interface
    pub fn new_from_vertices(vertices: &[&GridVertex], id: usize) -> GridInterface {
        let t1: Vector3;
        let t2: Vector3;
        let n: Vector3;
        let area: Real;

        // get the id's of the vertices
        let mut vertex_ids = Vec::with_capacity(vertices.len());
        for vertex in vertices.iter() {
            vertex_ids.push(vertex.id());
        }

        // shape specific setup
        let shape = InterfaceShape::from_number_of_vertices(vertex_ids.len() as u8);
        match shape {
            InterfaceShape::Line => {
                let v0v1 = vertices[0].vector_to(vertices[1]);
                t1 = v0v1.normalised();
                t2 = Vector3{x: 0.0, y: 0.0, z: 1.0};
                n = t1.cross(&t2).normalised();
                area = v0v1.length(); // per unit depth
            }
        }

        // compute geometric centre of the cell
        let centre = compute_centre_of_vertices(vertices);

        GridInterface{vertex_ids, area, n, t1, t2, shape, centre, id}
    }

    /// Access the area of the interface
    pub fn area(&self) -> Real {
        self.area
    }

    /// Access the interface normal
    pub fn norm(&self) -> Vector3 {
        self.n
    }

    /// Access the first interface tangent
    pub fn t1(&self) -> Vector3 {
        self.t1
    }

    /// Access the second interface tangent
    pub fn t2(&self) -> Vector3 {
        self.t2
    }

    /// The dimensionality of the interface
    pub fn dimensions(&self) -> u8 {
        match &self.shape {
            InterfaceShape::Line => 2,
        }
    }

    /// Compute if an interface is pointing towards or away from
    /// a point in space
    pub fn compute_direction(&self, point: &Vector3) -> Direction {
        // vector from centre of interface to the point
        let dir = point - &self.centre;

        // the sign of the dot product of dir with the interface
        // normal vector will tell us if the vectors are pointing
        // in the same direction or not
        let dot = dir.dot(&self.n);

        if dot.abs() < 1e-14 {
            panic!("The point is on the interface");
        }

        match dot > 0.0 {
            true => Direction::Inwards,
            false => Direction::Outwards,
        }
    }

    pub fn equal_to_vertices(&self, other: &[&GridVertex]) -> bool {
        for other_vertex in other.iter() {
            let mut has_vertex = false;
            for this_vertex_id in self.vertex_ids.iter() {
                if *this_vertex_id == other_vertex.id() {
                    has_vertex = true;
                    break;
                }
            }
            if !has_vertex {return false;}
        }
        true
    }

    pub fn equal_to_vertex_ids(&self, other: &[usize]) -> bool {
        for other_vertex in other.iter() {
            let mut has_vertex = false;
            for this_vertex_id in self.vertex_ids.iter() {
                if this_vertex_id == other_vertex {
                    has_vertex = true;
                    break;
                }
            }
            if !has_vertex {return false;}
        }
        true
    }
}

impl Interface for GridInterface {
    fn vertex_ids(&self) -> &Vec<usize> {
        &self.vertex_ids 
    }
    
    fn shape(&self) -> &InterfaceShape {
        &self.shape 
    }
}

impl Id for GridInterface {
    fn id(&self) -> usize {
        self.id
    }
}

impl PartialEq<Vec<&GridVertex>> for GridInterface {
    fn eq(&self, vertices:&Vec<&GridVertex>) -> bool {
        self.equal_to_vertices(vertices)
    }
}

impl PartialEq<&[&GridVertex]> for GridInterface {
    fn eq(&self, vertices: &&[&GridVertex]) -> bool {
        self.equal_to_vertices(vertices)
    }
}

impl PartialEq for GridInterface {
    fn eq(&self, other: &GridInterface) -> bool {
        self.id == other.id
    }
}
impl Eq for GridInterface {}

impl PartialOrd for GridInterface {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl Ord for GridInterface {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

/// Handles a collection of interfaces, making sure
/// each one is unique
#[derive(Debug)]
pub struct InterfaceCollection {
    interfaces: HashMap<usize, GridInterface>,
    id_to_hash: HashMap<usize, usize>,
}

impl InterfaceCollection {
    pub fn with_capacity(capacity: usize) -> InterfaceCollection {
        InterfaceCollection { 
            interfaces: HashMap::with_capacity(capacity),
            id_to_hash: HashMap::with_capacity(capacity)
        }
    }

    /// Either adds an interface with the specified vertices to the 
    /// collection, or returns the ID if the interface already exists.
    pub fn add_or_retrieve(&mut self, vertices: &[&GridVertex]) -> usize {
        let vertex_ids: Vec<usize> = vertices.iter().map(|vertex| vertex.id()).collect();
        let hash = hash(&vertex_ids);
        if !self.interfaces.contains_key(&hash) {
            let interface = GridInterface::new_from_vertices(vertices, self.interfaces.len());
            self.id_to_hash.insert(interface.id(), hash);
            self.interfaces.insert(hash, interface);
        }
        self.interfaces[&hash].id()
    }

    pub fn find_interface(&self, vertices: &[&GridVertex]) -> usize {
        let vertex_ids: Vec<usize> = vertices.iter().map(|vertex| vertex.id()).collect();
        let hash = hash(&vertex_ids);
        self.interfaces[&hash].id() 
    }

    pub fn interface_with_id(&self, id: usize) -> &GridInterface {
        let hash = self.id_to_hash[&id];
        &self.interfaces[&hash]
    }

    /// return the interfaces as owned values
    pub fn interfaces(&self) -> Vec<GridInterface> {
        let mut ifaces: Vec<GridInterface> = self.interfaces.values().cloned().collect();
        ifaces.sort();
        ifaces
    }
}

fn hash(vertex_ids: &[usize]) -> usize {
    // the idea is to sort the vertex id's from highest to lowest
    // then concatenate them together to make one large integer
    let mut id_vec = vertex_ids.to_vec();
    id_vec.sort();
    id_vec.reverse();

    let mut hash = "".to_string();
    for id in id_vec {
        hash += &id.to_string();
    }
    hash.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_test() {
        let vertices = &[0, 1, 11];
        let vertices_hash = hash(vertices);

        assert_eq!(vertices_hash, 1110);
    }

    #[test]
    fn equal() {
        let vertices = vec![
            GridVertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 0),
            GridVertex::new(Vector3{x: 1.0, y: 0.0, z: 0.0}, 1),
            GridVertex::new(Vector3{x: 0.0, y: 1.0, z: 0.0}, 2),
            GridVertex::new(Vector3{x: 1.0, y: 1.0, z: 0.0}, 3),
        ];
        let id = 1;
        let face1 = GridInterface::new_from_vertices(&[&vertices[1], &vertices[2]], id); 

        let interface_vertices = vec![
            &vertices[1],
            &vertices[2],
        ];

        assert!(face1.equal_to_vertices(&interface_vertices));
    }

    #[test]
    fn not_equal() {
        let vertices = vec![
            GridVertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 0),
            GridVertex::new(Vector3{x: 1.0, y: 0.0, z: 0.0}, 1),
            GridVertex::new(Vector3{x: 0.0, y: 1.0, z: 0.0}, 2),
            GridVertex::new(Vector3{x: 1.0, y: 1.0, z: 0.0}, 3),
        ];
        let id = 1;
        let face1 = GridInterface::new_from_vertices(&[&vertices[1], &vertices[2]], id); 

        let interface_vertices = vec![
            &vertices[2],
            &vertices[3],
        ];

        assert!(!face1.equal_to_vertices(&interface_vertices));
    }

    #[test]
    fn partial_eq() {
        let vertices = vec![
            GridVertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 0),
            GridVertex::new(Vector3{x: 1.0, y: 0.0, z: 0.0}, 1),
            GridVertex::new(Vector3{x: 0.0, y: 1.0, z: 0.0}, 2),
            GridVertex::new(Vector3{x: 1.0, y: 1.0, z: 0.0}, 3),
        ];
        let id = 1;
        let face1 = GridInterface::new_from_vertices(&[&vertices[1], &vertices[2]], id); 

        let interface_vertices = vec![
            &vertices[1],
            &vertices[2],
        ];

        assert_eq!(face1, interface_vertices);
    }

    #[test]
    fn partial_ne() {
        let vertices = vec![
            GridVertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 0),
            GridVertex::new(Vector3{x: 1.0, y: 0.0, z: 0.0}, 1),
            GridVertex::new(Vector3{x: 0.0, y: 1.0, z: 0.0}, 2),
            GridVertex::new(Vector3{x: 1.0, y: 1.0, z: 0.0}, 3),
        ];
        let id = 1;
        let face1 = GridInterface::new_from_vertices(&[&vertices[1], &vertices[2]], id); 

        let interface_vertices = vec![
            &vertices[0],
            &vertices[1],
        ];

        assert_ne!(face1, interface_vertices);
    }

    #[test]
    fn area() {
        let vertices = vec![
            GridVertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 0),
            GridVertex::new(Vector3{x: 1.0, y: 1.0, z: 0.0}, 1),
        ];
        let interface = GridInterface::new_from_vertices(&[&vertices[0], &vertices[1]], 0);

        assert_eq!(interface.area(), Real::sqrt(2.));
    }

    #[test]
    fn norm() {
        let vertices = vec![
            GridVertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 0),
            GridVertex::new(Vector3{x: 1.0, y: 1.0, z: 0.0}, 1),
        ];
        let interface = GridInterface::new_from_vertices(&[&vertices[0], &vertices[1]], 0);

        let norm = Vector3{x: 1./Real::sqrt(2.), y: -1./Real::sqrt(2.), z: 0.0};
        assert_eq!(interface.norm(), norm);
    }

    #[test]
    fn t1() {
        let vertices = vec![
            GridVertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 0),
            GridVertex::new(Vector3{x: 1.0, y: 1.0, z: 0.0}, 1),
        ];
        let interface = GridInterface::new_from_vertices(&[&vertices[0], &vertices[1]], 0);
        let t1 = Vector3{x: 1.0/Real::sqrt(2.), y: 1./Real::sqrt(2.), z: 0.0};

        assert_eq!(interface.t1(), t1);
    }

    #[test]
    fn t2() {
        let vertices = vec![
            GridVertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 0),
            GridVertex::new(Vector3{x: 1.0, y: 1.0, z: 0.0}, 1),
        ];
        let interface = GridInterface::new_from_vertices(&[&vertices[0], &vertices[1]], 0);
        let t2 = Vector3{x: 0.0, y: 0.0, z: 1.0};

        assert_eq!(interface.t2(), t2);
    }

    #[test]
    fn shape() {
        let vertices = vec![
            GridVertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 0),
            GridVertex::new(Vector3{x: 1.0, y: 1.0, z: 0.0}, 1),
        ];
        let interface = GridInterface::new_from_vertices(&[&vertices[0], &vertices[1]], 0);

        assert_eq!(interface.shape(), &InterfaceShape::Line);
    }

    #[test]
    fn dimensions() {
        let vertices = vec![
            GridVertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 0),
            GridVertex::new(Vector3{x: 1.0, y: 1.0, z: 0.0}, 1),
        ];
        let interface = GridInterface::new_from_vertices(&[&vertices[0], &vertices[1]], 0);

        assert_eq!(interface.dimensions(), 2);
    }

    #[test]
    fn compute_direction_outwards() {
        let vertices = vec![
            GridVertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 0),
            GridVertex::new(Vector3{x: 1.0, y: 1.0, z: 0.0}, 1),
        ];
        let interface = GridInterface::new_from_vertices(&[&vertices[0], &vertices[1]], 0);
        let centre = Vector3{x: 0.4, y: 0.6, z: 0.0};

        assert_eq!(interface.compute_direction(&centre), Direction::Outwards);
    }

    #[test]
    fn compute_direction_inwards() {
        let vertices = vec![
            GridVertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 0),
            GridVertex::new(Vector3{x: 1.0, y: 1.0, z: 0.0}, 1),
        ];
        let interface = GridInterface::new_from_vertices(&[&vertices[0], &vertices[1]], 0);
        let centre = Vector3{x: 0.6, y: 0.4, z: 0.0};

        assert_eq!(interface.compute_direction(&centre), Direction::Inwards);
    }

    #[test]
    fn vertex_ids() {
        let vertices = vec![
            GridVertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 3),
            GridVertex::new(Vector3{x: 1.0, y: 1.0, z: 0.0}, 1),
        ];
        let interface = GridInterface::new_from_vertices(&[&vertices[0], &vertices[1]], 0);

        assert_eq!(interface.vertex_ids(), &vec![3, 1]);
    }
}
