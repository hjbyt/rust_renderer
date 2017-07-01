use model_object::ModelObject;
use vector::Vector;
use ray::Ray;
use hit::Hit;
use material::Material;
use std::option::Option::{None, Some};

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub material: Material,
    pub center: Vector,
    pub radius: f64
}

impl ModelObject for Sphere {
    fn material(self) -> Material {
        self.material
    }

    fn try_hit(self, ray: Ray) -> Option<Hit> {
        // Geometric method
        let el = self.center - ray.position;
        let t_ca = el % ray.direction;
        if t_ca < 0.0 {
            return None;
        }
        let d_square = el.norm_squared() - (t_ca * t_ca);
        let r_square = self.radius * self.radius;
        if (d_square > r_square) {
            return None;
        }
        let t_hc = (r_square - d_square).sqrt();
        let distance_near = t_ca - t_hc;
        //let distance_far = t_ca + t_hc;
        if distance_near < 0.0 {
            return None;
        }
        let hit_point = ray.position + ray.direction * distance_near;
        let hit_normal = self.center.direction_to(hit_point);
        let hit = Hit::new(distance_near, hit_normal, hit_point);
        Some(hit)
    }
}