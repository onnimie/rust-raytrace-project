use crate::math::vector::Vector3;
use crate::math::matrix::Matrix4x4;
use crate::phong::Material;
use crate::raytracing::{Ray, RayHit};

#[derive(Debug, PartialEq)]
pub enum Shape {
    Ball,
    Cube,
}

#[derive(Debug)]
pub struct Object {
    _name: String,
    shape: Shape,
    size: f64,
    pos: Vector3<f64>,
    _velocity: Vector3<f64>,
    _acceleration: Vector3<f64>,
    _rotation: Matrix4x4,
    _rot_velocity: Vector3<f64>,
    _rot_acceleration: Vector3<f64>,
    _mass: f64,
}

impl Object {
    pub fn test_ball(size: f64, _velocity: Vector3<f64>) -> Object {
        Object {
            _name: String::from("testipallo"),
            shape: Shape::Ball,
            size,
            pos: Vector3::new(0.0, 0.0, 0.0),
            _velocity, 
            _acceleration: Vector3::new(0.0, 0.0, 0.0),
            _rotation: Matrix4x4::zeroes(),
            _rot_velocity: Vector3::new(0.0, 0.0, 0.0),
            _rot_acceleration: Vector3::new(0.0, 0.0, 0.0),
            _mass: 1.0,
        }
    }


    pub fn intersect(&self, ray: &Ray) -> Option<RayHit> {

        if self.shape == Shape::Ball {
            // (x-x0)^2 + (y-y0)^2 + (z-z0)^2 = R^2
            // solve for t for intersections: t^2(di^2+dj^2+dk^2) + 2t(adi+bdj+cdk) + (a^2+b^2+c^2) - R^2 = 0
            // simpler: f*t^2 + g*t + h = 0
            
            /*
            let a: f64 = ray.origin.x - self.pos.x;
            let b: f64 = ray.origin.y - self.pos.y;
            let c: f64 = ray.origin.z - self.pos.z;
            let di: f64 = ray.dir.x;
            let dj: f64 = ray.dir.y;
            let dk: f64 = ray.dir.z;
            let r: f64 = self.size;

            let f: f64 = di*di + dj*dj + dk*dk;
            let g: f64 = 2_f64 * (a*di + b*dj + c*dk);
            let h: f64 = a*a + b*b + c*c - r*r;
            */
            
            let k: Vector3<f64> = ray.origin.subtracted(&self.pos);
            let f: f64 = ray.dir.dot(&ray.dir);
            let g: f64 = 2_f64 * (k.dot(&ray.dir));
            let h: f64 = k.dot(&k) - (self.size * self.size);

            let discriminant: f64 = g*g - 4_f64*f*h;

            //if ray.dir == Vector3::UnitX() {
            //    dbg!(discriminant);
            //}
            
            // if discr < 0, there are no solutions and the ray does not intersect
            if discriminant < 0.0 {
                return None;
            }

            let t1: f64 = (-g + discriminant.sqrt()) / (2_f64*f);
            let t2: f64 = (-g - discriminant.sqrt()) / (2_f64*f);
            let p1: Vector3<f64> = ray.origin.added(&ray.dir.scaled(t1));
            let p2: Vector3<f64> = ray.origin.added(&ray.dir.scaled(t2));
            let n1: Vector3<f64> = p1.subtracted(&self.pos).normalized();
            let n2: Vector3<f64> = p2.subtracted(&self.pos).normalized();

            //if ray.dir == Vector3::UnitX() {
            //    dbg!(t1);
            //    dbg!(t2);
            //}

            if t1 <= ray.tmin && t2 <= ray.tmin { return None }
            if t2 > ray.tmin && (t2 <= t1 || t1 <= ray.tmin) {
                let rayhit: RayHit = RayHit::new(t2, p2, n2, Material::test_material());
                return Some(rayhit);
            }
            if t1 > ray.tmin && (t1 <= t2 || t2 <= ray.tmin) {
                let rayhit: RayHit = RayHit::new(t1, p1, n1, Material::test_material());
                return Some(rayhit);
            }
            return None
        }

        None
    }
}
