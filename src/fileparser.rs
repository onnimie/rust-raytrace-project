use std::fs;
use std::str::Lines;
use crate::math::vector::Vector3;
use crate::object::Triangle;


pub fn read_triangles_from_obj(path: &str) -> Vec<Triangle> {
    let contents: String = fs::read_to_string(path).unwrap();
    let lines: Lines<'_> = contents.lines();
    let mut vertices: Vec<Vector3<f64>> = Vec::new();
    let mut vertex_normals: Vec<Vector3<f64>> = Vec::new();
    let mut triangles: Vec<Triangle> = Vec::new();

    for line in lines {

        let mut split = line.split(' ').filter(|p: &&str| *p != "" );
        match split.next() {
            Some(c) => match c {
                "v" => {
                    let x: f64 = split.next().unwrap().parse().unwrap();
                    let y: f64 = split.next().unwrap().parse().unwrap();
                    let z: f64 = split.next().unwrap().parse().unwrap();
                    let vertex: Vector3<f64> = Vector3::new(x, y, z).scaled(10.0);
                    vertices.push(vertex);
                },
                "vn" => {
                    let x: f64 = split.next().unwrap().parse().unwrap();
                    let y: f64 = split.next().unwrap().parse().unwrap();
                    let z: f64 = split.next().unwrap().parse().unwrap();
                    let normal: Vector3<f64> = Vector3::new(x, y, z).normalized();
                    vertex_normals.push(normal);
                }
                "f" => {
                    let mut v_indices: Vec<usize> = vec![];
                    let mut vn_indices: Vec<usize> = vec![];
                    for s in split {
                        let mut vi_vt_vn = s.split('/');
                        let vi: usize = vi_vt_vn.next().unwrap().parse().unwrap();
                        let _vt = vi_vt_vn.next();
                        let vn: usize = vi_vt_vn.next().unwrap().parse().unwrap();
                        v_indices.push(vi);
                        vn_indices.push(vn);
                    }

                    let normal: Vector3<f64> = vertex_normals[vn_indices[0]-1].clone();

                    let mut i: usize = 3;
                    let v1: Vector3<f64> = vertices[v_indices[0]-1].clone();
                    let v2: Vector3<f64> = vertices[v_indices[1]-1].clone();
                    let v3: Vector3<f64> = vertices[v_indices[2]-1].clone();
                    triangles.push(Triangle::new(v1, v2, v3, normal));

                    while i < v_indices.len() {
                        let v1: Vector3<f64> = vertices[v_indices[i-1]-1].clone();
                        let v2: Vector3<f64> = vertices[v_indices[i]-1].clone();
                        let v3: Vector3<f64> = vertices[v_indices[0]-1].clone();
                        triangles.push(Triangle::new(v1, v2, v3, normal));
                        i += 1;
                    }
                }
                _ => {},
            },
            None => {}
        }
    }
    
    triangles
}

