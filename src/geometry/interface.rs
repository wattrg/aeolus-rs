use crate::geometry::vertex::Vertex;
use crate::util::vector3::Vector3;
use crate::numerical_methods::number::Number;

use super::geom_calc::compute_centre_of_vertices;

/// Allowable interface shapes
pub enum InterfaceShape {
    Line,
}

/// Describes if the interface is point inwards
/// or outwards for a particular cell
pub enum Direction {
    Inwards, Outwards,
}

/// A geometric interface
pub struct Interface<'a> {
    vertices: Vec<&'a Vertex>,
    area: Number,
    n: Vector3,
    t1: Vector3,
    t2: Vector3,
    centre: Vector3,
    shape: InterfaceShape,
}

impl <'a> Interface<'a> {
    /// Create an interface from a vector of vertices
    pub fn new_from_vertices(vertices: Vec<&'a Vertex>) -> Interface<'a> {
        let t1: Vector3;
        let t2: Vector3;
        let n: Vector3;
        let area: Number;
        let shape: InterfaceShape;
        match vertices.len() {
            0 | 1 => panic!("An Interface needs at least two vertices"),
            2 => {
                t1 = vertices[0].vector_to(vertices[1]).normalised();
                t2 = Vector3{x: 0.0, y: 0.0, z: 1.0};
                n = t1.cross(&t2).normalised();
                area = vertices[0].vector_to(vertices[1]).length();
                shape = InterfaceShape::Line;
            }
            _ => panic!("Constructing 3D interface not supported yet"),
        }
        let centre = compute_centre_of_vertices(&vertices);
        Interface{vertices, area, n, t1, t2, shape, centre}
    }

    /// Access the area of the interface
    pub fn area(&self) -> Number {
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

    /// Access the vertices of the interface
    pub fn vertices(&self) -> &Vec<&'a Vertex> {
        &self.vertices 
    }
    
    /// Access the shape of the interface
    pub fn shape(&self) -> &InterfaceShape {
        &self.shape 
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
}

