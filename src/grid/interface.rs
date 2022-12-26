use crate::grid::vertex::Vertex;
use crate::util::vector3::Vector3;
use crate::numerical_methods::number::Number;
use super::geom_calc::compute_centre_of_vertices;

/// Allowable interface shapes
#[derive(Debug, PartialEq, Eq)]
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
    pub fn area(&self, vertices: &[&Vertex]) -> Number {
        match &self {
            InterfaceShape::Line => vertices[0].vector_to(vertices[1])
                                               .length(),
        }
    }
}

/// Describes if the interface is point inwards
/// or outwards for a particular cell
#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Inwards, Outwards,
}

/// A geometric interface
#[derive(Debug)]
pub struct Interface {
    vertex_ids: Vec<usize>,
    area: Number,
    n: Vector3,
    t1: Vector3,
    t2: Vector3,
    centre: Vector3,
    shape: InterfaceShape,
    id: usize,
}

impl Interface {
    /// Create an interface from a vector of vertices
    /// 
    /// # Parameters
    /// * `vertices`: The vertices making up the interface
    ///
    /// * `id`: The id of the interface
    pub fn new_from_vertices(vertices: &[&Vertex], id: usize) -> Interface {
        let t1: Vector3;
        let t2: Vector3;
        let n: Vector3;
        let area: Number;

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

        Interface{vertex_ids, area, n, t1, t2, shape, centre, id}
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
    pub fn vertex_ids(&self) -> &Vec<usize> {
        &self.vertex_ids 
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

    pub fn id(&self) -> usize {
        self.id
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

    fn equal_to_vertices(&self, other: &[&Vertex]) -> bool {
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
}

impl PartialEq<Vec<&Vertex>> for Interface {
    fn eq(&self, vertices:&Vec<&Vertex>) -> bool {
        self.equal_to_vertices(vertices)
    }
}

impl PartialEq for Interface {
    fn eq(&self, other: &Interface) -> bool {
        self.id == other.id
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal() {
        let vertices = vec![
            Vertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 0),
            Vertex::new(Vector3{x: 1.0, y: 0.0, z: 0.0}, 1),
            Vertex::new(Vector3{x: 0.0, y: 1.0, z: 0.0}, 2),
            Vertex::new(Vector3{x: 1.0, y: 1.0, z: 0.0}, 3),
        ];
        let id = 1;
        let face1 = Interface::new_from_vertices(&[&vertices[1], &vertices[2]], id); 

        let interface_vertices = vec![
            &vertices[1],
            &vertices[2],
        ];

        assert!(face1.equal_to_vertices(&interface_vertices));
    }

    #[test]
    fn not_equal() {
        let vertices = vec![
            Vertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 0),
            Vertex::new(Vector3{x: 1.0, y: 0.0, z: 0.0}, 1),
            Vertex::new(Vector3{x: 0.0, y: 1.0, z: 0.0}, 2),
            Vertex::new(Vector3{x: 1.0, y: 1.0, z: 0.0}, 3),
        ];
        let id = 1;
        let face1 = Interface::new_from_vertices(&[&vertices[1], &vertices[2]], id); 

        let interface_vertices = vec![
            &vertices[2],
            &vertices[3],
        ];

        assert!(!face1.equal_to_vertices(&interface_vertices));
    }

    #[test]
    fn partial_eq() {
        let vertices = vec![
            Vertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 0),
            Vertex::new(Vector3{x: 1.0, y: 0.0, z: 0.0}, 1),
            Vertex::new(Vector3{x: 0.0, y: 1.0, z: 0.0}, 2),
            Vertex::new(Vector3{x: 1.0, y: 1.0, z: 0.0}, 3),
        ];
        let id = 1;
        let face1 = Interface::new_from_vertices(&[&vertices[1], &vertices[2]], id); 

        let interface_vertices = vec![
            &vertices[1],
            &vertices[2],
        ];

        assert_eq!(face1, interface_vertices);
    }

    #[test]
    fn partial_ne() {
        let vertices = vec![
            Vertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 0),
            Vertex::new(Vector3{x: 1.0, y: 0.0, z: 0.0}, 1),
            Vertex::new(Vector3{x: 0.0, y: 1.0, z: 0.0}, 2),
            Vertex::new(Vector3{x: 1.0, y: 1.0, z: 0.0}, 3),
        ];
        let id = 1;
        let face1 = Interface::new_from_vertices(&[&vertices[1], &vertices[2]], id); 

        let interface_vertices = vec![
            &vertices[0],
            &vertices[1],
        ];

        assert_ne!(face1, interface_vertices);
    }

    #[test]
    fn area() {
        let vertices = vec![
            Vertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 0),
            Vertex::new(Vector3{x: 1.0, y: 1.0, z: 0.0}, 1),
        ];
        let interface = Interface::new_from_vertices(&[&vertices[0], &vertices[1]], 0);

        assert_eq!(interface.area(), Number::sqrt(2.));
    }

