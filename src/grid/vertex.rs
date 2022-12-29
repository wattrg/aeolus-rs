use crate::util::vector3::Vector3;
use crate::numerical_methods::number::Number;

/// Geometric vertex
#[derive(Debug)]
pub struct Vertex {
    pos: Vector3,
    id: usize,
}

impl Vertex {
    /// Create a new vertex
    pub fn new(pos: Vector3, id: usize) -> Vertex {
        Vertex{pos, id}
    }

    /// Calculate the distance to another `Vertex`
    pub fn dist_to(&self, other: &Vertex) -> Number {
        self.pos.dist_to(&other.pos)  
    }

    /// Compute the vector from `self` to `other`
    pub fn vector_to(&self, other: &Vertex) -> Vector3 {
        &other.pos - &self.pos
    }

    /// Access the position of the vertex
    pub fn pos(&self) -> &Vector3 {
        &self.pos
    }

    /// The id of the vertex
    pub fn id(&self) -> usize {
        self.id
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Vertex) -> bool {
        self.id == other.id && self.pos == other.pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let vertex = Vertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 0);
        let vertex_ref = Vertex{pos: Vector3{x: 0.0, y:0.0, z: 0.0}, id: 0};

        assert_eq!(vertex, vertex_ref);
    }

    #[test]
    fn dist_to() {
        let vertex1 = Vertex{pos: Vector3{x: 1.0, y: 2.0, z: 3.0}, id: 0};
        let vertex2 = Vertex{pos: Vector3{x: 2.0, y: 3.0, z: 4.0}, id: 1};
        let result = Number::sqrt(3.);
        let dist = vertex1.dist_to(&vertex2);

        assert_eq!(dist, result);
    }

    #[test]
    fn vector_to() {
        let vertex1 = Vertex{pos: Vector3{x: 1.0, y: 2.0, z: 3.0}, id: 0};
        let vertex2 = Vertex{pos: Vector3{x: 2.0, y: 3.0, z: 4.0}, id: 1};
        let result = Vector3{x: 1.0, y: 1.0, z: 1.0};
        let vec = vertex1.vector_to(&vertex2);

        assert_eq!(vec, result);
    }

    #[test]
    fn partial_eq() {
        let vertex1 = Vertex{pos: Vector3{x: 1.0, y: 2.0, z: 3.0}, id: 0};
        let vertex2 = Vertex{pos: Vector3{x: 1.0, y: 2.0, z: 3.0}, id: 0};

        assert_eq!(vertex1, vertex2);
    }

    #[test]
    fn partial_ne() {
        let vertex1 = Vertex{pos: Vector3{x: 1.0, y: 2.0, z: 3.0}, id: 0};
        let vertex2 = Vertex{pos: Vector3{x: 1.0, y: 1.0, z: 4.0}, id: 1};

        assert_ne!(vertex1, vertex2);
    }

    #[test]
    fn pos() {
        let vertex = Vertex{pos: Vector3{x: 1.0, y: 2.0, z: 3.0}, id: 0};

        assert_eq!(vertex.pos(), &Vector3{x: 1.0, y: 2.0, z: 3.0});
    }

    #[test]
    fn id() {
        let vertex = Vertex{pos: Vector3{x: 1.0, y: 2.0, z: 3.0}, id: 1};

        assert_eq!(vertex.id(), 1);
    }
}
