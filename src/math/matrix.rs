//use std::ops::{Add, Sub, Mul, Div};
use std::fmt::{Debug};
use super::vector::Vector3;

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

    //pub fn prod(&self, vector: Vector3<f64>) {

    //}
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix3x3
{
    elems: [[f64; 3]; 3],
}

impl Matrix3x3 {
    pub fn zeroes() -> Self {
        Self {
            elems: [[0.0; 3]; 3],
        }
    }

    pub fn ones() -> Self {
        Self {
            elems: [[1.0; 3]; 3],
        }
    }

    pub fn identity() -> Self {
        Self {
            elems: [[1.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0],
                    [0.0, 0.0, 1.0]],
        }
    }

    pub fn from_cols(col1: &Vector3<f64>, col2: &Vector3<f64>, col3: &Vector3<f64>) -> Self {
        Self {
            elems: [[col1.x, col2.x, col3.x],
                    [col1.y, col2.y, col3.y],
                    [col1.z, col2.z, col3.z]],
        }
    }

    pub fn row(&self, row_index: usize) -> Vector3<f64> {
        let r: [f64; 3] = self.elems[row_index];
        Vector3 { x: r[0], y: r[1], z: r[2] }
    }

    //pub fn prod(&self, vector: Vector3<f64>) {

    //}

    //pub fn inverse(&self) -> Self {

    //}
}
