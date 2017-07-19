use vector::Vector;
use model_object::ModelObject;
use ray::Ray;

pub struct Hit<'a> {
    pub distance: f64,
    pub hit_normal: Vector,
    pub hit_point: Vector,
    pub object: &'a ModelObject,
    pub direction_to_source: Vector,
}

impl<'a> Hit<'a> {
    pub fn new(hit_ray: &Ray,
               distance: f64,
               hit_normal: Vector,
               hit_point: Vector,
               object: &'a ModelObject) -> Hit<'a> {
        Hit {
            distance: distance,
            hit_normal: hit_normal,
            hit_point: hit_point,
            object: object,
            direction_to_source: -hit_ray.direction,
        }
    }
}