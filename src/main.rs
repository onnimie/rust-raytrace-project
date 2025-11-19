
use raytrace::math::vector::Vector3;

fn main() {
    println!("Test computing a vector magnitude!");

    let vec1: Vector3<f64> = Vector3::new(1.0, 1.0, 1.0);
    let length: f64 = vec1.len();

    println!("The vector: {:?}", vec1);
    println!("The length: {}", length);
}
