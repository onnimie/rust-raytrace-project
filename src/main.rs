use std::io::{self};//, Stdout};
use std::f64::consts::PI;
use std::thread;
use std::time::Duration;

use raytrace::camera::Camera;
use raytrace::math::vector::Vector3;
use raytrace::scene::Scene;
use raytrace::terminal::TerminalScreen;

fn main() {

    let mut camera: Camera = 
        Camera::new(Vector3::new(-400.0, 0.0, 0.0),
            Vector3::unit_x(),
            Vector3::unit_y(),
            Vector3::unit_z(),
            0.001,
            PI/4.0);
    
    let scene: Scene = Scene::test_scene();

    let mut terminal_screen: TerminalScreen = TerminalScreen::new(
        io::stdout(),
        202,
        46,
        1.0/2.0
    );

    terminal_screen.init_screen_area().unwrap();
    println!("\n");
    
    loop {
        terminal_screen.render_scene_to_screen_area(&scene, &camera).unwrap();
        camera.move_by(&Vector3::new(20.0, 0.0, 0.0));
        println!("camera pos: {:?}", camera.eyepoint);
        thread::sleep(Duration::from_millis(500));
    }
    
}
