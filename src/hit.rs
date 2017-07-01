use vector::Vector;

pub struct Hit {
    pub distance: f64,
    pub hit_normal: Vector,
    pub hit_point: Vector,
}

impl Hit {
    pub fn new(distance: f64,
               hit_normal: Vector,
               hit_point: Vector) -> Hit {
        Hit { distance: distance, hit_normal: hit_normal, hit_point: hit_point }
    }
}