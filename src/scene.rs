use vector::{self, Vector};
use camera::Camera;
use color::{Color, BLACK};
use model_object::ModelObject;
use material::Material;
use sphere::Sphere;
use plane::Plane;
use color_image::ColorImage;
use ray::Ray;
use hit::Hit;
use light::Light;
use rand::{self, Rng};
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub struct Scene {
    pub background_color: Color,
    pub objects: Vec<Box<ModelObject>>,
    pub camera: Camera,
    pub lights: Vec<Light>,
}

const RAY_SMALL_ADVANCEMENT: f64 = 0.000000001;

const MAX_RECURSION: u32 = 10;

//TODO: move to a scene file
pub fn get_simple_scene() -> Scene {
    let material = Material::new(Color::new(1.0, 0.0, 0.0),
                                 Color::new(1.0, 1.0, 1.0),
                                 BLACK,
                                 30.0,
                                 0.0);
    let material2 = Material::new(Color::new(0.0, 1.0, 0.0),
                                  BLACK,
                                  BLACK,
                                  1.0,
                                  0.0);
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
    let plane = Plane {
        material: material2,
        normal: Vector::new(0.0, 1.0, 0.0),
        offset: -1.0
    };
    let light = Light::new(
        Vector::new(0.0, 1.0, 1.0),
        Color::new(1.0, 1.0, 1.0),
        1.0,
        0.9,
        1.0
    );
    Scene {
        background_color: Color::new(0.0, 1.0, 1.0),
        objects: vec![Box::new(sphere), Box::new(plane)],
        camera: camera,
        lights: vec![light],
    }
}

impl Scene {
    pub fn from_file_path(file_path: &str) -> io::Result<Scene> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let mut camera: Option<Camera> = None;
        let mut settings: Option<(Color, u32, u32, u32)> = None;
        let mut materials: Vec<Material> = Vec::new();
        let mut objects: Vec<Box<ModelObject>> = Vec::new();
        let mut lights: Vec<Light> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let mut parts = &mut line.split_whitespace();
            let item_type = parts.next().unwrap(); //Note: this can't fail because line is not empty
            match item_type {
                "cam" => {
                    camera = Some(Camera::new(
                        parse_vector(parts),
                        parse_vector(parts),
                        parse_vector(parts),
                        parse_f64(parts),
                        parse_f64(parts),
                        500, //TODO
                        500, //TODO
                    ));
                }
                "set" => {
                    settings = Some((
                        parse_color(parts),
                        parse_u32(parts),
                        parse_u32(parts),
                        parse_u32(parts),
                    ));
                }
                "mtl" => {
                    materials.push(Material::new(
                        parse_color(parts),
                        parse_color(parts),
                        parse_color(parts),
                        parse_f64(parts),
                        parse_f64(parts),
                    ));
                }
                "sph" => {
                    objects.push(Box::new(Sphere {
                        center: parse_vector(parts),
                        radius: parse_f64(parts),
                        material: materials[parse_u32(parts) as usize - 1],
                    }));
                }
                "pln" => {
                    objects.push(Box::new(Plane {
                        normal: parse_vector(parts),
                        offset: parse_f64(parts),
                        material: materials[parse_u32(parts) as usize - 1],
                    }));
                }
                "trg" => {
                    unimplemented!();
                    //objects.push(Box::new(Triangle::new(
                    //    parse_vector(parts),
                    //    parse_vector(parts),
                    //    parse_vector(parts),
                    //    parse_uint(parts),
                    //)));
                }
                "lgt" => {
                    lights.push(Light::new(
                        parse_vector(parts),
                        parse_color(parts),
                        parse_f64(parts),
                        parse_f64(parts),
                        parse_f64(parts),
                    ));
                }
                _ => panic!("Unrecognized scene item") //TODO: return error
            }
        }

        let camera = camera.expect("Camera item not found"); //TODO
        let settings = settings.expect("Settings item not found"); //TODO

        Ok(Scene {
            background_color: settings.0,
            objects,
            camera,
            lights,
        })
    }

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
        let mut total_diffuse_component = BLACK;
        let mut total_specular_component = BLACK;
        for light in self.lights.iter() {
            let light_intensity = self.get_light_intensity_for_hit(light, hit);
            if light_intensity == 0.0 {
                continue;
            }

            let light_color = light.color * light_intensity;
            let direction_to_light = hit.hit_point.direction_to(light.position);

            // Diffuse component
            let diffusion = hit.hit_normal % direction_to_light;
            assert!(!diffusion.is_nan() && diffusion <= 1.0);
            let diffusion = diffusion.max(0.0);
            let diffuse_color = light_color * diffusion;
            total_diffuse_component += diffuse_color;

            // Specular component
            if hit.object.material().is_specular() {
                let direction_to_light_reflection = direction_to_light.reflect_around(&hit.hit_normal);
                let cos_angle = direction_to_light_reflection % hit.direction_to_source;
                if cos_angle > 0.0 {
                    let specular = cos_angle.powf(hit.object.material().phong_specularity);
                    let specular_color = (specular * light.specular_intensity) * light_color;
                    total_specular_component += specular_color;
                }
            }
        }
        total_diffuse_component *= hit.object.material().diffuse_color;
        total_specular_component *= hit.object.material().specular_color;

        total_diffuse_component + total_specular_component
    }

    fn get_hit_reflection_color(&self, hit: &Hit, recursion_level: u32) -> Color {
        if !hit.object.material().is_reflective() {
            return BLACK;
        }
        let hit_reflection_direction = hit.direction_to_source.reflect_around(&hit.hit_normal);
        debug_assert!(::utils::almost_eq(hit_reflection_direction.norm(), 1.0));
        let mut reflection_ray = Ray::new(hit.hit_point, hit_reflection_direction);
        // Move reflection exit point forward a bit to avoid numeric issues (hitting the same surface)
        reflection_ray.advance(RAY_SMALL_ADVANCEMENT);
        let reflection_color = self.color_ray_hits(&reflection_ray, recursion_level);
        reflection_color * hit.object.material().reflection_color
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
            if !ray_hit.object.material().is_transparent() {
                ray_intensity = 0.0;
                break;
            }
            // Attenuate intensity
            ray_intensity *= ray_hit.object.material().transparency;
        }
        ray_intensity
    }
}

//TODO: fix error handling
fn parse_f64<'a, I: Iterator<Item=&'a str>>(params: &mut I) -> f64 {
    let param = params.next().expect("not enough params");
    param.parse().expect("expected float")
}

fn parse_u32<'a, I: Iterator<Item=&'a str>>(params: &mut I) -> u32 {
    let param = params.next().expect("not enough params");
    param.parse().expect("expected unsigned integer")
}

fn parse_vector<'a, I: Iterator<Item=&'a str>>(params: &mut I) -> Vector {
    Vector::new(parse_f64(params), parse_f64(params), parse_f64(params))
}

fn parse_color<'a, I: Iterator<Item=&'a str>>(params: &mut I) -> Color {
    Color::new(parse_f64(params), parse_f64(params), parse_f64(params))
}
