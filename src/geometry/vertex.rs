use crate::util::vector3::Vector3;
use crate::numerical_methods::number::Number;

/// Geometric vertex
pub struct Vertex {
    pos: Vector3,

    id: usize,
}

impl Vertex {
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

    pub fn id(&self) -> usize {
        self.id
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Vertex) -> bool {
        self.id == other.id
    }

    fn ne(&self, other: &Vertex) -> bool {
        self.id != other.id
    }
}
