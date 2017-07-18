use vector::Vector;
use model_object::ModelObject;
use color::Color;

pub struct Hit<'a> {
    pub distance: f64,
    pub hit_normal: Vector,
    pub hit_point: Vector,
    pub object: &'a ModelObject,
}

impl<'a> Hit<'a> {
    pub fn new(distance: f64,
               hit_normal: Vector,
               hit_point: Vector,
               object: &ModelObject) -> Hit {
        Hit {
            distance: distance,
            hit_normal: hit_normal,
            hit_point: hit_point,
            object: object
        }
    }

    pub fn get_direct_color(&self) -> Color {
        //TODO
        self.object.material().diffuse_color
    }

    pub fn get_reflection_color(&self, recursion_level: u32) -> Color {
        Color::new(0.0, 0.0, 0.0) //TODO
    }
}