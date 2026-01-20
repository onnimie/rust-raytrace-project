use std::fs;
use std::str::Lines;
use crate::math::vector::Vector3;
use crate::object::Triangle;


pub fn read_triangles_from_obj(path: &str) -> Vec<Triangle> {
    let contents: String = fs::read_to_string(path).unwrap();
    let lines: Lines<'_> = contents.lines();
    let mut vertices: Vec<Vector3<f64>> = Vec::new();
    let mut triangles: Vec<Triangle> = Vec::new();

    for line in lines {

        let mut split: std::str::Split<'_, char> = line.split(' ');
        match split.next().unwrap() {
            "v" => {
                let x: f64 = split.next().unwrap().parse().unwrap();
                let y: f64 = split.next().unwrap().parse().unwrap();
                let z: f64 = split.next().unwrap().parse().unwrap();
                let vertex: Vector3<f64> = Vector3::new(x, y, z);
                vertices.push(vertex);
            },
            "f" => {
                let mut v_indices: Vec<usize> = vec![];
                for s in split {
                    let vi: usize = s.parse().unwrap();
                    v_indices.push(vi);
                }

                let mut i: usize = 3;
                let v1: Vector3<f64> = vertices[v_indices[0]-1].clone();
                let v2: Vector3<f64> = vertices[v_indices[1]-1].clone();
                let v3: Vector3<f64> = vertices[v_indices[2]-1].clone();
                triangles.push(Triangle::new(v1, v2, v3));

                while i < v_indices.len() {
                    let v1: Vector3<f64> = vertices[v_indices[i-1]-1].clone();
                    let v2: Vector3<f64> = vertices[v_indices[i]-1].clone();
                    let v3: Vector3<f64> = vertices[v_indices[0]-1].clone();
                    triangles.push(Triangle::new(v1, v2, v3));
                    i += 1;
                }
            }
            _ => {},
        }
    }
    
    triangles
}

