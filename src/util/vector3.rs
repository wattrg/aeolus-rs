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
        self.x /= factor;
        self.y /= factor;
        self.z /= factor;
    }

    /// Normalise the vector in place
    pub fn normalise(&mut self) {
        let length = self.length();
        self.scale_in_place(length);
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
        let y = self.x*other.z - self.z*other.x;
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
