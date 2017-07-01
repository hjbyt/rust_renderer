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

extern crate image;

fn main() {
    let mut img = color_image::ColorImage::new(500, 500);
    img[(100, 100)] = color::Color::new(1.0, 1.0, 1.0);
    let img_buf = img.to_image_buffer();
    img_buf.save("output.png").unwrap();
    println!("{:?}", 123);
}
