use vector::{self, Vector};
use camera::Camera;
use color::{Color, BLACK};
use model_object::ModelObject;
use material::Material;
use sphere::Sphere;
use color_image::ColorImage;
use ray::Ray;
use hit::Hit;
use light::Light;
use rand::{self, Rng};

pub struct Scene {
    pub background_color: Color,
    pub objects: Vec<Box<ModelObject>>,
    pub camera: Camera,
    pub lights: Vec<Light>,
}

const MAX_RECURSION: u32 = 10;

//TODO: move to a scene file
pub fn get_simple_scene() -> Scene {
    let material = Material::new(Color::new(1.0, 0.0, 0.0), BLACK, BLACK, 0.0, 0.0);
    let camera = Camera::new(
        vector::ZERO,
        Vector::new(0.0, 0.0, 1.0),
        Vector::new(0.0, 1.0, 0.0),
        1.4,
        1.0,
        500,
        500
    );
    let sphere = Sphere {
        material: material,
        center: Vector::new(0.0, 0.0, 4.0),
        radius: 1.0
    };
    let light = Light::new(
        Vector::new(0.0, 1.0, 1.0),
        Color::new(1.0, 1.0, 1.0),
        30.0,
        1.0,
        1.0
    );
    Scene {
        background_color: BLACK,
        objects: vec![Box::new(sphere)],
        camera: camera,
        lights: vec![light],
    }
}

impl Scene {
    pub fn render(&self) -> ColorImage {
        let width = self.camera.image_width;
        let height = self.camera.image_height;
        let mut color_image = ColorImage::new(width, height);
        for y in 0..height {
            for x in 0..width {
                color_image[(x as usize, y as usize)] = self.render_pixel(x, y);
            }
        }
        color_image
    }

    pub fn render_pixel(&self, x: u32, y: u32) -> Color {
        let ray = self.camera.construct_ray_through_pixel(x, y);
        self.color_ray_hits(&ray, 0)
    }

    pub fn color_ray_hits(&self, ray: &Ray, recursion_level: u32) -> Color {
        let new_recursion_level = recursion_level + 1;
        if new_recursion_level > MAX_RECURSION {
            return self.background_color;
        }
        let hits = self.find_hits(ray);
        self.color_hits(&hits, recursion_level)
    }

    pub fn find_hits(&self, ray: &Ray) -> Vec<Hit> {
        let mut hits: Vec<Hit> = self.objects.iter()
            .filter_map(|object| object.try_hit(ray))
            .collect::<Vec<Hit>>();
        //TODO: handle NANs somehow?
        hits.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
        hits
    }

    pub fn color_hits(&self, hits: &Vec<Hit>, recursion_level: u32) -> Color {
        let mut total_color = BLACK;
        let mut prev_transparency = 1f64;
        for hit in hits {
            let current_transparency = hit.object.material().transparency;
            let direct = self.get_hit_direct_color(hit) * (1f64 - current_transparency);
            let reflection = self.get_hit_reflection_color(hit, recursion_level);
            let color = (direct + reflection) * prev_transparency;
            total_color += color;
            prev_transparency *= current_transparency;
            if current_transparency == 0f64 {
                return total_color;
            }
        }

        total_color + self.background_color * prev_transparency
    }


    fn get_hit_direct_color(&self, hit: &Hit) -> Color {
        //TODO
        hit.object.material().diffuse_color
    }

    fn get_hit_reflection_color(&self, hit: &Hit, recursion_level: u32) -> Color {
        Color::new(0.0, 0.0, 0.0) //TODO
    }

    fn get_light_intensity_for_hit(&self, light: &Light, hit: &Hit) -> f64 {
        let light_direction = hit.hit_point.direction_to(light.position);
        let direction_x = if light_direction.x == 0.0 && light_direction.y == 0.0 {
            Vector::new(1.0, 0.0, 0.0)
        } else {
            // Rotate light direction by 90 degrees around z-axis to get an orthogonal vector
            Vector::new(-light_direction.y, light_direction.x, light_direction.z)
        };
        let direction_y = *(light_direction ^ direction_x).normalize();

        //TODO: get from scene
        let n: u32 = 1;
        let cell_radius = light.radius / n as f64;
        let hald_radius = cell_radius / 2.0;

        let mut total_intensity = 0.0;
        let mut rng = rand::thread_rng();
        for x in 0..n {
            for y in 0..n {
                let x_offset = cell_radius * (x as f64 + rng.next_f64()) - hald_radius;
                let y_offset = cell_radius * (y as f64 + rng.next_f64()) - hald_radius;
                let x_delta = direction_x * x_offset;
                let y_delta = direction_y * y_offset;
                let cell_point = light.position + x_delta + y_delta;
                let ray = Ray::construct_ray(cell_point, hit.hit_point);
                total_intensity += self.get_ray_intensity(hit, &ray);
            }
        }

        let intensity = total_intensity / (n * n) as f64;
        // Interpolate intensity such that the minimum is shadow_intensity
        let intensity = 1.0 - ((1.0 - intensity) * light.shadow_intensity);
        return intensity;
    }

    fn get_ray_intensity(&self, hit: &Hit, ray: &Ray) -> f64 {
        let mut ray_intensity = 1.0;
        let max_hit_distance = hit.hit_point.distance_to(ray.position) + ::utils::EPSILON;
        for ray_hit in self.find_hits(ray) {
            //TODO: make sure this short circuits
            // Check if we got to the given hit or if we passed it
            // (at object edges ray_hit can miss the original ray hit)
            let objects_equal = ray_hit.object as *const _ == hit.object as *const _;
            let hits_almost_equal = objects_equal && ray_hit.hit_point.almost_equal_to(hit.hit_point, ::utils::EPSILON);
            let passed_max_distance = ray_hit.hit_point.distance_to(ray.position) > max_hit_distance;
            if hits_almost_equal || passed_max_distance {
                break;
            }
            // Check if we hit an opaque object
            if ray_hit.object.material().transparency == 0.0 {
                ray_intensity = 0.0;
                break;
            }
            // Attenuate intensity
            ray_intensity *= ray_hit.object.material().transparency;
        }
        ray_intensity
    }
}

