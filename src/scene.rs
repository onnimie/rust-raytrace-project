use crate::math::vector::Vector3;
use crate::object::{ObjectWithIntersect, PrimalObject, Triangle, TriangleMesh};
use crate::phong::{DirectionalLight, PointLight};
use crate::fileparser::read_triangles_from_obj;

pub trait Updatable {
    fn update(&self, dt: f64) -> ();
}


pub struct Scene {
    pub objects: Vec<Box<dyn ObjectWithIntersect>>,
    pub ambient_light_intensity: Vector3<f64>,
    pub directional_lights: Vec<DirectionalLight>,
    pub point_lights: Vec<PointLight>,
    pub active: bool,
}



impl Scene {
    fn _update(&self, _dt: f64) -> () {}

    pub fn test_scene() -> Self {

        let test_ball1: PrimalObject = PrimalObject::test_ball(100.0, Vector3::new(0.0, 0.0, 0.0));
        let test_ball2: PrimalObject = PrimalObject::test_ball(30.0, Vector3::new(-110.0, 40.0, 0.0));

        let test_triangle_mesh: TriangleMesh = TriangleMesh { triangles: read_triangles_from_obj("./assets/al.obj") };

        let test_triangle: Triangle = Triangle {
            vertices: [Vector3::new(-80.0, 30.0, -50.0),//
                    Vector3::new(-50.0, -60.0, 80.0),
                    Vector3::new(-110.0, -100.0, -50.0)],
            };

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
            objects: vec![Box::new(test_triangle_mesh)],//vec![Box::new(test_ball2), Box::new(test_ball1), Box::new(test_triangle)],
            ambient_light_intensity: Vector3::new(0.0, 0.0, 0.0),
            directional_lights: vec![test_directional_light],
            point_lights: vec![test_point_light],
            active: true,
        };

        return test_scene;
    }
}