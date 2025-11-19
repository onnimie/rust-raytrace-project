use crate::math::vector::Vector3;
use crate::math::matrix::Matrix4x4;


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