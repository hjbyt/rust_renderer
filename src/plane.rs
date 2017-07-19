use model_object::ModelObject;
use vector::Vector;
use ray::Ray;
use hit::Hit;
use material::Material;
use std::option::Option::{None, Some};

pub struct Plane {
    pub material: Material,
    pub normal: Vector,
    pub offset: f64
}

impl ModelObject for Plane {
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
        let hit_point = ray.position + (t * ray.direction);
        let hit_normal = if cos_angle > 0.0 {
            -self.normal
        } else {
            self.normal
        };
        Some(Hit::new(ray, t, hit_normal, hit_point, self))
    }
}
