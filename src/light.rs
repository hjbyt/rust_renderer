use color::Color;
use vector::Vector;

pub struct Light {
    pub position: Vector,
    pub color: Color,
    pub specular_intensity: f64,
    pub shadow_intensity: f64,
    pub radius: f64,
}

impl Light {
    pub fn new(position: Vector,
               color: Color,
               specular_intensity: f64,
               shadow_intensity: f64,
               radius: f64) -> Light {
        debug_assert!(0.0 <= shadow_intensity && shadow_intensity <= 1.0);
        Light {
            position,
            color,
            specular_intensity,
            shadow_intensity,
            radius
        }
    }
}