//use std::ops::{Add, Sub, Mul, Div};
use std::fmt::{Debug};

//use super::vector::Sqrtable;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix4x4
{
    elems: [[f64; 4]; 4],

}


impl Matrix4x4 {
    pub fn zeroes() -> Self {
        Self {
            elems: [[0.0; 4]; 4],
        }
    }

    pub fn ones() -> Self {
        Self {
            elems: [[1.0; 4]; 4],
        }
    }

    pub fn identity() -> Self {
        Self {
            elems: [[1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0]],
        }
    }
}