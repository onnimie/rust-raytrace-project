use crate::math::vector::Vector3;
use crate::object::{Object};

pub trait Updatable {
    fn update(&self, dt: f64) -> ();
}


pub struct Scene {
    pub objects: Vec<Object>,
    pub active: bool,
}



impl Scene {
    fn update(&self, dt: f64) -> () {}

    fn test_scene() -> Self {

        let test_ball: Object = Object::test_ball(10.0, Vector3::new(0.0, 0.0, 0.0));

        let test_scene: Scene = Scene {
            objects: vec![test_ball],
            active: true,
        };

        return test_scene;
    }
}