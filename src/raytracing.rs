use crate::math::vector::Vector3;
use crate::scene::Scene;

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


    pub fn trace(&self, scene: &Scene) -> Option<RayHit> {

        // find objects from the scene reference
        // compute objects' intersections with the given ray
        // find the nearest intersection (minimal t)
        // create a corresponding RayHit and return it (None if no intersections)
        
        let test_ball = scene.objects.first().unwrap();
        let ret = test_ball.intersect(self);

        //if self.dir == Vector3::UnitX() {
            //dbg!(&test_ball);
            //dbg!(&ret);
        //}
        ret
    }
}

#[derive(Debug)]
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
