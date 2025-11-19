//use crate::math::vector::Vector3;
use crate::object::Object;

pub trait Updatable {
    fn update(&self, dt: f64) -> ();
}


pub struct Scene {
    objects: Vec<Object>,
    active: bool,
}