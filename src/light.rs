use color::Color;
use vector::Vector;
use hit::Hit;
use rand::{self, Rng};
use ray::Ray;


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

    pub fn get_intensity_for_hit(&self, hit: &Hit) -> f64 {
        let light_directon = hit.hit_point.direction_to(self.position);
        let direction_x = if light_directon.x == 0.0 && light_directon.y == 0.0 {
            Vector::new(1.0, 0.0, 0.0)
        } else {
            // Rotate light direction by 90 degrees around z-axis to get an orthogonal vector
            Vector::new(-light_directon.y, light_directon.x, light_directon.z)
        };
        let direction_y = *(light_directon ^ direction_x).normalize();

        //TODO: get from scene
        let n: u32 = 1;
        let cell_radius = self.radius / n as f64;
        let hald_radius = cell_radius / 2.0;

        let mut total_intensity = 0.0;
        let mut rng = rand::thread_rng();
        for x in 0..n {
            for y in 0..n {
                let x_offset = cell_radius * (x as f64 + rng.next_f64()) - hald_radius;
                let y_offset = cell_radius * (y as f64 + rng.next_f64()) - hald_radius;
                let x_delta = direction_x * x_offset;
                let y_delta = direction_y * y_offset;
                let cell_point = self.position + x_delta + y_delta;
                let ray = Ray::construct_ray(cell_point, hit.hit_point);
                total_intensity += get_ray_intensity(hit, &ray);
            }
        }

        let intensity = total_intensity / (n * n) as f64;
        // Interpolate intensity such that the minimum is shadow_intensity
        let intensity = 1.0 - ((1.0 - intensity) * self.shadow_intensity);
        return intensity;

        fn get_ray_intensity(hit: &Hit, ray: &Ray) -> f64 {
            let mut ray_intensity = 1.0;
            let max_hit_distance = hit.hit_point.distance_to(ray.position) + ::utils::EPSILON;
            //for ray_hit :
            //TODO: continue


            ray_intensity
        }
    }
}