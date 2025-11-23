use crate::math::vector::Vector3;
use crate::object::{Object};
use crate::phong::{DirectionalLight, PointLight};

pub trait Updatable {
    fn update(&self, dt: f64) -> ();
}


pub struct Scene {
    pub objects: Vec<Object>,
    pub ambient_light_intensity: Vector3<f64>,
    pub directional_lights: Vec<DirectionalLight>,
    pub point_lights: Vec<PointLight>,
    pub active: bool,
}



impl Scene {
    fn _update(&self, _dt: f64) -> () {}

    pub fn test_scene() -> Self {

        let test_ball1: Object = Object::test_ball(100.0, Vector3::new(0.0, 0.0, 0.0));
        let test_ball2: Object = Object::test_ball(30.0, Vector3::new(-110.0, 40.0, 0.0));

        let test_directional_light: DirectionalLight =
            DirectionalLight::new(
                Vector3::unit_y(),
                Vector3::fill(0.0)
            );
        let test_point_light = 
            PointLight::new(
                Vector3::new(-100.0, -300.0, 80.0),
                Vector3::fill(100.0),
                0.0,
                0.005,
                0.005,                                
            );

        let test_scene: Scene = Scene {
            objects: vec![test_ball2, test_ball1],
            ambient_light_intensity: Vector3::new(0.0, 0.0, 0.0),
            directional_lights: vec![test_directional_light],
            point_lights: vec![test_point_light],
            active: true,
        };

        return test_scene;
    }
}