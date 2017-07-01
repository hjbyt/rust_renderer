use vector;
use vector::Vector;
use camera::Camera;
use color::{Color, BLACK};
use model_object::ModelObject;
use material::Material;
use sphere::Sphere;
use color_image::ColorImage;

pub struct Scene {
    pub background_color: Color,
    pub objects: Vec<Box<ModelObject>>,
    pub camera: Camera,
    //    pub lights: Vec<Light>,
}

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
    Scene {
        background_color: BLACK,
        objects: vec![Box::new(sphere)],
        camera: camera,
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
        //TODO
        BLACK
    }
}

