use crate::geometry::vertex::Vertex;
use crate::util::vector3::Vector3;
use crate::numerical_methods::number::Number;
use super::geom_calc::compute_centre_of_vertices;

/// Allowable interface shapes
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
}

/// Describes if the interface is point inwards
/// or outwards for a particular cell
pub enum Direction {
    Inwards, Outwards,
}

/// A geometric interface
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
    /// `vertices`: All the vertices existing in the grid
    ///
    /// `vertex_ids`: The id's of the vertices in this particular interface
    ///
    /// `id`: The id of the interface
    pub fn new_from_vertices(vertices: &[Vertex], vertex_ids: Vec<usize>, id: usize) -> Interface {
        let t1: Vector3;
        let t2: Vector3;
        let n: Vector3;
        let area: Number;

        // construct temporary vector of vertex references
        let mut vs: Vec<&Vertex> = Vec::with_capacity(vertex_ids.len());
        for vertex in vertex_ids.iter() {
            vs.push(&vertices[*vertex]);
        }

        // shape specific setup
        let shape = InterfaceShape::from_number_of_vertices(vertex_ids.len() as u8);
        match shape {
            InterfaceShape::Line => {
                let v0v1 = vs[0].vector_to(vs[1]);
                t1 = v0v1.normalised();
                t2 = Vector3{x: 0.0, y: 0.0, z: 1.0};
                n = t1.cross(&t2).normalised();
                area = v0v1.length(); // per unit depth
            }
        }

        // compute geometric centre of the cell
        let centre = compute_centre_of_vertices(&vs);

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

    fn equal(&self, other: &[&Vertex]) -> bool {
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
    fn eq(&self, other: &Vec<&Vertex>) -> bool {
        self.equal(other)
    }
}

