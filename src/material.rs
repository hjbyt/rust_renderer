use color::{Color, BLACK};

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Material {
    pub diffuse_color: Color,
    pub specular_color: Color,
    pub reflection_color: Color,
    pub phong_specularity: f64,
    pub transparency: f64,
    _is_transparent: bool,
    _is_reflective: bool,
    _is_specular: bool,
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
            transparency: transparency,
            _is_transparent : transparency > 0.0,
            _is_reflective: reflection_color != BLACK,
            _is_specular: specular_color != BLACK,
        }
    }

    pub fn is_transparent(&self) -> bool {
        self._is_transparent
    }

    pub fn is_reflective(&self) -> bool {
        self._is_reflective
    }

    pub fn is_specular(&self) -> bool {
        self._is_specular
    }
}
