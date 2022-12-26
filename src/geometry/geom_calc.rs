use crate::numerical_methods::number::Number;
use crate::geometry::vertex::Vertex;
use crate::util::vector3::Vector3;

/// Compute the area of a triangle with given vertices
pub fn triangle_area(vertices: &[&Vertex]) -> Number  {
    debug_assert!(vertices.len() == 3, "Expected 3 points in triangle");

    let a = vertices[0].pos();
    let b = vertices[1].pos();
    let c = vertices[2].pos();
    let tmp = a.x * (b.y - c.y) + b.x*(c.y - a.y) + c.x*(a.y - b.y);

    0.5 * tmp.abs()
}

/// Compute the area of a quadrilateral with given vertices
pub fn quad_area(vertices: &[&Vertex]) -> Number {
    debug_assert!(vertices.len() == 4, "Expected 4 points in quadralateral");

    // use the shoelace formula applied to a quad
    // https://math.stackexchange.com/questions/1259094/coordinate-geometry-area-of-a-quadrilateral
    let a = vertices[0].pos();
    let b = vertices[1].pos();
    let c = vertices[2].pos();
    let d = vertices[3].pos();
    let tmp_plus = a.x*b.y + b.x*c.y + c.x*d.y + d.x*a.y;
    let tmp_minus = b.x*a.y + c.x*b.y + d.x*c.y + a.x*d.y;

    0.5 * (tmp_plus - tmp_minus).abs()
}

pub fn compute_centre_of_vertices(vertices: &[&Vertex]) -> Vector3 {
    let mut centre = Vector3{x: 0.0, y: 0.0, z: 0.0};
    for vertex in vertices.iter() {
        centre += *vertex.pos();
    }       
    centre.scale_in_place(1./vertices.len() as Number);
    centre
}
