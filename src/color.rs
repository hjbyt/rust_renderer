use std::ops::{Add, Mul, Div, AddAssign, MulAssign, DivAssign};
use std::iter::Sum;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}


//
//
//

pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0 };

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r: r, g: g, b: b }
    }

    pub fn clamped(self) -> Color {
        Color {
            r: self.r.max(0.0).min(1.0),
            g: self.g.max(0.0).min(1.0),
            b: self.b.max(0.0).min(1.0)
        }
    }

    pub fn bytes(self) -> (u8, u8, u8) {
        let c = self.clamped() * 255f64;
        (c.r.round() as u8, c.g.round() as u8, c.b.round() as u8)
    }
}

//
//
//

impl Add for Color {
    type Output = Self;
    fn add(self, other: Color) -> Color {
        Color { r: self.r + other.r, g: self.g + other.g, b: self.b + other.b }
    }
}

impl Mul for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        Color { r: self.r * other.r, g: self.g * other.g, b: self.b * other.b }
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, scalar: f64) -> Color {
        Color { r: self.r * scalar, g: self.g * scalar, b: self.b * scalar }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, vector: Color) -> Color {
        let scalar = self;
        Color { r: vector.r * scalar, g: vector.g * scalar, b: vector.b * scalar }
    }
}

impl Div for Color {
    type Output = Color;
    fn div(self, other: Color) -> Color {
        Color { r: self.r / other.r, g: self.g / other.g, b: self.b / other.b }
    }
}

impl Div<f64> for Color {
    type Output = Color;
    fn div(self, scalar: f64) -> Color {
        Color { r: self.r / scalar, g: self.g / scalar, b: self.b / scalar }
    }
}

//
//
//

impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl MulAssign for Color {
    fn mul_assign(&mut self, other: Color) {
        self.r *= other.r;
        self.g *= other.g;
        self.b *= other.b;
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, scalar: f64) {
        self.r *= scalar;
        self.g *= scalar;
        self.b *= scalar;
    }
}

impl DivAssign for Color {
    fn div_assign(&mut self, other: Color) {
        self.r /= other.r;
        self.g /= other.g;
        self.b /= other.b;
    }
}

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, scalar: f64) {
        self.r /= scalar;
        self.g /= scalar;
        self.b /= scalar;
    }
}

impl Sum for Color {
    fn sum<I: Iterator<Item=Color>>(iter: I) -> Color {
        let mut total_color = BLACK;
        for color in iter {
            total_color += color;
        }
        total_color
    }
}
