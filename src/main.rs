#[macro_use]
mod utils;
mod vector;
mod sphere;
mod plane;
mod triangle;
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
extern crate threadpool;
extern crate num_cpus;

fn main() {
    let scene = scene::Scene::from_file_path("scenes/Pool.txt").unwrap(); //TODO
    let color_image = scene.render(num_cpus::get());
    let image_buffer = color_image.to_image_buffer();
    image_buffer.save("output.png").unwrap();
    println!("Done");
}
