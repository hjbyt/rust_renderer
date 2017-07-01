mod vector;
mod sphere;
mod color;
mod material;
mod ray;
mod hit;

fn main() {
    let s = sphere::Sphere { center: vector::ZERO, radius: 1.0 };
    println!("{:?}", s);
}
