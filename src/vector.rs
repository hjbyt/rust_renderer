use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};

#[derive(Debug, Copy, Clone, Default, PartialEq)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}


//
//
//

impl Vector {
    pub fn zero() -> Vector {
        Vector { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x: x, y: y, z: z }
    }
}

//
//
//

impl Add for Vector {
    type Output = Self;
    fn add(self, other: Vector) -> Vector {
        Vector { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        Vector { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl Mul for Vector {
    type Output = Vector;
    fn mul(self, other: Vector) -> Vector {
        Vector { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z }
    }
}

impl Mul<i64> for Vector {
    type Output = Vector;
    fn mul(self, scalar: i64) -> Vector {
        let s = scalar as f64;
        Vector { x: self.x * s, y: self.y * s, z: self.z * s }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, scalar: f64) -> Vector {
        Vector { x: self.x * scalar, y: self.y * scalar, z: self.z * scalar }
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;
    fn mul(self, vector: Vector) -> Vector {
        let scalar = self;
        Vector { x: vector.x * scalar, y: vector.y * scalar, z: vector.z * scalar }
    }
}

impl Mul<Vector> for i64 {
    type Output = Vector;
    fn mul(self, vector: Vector) -> Vector {
        let scalar = self as f64;
        Vector { x: vector.x * scalar, y: vector.y * scalar, z: vector.z * scalar }
    }
}

impl Div for Vector {
    type Output = Vector;
    fn div(self, other: Vector) -> Vector {
        Vector { x: self.x / other.x, y: self.y / other.y, z: self.z / other.z }
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Vector {
        Vector { x: -self.x, y: -self.y, z: -self.z }
    }
}

//
//
//

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Vector) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, other: Vector) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl MulAssign for Vector {
    fn mul_assign(&mut self, other: Vector) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl DivAssign for Vector {
    fn div_assign(&mut self, other: Vector) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

//
//
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stuff() {
        let a = Vector { x: 1.0, y: 0.0, z: 0.0 };
        let b = Vector { x: 0.0, y: 1.0, z: 0.0 };
        let c = Vector { x: 0.0, y: 0.0, z: 1.0 };
        println!("{:?}", 2 * ((a * 2.5) + (b * 3) + (5.2 * c)));
    }
}
