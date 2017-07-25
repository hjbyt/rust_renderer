use model_object::ModelObject;
use vector::Vector;
use ray::Ray;
use hit::Hit;
use material::Material;
use std::option::Option::{None, Some};

#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    pub material: Material,
    pub v1: Vector,
    pub v2: Vector,
    pub v3: Vector,

    pub normal: Vector,
    pub offset: f64
}

impl Triangle {
    pub fn new(v1: Vector, v2: Vector, v3: Vector, material: Material) -> Triangle {
        let s1 = v2 - v1;
        let s2 = v3 - v1;
        let normal = (s1 ^ s2).normalized();
        Triangle {
            material,
            v1,
            v2,
            v3,
            normal,
            offset: normal % v1,
        }
    }
}

impl ModelObject for Triangle {
    fn material(&self) -> Material {
        self.material
    }

    fn try_hit(&self, ray: &Ray) -> Option<Hit> {
        let cos_angle = self.normal % ray.direction;
        if cos_angle == 0.0 {
            return None;
        }
        let t = (self.offset - (ray.position % self.normal)) / cos_angle;
        if t < 0.0 {
            return None;
        }

        let hit_point = ray.position + ray.direction * t;
        if !(check_vec_above_plane(ray.direction, ray.position, self.v1, self.v2, self.v3)
            && check_vec_above_plane(ray.direction, ray.position, self.v2, self.v3, self.v1)
            && check_vec_above_plane(ray.direction, ray.position, self.v3, self.v1, self.v2)) {
            return None;
        }
        let hit_normal = if cos_angle > 0.0 {
            -self.normal
        } else {
            self.normal
        };

        Some(Hit::new(ray, t, hit_normal, hit_point, self))
    }
}

fn check_vec_above_plane(vec: Vector,
                         p1: Vector,
                         p2: Vector,
                         p3: Vector,
                         reference_point: Vector) -> bool {
    let v1 = p2 - p1;
    let v2 = p3 - p1;
    let mut n = (v2 ^ v1).normalized();
    if n % (reference_point - p1) < 0.0 {
        n *= -1.0;
    }
    vec % n >= 0.0
}