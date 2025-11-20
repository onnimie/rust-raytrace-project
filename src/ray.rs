use crate::math::vector::Vector3;
use crate::scene::Scene;

pub struct Ray {
    pub origin: Vector3<f64>,
    pub dir: Vector3<f64>,
}


pub struct RayHit {
    pub t: f64,
    pub pos: Vector3<f64>,
    pub material: String,
}

impl RayHit {
    pub fn new(t: f64, pos: Vector3<f64>, material: String) -> Self {
        Self {
            t,
            pos,
            material,
        }
    }
}


pub fn trace(ray: &Ray, scene: &Scene, tmin: f64) -> Option<RayHit> {

    // find objects from the scene reference
    // compute objects' intersections with the given ray
    // find the nearest intersection (minimal t)
    // create a corresponding RayHit and return it (None if no intersections)
    None
}