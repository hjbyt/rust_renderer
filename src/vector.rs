use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg, Rem, BitXor};

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}


//
//
//

pub const ZERO: Vector = Vector { x: 0.0, y: 0.0, z: 0.0 };

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x: x, y: y, z: z }
    }

    pub fn dot(a: Vector, b: Vector) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: Vector, b: Vector) -> Vector {
        Vector {
            x: a.y * b.z - a.z * b.y,
            y: b.x * a.z - b.z * a.x,
            z: a.x * b.y - a.y * b.x
        }
    }

    pub fn norm_squared(self) -> f64 {
        self % self
    }

    pub fn norm(self) -> f64 {
        self.norm_squared().sqrt()
    }

    pub fn normalize(&mut self) -> &mut Vector {
        *self *= 1.0 / self.norm();
        self
    }

    pub fn normalized(self) -> Vector {
        let s = 1.0 / self.norm();
        self * s
    }

    pub fn direction_to(self, other: Vector) -> Vector {
        *(other - self).normalize()
    }

    pub fn distance_to(self, other: Vector) -> f64 {
        (other - self).norm()
    }

    pub fn distance(a: Vector, b: Vector) -> f64 {
        a.distance_to(b)
    }

    pub fn almost_equal_to(self, other: Vector, epsilon: f64) -> bool {
        self.distance_to(other) <= epsilon
    }

    pub fn almost_equals(a: Vector, b: Vector, epsilon: f64) -> bool {
        a.almost_equal_to(b, epsilon)
    }

    pub fn reflect_around(&self, other: &Vector) -> Vector {
        let d = *other % *self;
        ((2.0 * d) * *other ) - *self
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

impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl DivAssign for Vector {
    fn div_assign(&mut self, other: Vector) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl Rem for Vector {
    type Output = f64;
    fn rem(self, other: Vector) -> f64 {
        Vector::dot(self, other)
    }
}

impl BitXor for Vector {
    type Output = Vector;
    fn bitxor(self, other: Vector) -> Vector {
        Vector::cross(self, other)
    }
}
