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
    pub x: T,
    pub y: T,
    pub z: T,
}


impl<T> Vector3<T>
where
    T: Copy + Clone
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    pub fn new_from_arr(elems: [T; 3]) -> Self {
        Self {
            x: elems[0],
            y: elems[1],
            z: elems[2],
        }
    }

    pub fn elems(&self) -> [T; 3] {
        [self.x, self.y, self.z]
    }
}

impl Vector3<f64> {
    pub fn unit_y() -> Self {
        Self {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }

    pub fn unit_x() -> Self {
        Self {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn unit_z() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 1.0,
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

    pub fn dot(&self, other: &Self) -> T {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn scaled(&self, scalar: T) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn scale(&mut self, scalar: T) {
        self.x = self.x * scalar;
        self.y = self.y * scalar;
        self.z = self.z * scalar;
    }

    pub fn added(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn add(&mut self, other: &Self) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
    }

    pub fn subtracted(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn subtract(&mut self, other: &Self) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
        self.z = self.z - other.z;
    }

    pub fn normalized(&self) -> Self {
        let leng: T = self.len();
        Self {
            x: self.x / leng,
            y: self.y / leng,
            z: self.z / leng,
        } 
    }

    pub fn normalize(&mut self) {
        let leng: T = self.len();
        self.x = self.x / leng;
        self.y = self.y / leng;
        self.z = self.z / leng;
    }
}