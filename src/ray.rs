use vector::Vector;

pub struct Ray {
    pub position: Vector,
    pub direction: Vector,
}

impl Ray {
    pub fn new(position: Vector, direction: Vector) -> Ray {
        debug_assert!(::utils::almost_eq(direction.norm(), 1.0));
        Ray { position: position, direction: direction }
    }

    pub fn construct_ray(from: Vector, to: Vector) -> Ray {
        Ray::new(from, from.direction_to(&to))
    }

    pub fn advance(&mut self, by: f64) {
        self.position += self.direction * by;
    }
}