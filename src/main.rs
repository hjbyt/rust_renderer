mod vector;
mod sphere;

fn main() {
    let s = sphere::Sphere { center: vector::ZERO, radius: 1.0 };
    println!("{:?}", s);
}
