use ray::Ray;
use hit::Hit;
use std::option::Option;
use material::Material;

pub trait ModelObject {
    fn material(&self) -> Material;
    fn try_hit(&self, ray: Ray) -> Option<Hit>;
}