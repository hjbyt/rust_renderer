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
mod ray_tracer;
mod light;

extern crate image;

fn main() {
    let scene = ray_tracer::get_simple_scene();
    let color_image = scene.render();
    let image_buffer = color_image.to_image_buffer();
    image_buffer.save("output.png").unwrap();
    println!("Done");
}
