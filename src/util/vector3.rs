use crate::numerical_methods::number::Number;
use std::ops;

/// A generic 3 dimensional vector
#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    /// The x component
    pub x: Number,

    /// The y component
    pub y: Number,

    /// The z component
    pub z: Number,
}

impl Vector3 {
    /// Calculate the length of the vector
    pub fn length(&self) -> Number {
        Number::sqrt(self.x*self.x + self.y*self.y + self.z*self.z)
    }

    pub fn scale_in_place(&mut self, factor: Number) {
        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
    }

    /// Normalise the vector in place
    pub fn normalise(&mut self) {
        let length = self.length();
        self.scale_in_place(1. / length);
    }

    /// Return a new normal vector in the same direction as `other`
    pub fn normalised(&self) -> Vector3 {
        let length = self.length();
        Vector3 {
            x: self.x/length, 
            y: self.y/length, 
            z: self.z/length
        }
    }

    /// Add one vector to another, modifying the first one
    pub fn add_in_place(&mut self, amount: &Vector3) {
        self.x += amount.x;
        self.y += amount.y;
        self.z += amount.z;
    }
    
    /// Calculate the distance to another vector
    pub fn dist_to(&self, other: &Vector3) -> Number {
        let x = other.x - self.x;
        let y = other.y - self.y;
        let z = other.z - self.z;

        Number::sqrt(x*x + y*y + z*z)
    }

    pub fn dot(&self, other: &Vector3) -> Number {
        self.x*other.x + self.y*other.y + self.z*other.z
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        let x = self.y*other.z - self.z*other.y;
        let y = self.z*other.x - self.x*other.z;
        let z = self.x*other.y - self.y*other.x;

        Vector3{x, y, z}
    }
}

impl ops::Add for &Vector3 {
    type Output = Vector3;

    fn add(self, other: &Vector3) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl ops::Sub for &Vector3 {
    type Output = Vector3;

    fn sub(self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length() {
        let vec = Vector3{x: 1.0, y: 2.0, z: 3.0};
        assert_eq!(vec.length(), Number::sqrt(14.));
    }

    #[test]
    fn partial_eq() {
        let vec1 = Vector3{x: 1.0, y: 2.0, z:3.0};
        let vec2 = Vector3{x: 1.0, y: 2.0, z:3.0};

        assert_eq!(vec1, vec2);
    }

    #[test] 
    #[should_panic]
    fn partial_eq_different_vecs() {
        let vec1 = Vector3{x: 1.0, y: 2.0, z:3.0};
        let vec2 = Vector3{x: 2.0, y: 2.0, z:3.0};

        assert_eq!(vec1, vec2);
    }

    #[test]
    fn scale() {
        let mut vec = Vector3{x: 1.0, y: 2.0, z: 3.0};
        vec.scale_in_place(0.5);

        assert_eq!(vec, Vector3{x: 0.5, y: 1.0, z: 1.5});
    }

    #[test]
    fn normalise() {
        let mut vec = Vector3{x: 1.0, y: 2.0, z: 3.0};
        vec.normalise();
        let length = Number::sqrt(14.);
        let normalised_vec = Vector3{x: 1./length, y: 2./length, z: 3./length};
        
        assert_eq!(vec, normalised_vec);
    }

    #[test]
    fn normalised() {
        let vec = Vector3{x: 1.0, y: 2.0, z: 3.0};
        let length = Number::sqrt(14.);
        let normalised_vec = Vector3{x: 1./length, y: 2./length, z: 3./length};

        assert_eq!(vec.normalised(), normalised_vec);
    }

    #[test]
    fn add_in_place() {
        let mut vec1 = Vector3{x: 1.0, y: 2.0, z: 3.0};
        let vec2 = Vector3{x: 2.5, y: 3.5, z: 4.5};
        let result = Vector3{x: 3.5, y: 5.5, z: 7.5};
        vec1.add_in_place(&vec2);

        assert_eq!(vec1, result);
    }

    #[test]
    fn dist_to() {
        let vec1 = Vector3{x: 1.0, y: 2.0, z: 3.0};
        let vec2 = Vector3{x: 2.0, y: 3.0, z: 4.0};
        let dist = Number::sqrt(3.);

        assert_eq!(vec1.dist_to(&vec2), dist);
    }

    #[test]
    fn dot() {
        let vec1 = Vector3{x: 1.0, y: 2.0, z: 3.0};
        let vec2 = Vector3{x: 2.0, y: 3.0, z: 4.0};

        assert_eq!(vec1.dot(&vec2), 20.);
    }

    #[test]
    fn cross() {
        let vec1 = Vector3{x: 1.0, y: 2.0, z: 3.0};
        let vec2 = Vector3{x: 2.0, y: 3.0, z: 4.0};
        let result = Vector3{x: -1., y: 2., z: -1.};

        assert_eq!(vec1.cross(&vec2), result);
    }

    #[test]
    fn add() {
        let vec1 = Vector3{x: 1.0, y: 2.0, z: 3.0};
        let vec2 = Vector3{x: 2.0, y: 3.0, z: 4.0};
        let result = Vector3{x: 3.0, y: 5.0, z: 7.0};

        assert_eq!(&vec1 + &vec2, result);
    }

    #[test]
    fn add_assign() {
        let mut vec1 = Vector3{x: 1.0, y: 2.0, z: 3.0};
        let vec2 = Vector3{x: 2.0, y: 3.0, z: 4.0};
        vec1 += vec2; 
        let result = Vector3{x: 3.0, y: 5.0, z: 7.0};

        assert_eq!(vec1, result);
    }

    #[test]
    fn sub() {
        let vec1 = Vector3{x: 1.0, y: 2.0, z: 3.0};
        let vec2 = Vector3{x: 2.0, y: 3.0, z: 4.0};
        let result = Vector3{x: -1.0, y: -1.0, z: -1.0};

        assert_eq!(&vec1 - &vec2, result);
    }
}
