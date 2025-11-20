use crate::math::vector::Vector3;
use crate::math::matrix::Matrix4x4;
use crate::ray::{Ray, RayHit};

#[derive(PartialEq)]
pub enum Shape {
    Ball,
    Cube,
}


pub struct Object {
    name: String,
    shape: Shape,
    size: f64,
    pos: Vector3<f64>,
    velocity: Vector3<f64>,
    acceleration: Vector3<f64>,
    rotation: Matrix4x4,
    rot_velocity: Vector3<f64>,
    rot_acceleration: Vector3<f64>,
    mass: f64,
}

impl Object {
    pub fn test_ball(size: f64, velocity: Vector3<f64>) -> Object {
        Object {
            name: String::from("testipallo"),
            shape: Shape::Ball,
            size,
            pos: Vector3::new(0.0, 0.0, 0.0),
            velocity, 
            acceleration: Vector3::new(0.0, 0.0, 0.0),
            rotation: Matrix4x4::zeroes(),
            rot_velocity: Vector3::new(0.0, 0.0, 0.0),
            rot_acceleration: Vector3::new(0.0, 0.0, 0.0),
            mass: 1.0,
        }
    }


    pub fn intersect(&self, ray: &Ray) -> Option<RayHit> {

        if self.shape == Shape::Ball {
            // (x-x0)^2 + (y-y0)^2 + (z-z0)^2 = R^2
            // solve for t for intersections: t^2(di^2+dj^2+dk^2) + 2t(adi+bdj+cdk) + (a^2+b^2+c^2) - R^2 = 0
            // simpler: f*t^2 + g*t + h = 0
            
            let a: f64 = ray.origin.x - self.pos.x;
            let b: f64 = ray.origin.y - self.pos.y;
            let c: f64 = ray.origin.z - self.pos.z;
            let di: f64 = ray.dir.x;
            let dj: f64 = ray.dir.y;
            let dk: f64 = ray.dir.z;
            let r: f64 = self.size;

            let f: f64 = di.sqrt() + dj.sqrt() + dk.sqrt();
            let g: f64 = 2_f64 * (a*di + b*dj + c*dk);
            let h: f64 = a.sqrt() + b.sqrt() + c.sqrt() - r.sqrt();

            let discriminant: f64 = g.sqrt() - 4_f64*f*h;
            
            // if discr < 0, there are no solutions and the ray does not intersect
            if discriminant < 0.0 {
                return None;
            }

            let t1: f64 = (-g + discriminant.sqrt()) / (2_f64*f);
            let t2: f64 = (-g - discriminant.sqrt()) / (2_f64*f);
            let p1: Vector3<f64> = ray.origin.added(&ray.dir.scaled(t1));
            let p2: Vector3<f64> = ray.origin.added(&ray.dir.scaled(t2));

            if t1 <= 0.0 && t2 <= 0.0 { return None }
            if t1 <= 0.0 {
                let rayhit: RayHit = RayHit::new(t2, p2, String::from("red"));
                return Some(rayhit);
            }
            if t2 <= 0.0 {
                let rayhit: RayHit = RayHit::new(t1, p1, String::from("red"));
                return Some(rayhit);
            }
            // both are positive
            if t2 < t1 {
                let rayhit: RayHit = RayHit::new(t1, p1, String::from("red"));
                return Some(rayhit);
            } else {
                let rayhit: RayHit = RayHit::new(t2, p2, String::from("red"));
                return Some(rayhit);
            }
        }

        None
    }
}
