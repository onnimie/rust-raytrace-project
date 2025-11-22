use crate::math::vector::Vector3;
use crate::scene::Scene;
use crate::phong::{self, Material};

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3<f64>,
    pub dir: Vector3<f64>,
    pub tmin: f64,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, dir: Vector3<f64>, tmin: f64) -> Self {
        Self {
            origin,
            dir,
            tmin,
        }
    }


    pub fn trace(&self, scene: &Scene) -> Option<Vector3<f64>> {

        // find objects from the scene reference
        // compute objects' intersections with the given ray
        // find the nearest intersection (minimal t)
        // create a corresponding RayHit
        // compute the lighting on the hit, and return the color (illumination)
        
        let test_ball = scene.objects.first().unwrap();
        let rayhit = test_ball.intersect(self);

        //if self.dir == Vector3::UnitX() {
            //dbg!(&test_ball);
            //dbg!(&ret);
        //}
        match rayhit {
            Some(hit) => Some(phong::compute_phong_illumination(&hit, &self, scene)),
            None => None
        }
    }
}

#[derive(Debug)]
pub struct RayHit {
    pub t: f64,
    pub pos: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub material: Material,
}

impl RayHit {
    pub fn new(t: f64, pos: Vector3<f64>, normal: Vector3<f64>, material: Material) -> Self {
        Self {
            t,
            pos,
            normal,
            material,
        }
    }
}
