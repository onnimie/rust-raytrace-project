use crate::math::vector::Vector3;
use crate::scene::Scene;

pub struct Ray {
    origin: Vector3<f64>,
    dir: Vector3<f64>,
}


pub struct RayHit {
    t: f64,
    pos: Vector3<f64>,
    material: String,
}


pub fn trace(ray: &Ray, scene: &Scene, tmin: f64) -> Option<RayHit> {

    // find objects from the scene reference
    // compute objects' intersections with the given ray
    // find the nearest intersection (minimal t)
    // create a corresponding RayHit and return it (None if no intersections)
    None
}