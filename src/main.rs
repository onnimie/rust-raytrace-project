
use raytrace::math::vector::Vector3;

fn main() {
    println!("Test computing a vector magnitude!");

    let mut vec1: Vector3<f64> = Vector3::new(1.0, 1.0, 1.0);
    let length1: f64 = vec1.len();

    println!("The vector: {:?}", vec1);
    println!("The length: {}", length1);

    vec1.normalize();
    let length2 = vec1.len();
    println!("normalized: {:?}", vec1);
    println!("The length: {}", length2);
}
