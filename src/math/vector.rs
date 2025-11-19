use std::ops::{Add, Sub, Mul, Div};
use std::fmt::{Debug};

// afaik, rust std has square root implemented for only f32 and f64, and no std::ops trait for it either
pub trait Sqrtable { fn sqrt(self) -> Self; }
impl Sqrtable for f32 { fn sqrt(self) -> Self { f32::sqrt(self) } }
impl Sqrtable for f64 { fn sqrt(self) -> Self { f64::sqrt(self) } }

// Maybe just replace with: struct Vector3(f64, f64, f64)?
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3<T>
{
    x: T,
    y: T,
    z: T,
}


impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z,
        }
    }
}

impl<T> Vector3<T>
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T> + Debug + Copy + Clone + Sqrtable,
{
    pub fn len(&self) -> T {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }
}