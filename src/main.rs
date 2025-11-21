use std::io::{self, Stdout};
use std::f64::consts::PI;
use std::thread;
use std::time::Duration;

use raytrace::camera::{self, Camera};
use raytrace::math::vector::Vector3;
use raytrace::scene::Scene;
use raytrace::terminal;

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


    println!();


    let mut camera = 
        Camera::new(Vector3::new(-400.0, 0.0, 0.0),
            Vector3::UnitX(),
            Vector3::UnitY(),
            Vector3::UnitZ(),
            0.001,
            PI/4.0);
    
    let scene = Scene::test_scene();

    let mut stdout: Stdout = io::stdout();
    terminal::init_screen_area(&mut stdout).unwrap();
    println!("\n");
    
    loop {
        terminal::render_scene_to_screen_area(&mut stdout, 200, 46, &scene, &camera).unwrap();
        camera.move_by(&Vector3::new(20.0, 0.0, 0.0));
        println!("camera pos: {:?}", camera.eyepoint);
        thread::sleep(Duration::from_millis(500));
        
    }
    
    //terminal::render_scene_to_screen_area(&mut stdout, 200, 46, &scene, &camera).unwrap();

    

    //println!("\n0,0 with 90,20 is {:?}", camera::map_terminal_pixel_pos_to_screen_coord((0,0), 90, 20));
    
}
