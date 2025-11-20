//use crate::math::vector::Vector3;
use crate::object::Object;

pub trait Updatable {
    fn update(&self, dt: f64) -> ();
}


pub struct Scene {
    pub objects: Vec<Object>,
    pub active: bool,
}



impl Scene {
    fn update(&self, dt: f64) -> () {}


}