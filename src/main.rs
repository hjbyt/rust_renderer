#[macro_use]
mod utils;
mod vector;
mod sphere;
mod color;
mod material;
mod ray;
mod hit;
mod model_object;
mod camera;
mod color_image;
mod scene;
mod light;

extern crate image;
extern crate rand;

fn main() {
    let scene = scene::get_simple_scene();
    let color_image = scene.render();
    let image_buffer = color_image.to_image_buffer();
    image_buffer.save("output.png").unwrap();
    println!("Done");
}