    #[test]
    fn norm() {
        let vertices = vec![
            Vertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 0),
            Vertex::new(Vector3{x: 1.0, y: 1.0, z: 0.0}, 1),
        ];
        let interface = Interface::new_from_vertices(&[&vertices[0], &vertices[1]], 0);

        let norm = Vector3{x: 1./Number::sqrt(2.), y: -1./Number::sqrt(2.), z: 0.0};
        assert_eq!(interface.norm(), norm);
    }

    #[test]
    fn t1() {
        let vertices = vec![
            Vertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 0),
            Vertex::new(Vector3{x: 1.0, y: 1.0, z: 0.0}, 1),
        ];
        let interface = Interface::new_from_vertices(&[&vertices[0], &vertices[1]], 0);
        let t1 = Vector3{x: 1.0/Number::sqrt(2.), y: 1./Number::sqrt(2.), z: 0.0};

        assert_eq!(interface.t1(), t1);
    }

    #[test]
    fn t2() {
        let vertices = vec![
            Vertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 0),
            Vertex::new(Vector3{x: 1.0, y: 1.0, z: 0.0}, 1),
        ];
        let interface = Interface::new_from_vertices(&[&vertices[0], &vertices[1]], 0);
        let t2 = Vector3{x: 0.0, y: 0.0, z: 1.0};

        assert_eq!(interface.t2(), t2);
    }

    #[test]
    fn shape() {
        let vertices = vec![
            Vertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 0),
            Vertex::new(Vector3{x: 1.0, y: 1.0, z: 0.0}, 1),
        ];
        let interface = Interface::new_from_vertices(&[&vertices[0], &vertices[1]], 0);

        assert_eq!(interface.shape(), &InterfaceShape::Line);
    }

    #[test]
    fn dimensions() {
        let vertices = vec![
            Vertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 0),
            Vertex::new(Vector3{x: 1.0, y: 1.0, z: 0.0}, 1),
        ];
        let interface = Interface::new_from_vertices(&[&vertices[0], &vertices[1]], 0);

        assert_eq!(interface.dimensions(), 2);
    }

    #[test]
    fn compute_direction_outwards() {
        let vertices = vec![
            Vertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 0),
            Vertex::new(Vector3{x: 1.0, y: 1.0, z: 0.0}, 1),
        ];
        let interface = Interface::new_from_vertices(&[&vertices[0], &vertices[1]], 0);
        let centre = Vector3{x: 0.4, y: 0.6, z: 0.0};

        assert_eq!(interface.compute_direction(&centre), Direction::Outwards);
    }

    #[test]
    fn compute_direction_inwards() {
        let vertices = vec![
            Vertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 0),
            Vertex::new(Vector3{x: 1.0, y: 1.0, z: 0.0}, 1),
        ];
        let interface = Interface::new_from_vertices(&[&vertices[0], &vertices[1]], 0);
        let centre = Vector3{x: 0.6, y: 0.4, z: 0.0};

        assert_eq!(interface.compute_direction(&centre), Direction::Inwards);
    }

    #[test]
    fn vertex_ids() {
        let vertices = vec![
            Vertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 3),
            Vertex::new(Vector3{x: 1.0, y: 1.0, z: 0.0}, 1),
        ];
        let interface = Interface::new_from_vertices(&[&vertices[0], &vertices[1]], 0);

        assert_eq!(interface.vertex_ids(), &vec![3, 1]);
    }
}
