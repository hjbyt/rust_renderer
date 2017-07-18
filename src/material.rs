use color::Color;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Material {
    pub diffuse_color: Color,
    pub specular_color: Color,
    pub reflection_color: Color,
    pub phong_specularity: f64,
    pub transparency: f64,
}

impl Material {
    pub fn new(diffuse_color: Color,
               specular_color: Color,
               reflection_color: Color,
               phong_specularity: f64,
               transparency: f64) -> Material {
        debug_assert!(0.0 <= transparency && transparency <= 1.0);
        Material {
            diffuse_color: diffuse_color,
            specular_color: specular_color,
            reflection_color: reflection_color,
            phong_specularity: phong_specularity,
            transparency: transparency
        }
    }
}