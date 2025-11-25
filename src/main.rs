use std::io::{self, Stdin, Read};//, Stdout};
use std::f64::consts::PI;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::time::Duration;
use std::env;

use raytrace::camera::Camera;
use raytrace::math::vector::Vector3;
use raytrace::scene::Scene;
use raytrace::terminal::TerminalScreen;

fn main() {

    let args: Vec<String> = env::args().collect();
    //dbg!(args);
    let mut terminal_w: u64 = 202;
    let mut terminal_h: u64 = 46;
    let mut terminal_line_to_monospace_aspect_ratio: f64 = 0.5;
    if args.len() >= 3 {
        terminal_w = args[1].parse().unwrap();
        terminal_h = args[2].parse().unwrap();
    }
    if args.len() == 4 {
        terminal_line_to_monospace_aspect_ratio = args[3].parse().unwrap();
    }

    let mut camera: Camera = 
        Camera::new(Vector3::new(-220.0, 0.0, 0.0),
            Vector3::unit_x(),
            Vector3::unit_y(),
            Vector3::unit_z(),
            0.001,
            PI/4.0);
    
    let mut scene: Scene = Scene::test_scene();

    let mut terminal_screen: TerminalScreen = TerminalScreen::new(
        io::stdout(),
        terminal_w,
        terminal_h,
        terminal_line_to_monospace_aspect_ratio
    );

    terminal_screen.init_screen_area().unwrap();
    println!("\n");
    let (tx, rx): (Sender<bool>, Receiver<u8>) = test();

    let mut param_t: f64 = 0.0;
    
    loop {
        tx.send(false).unwrap();

        match rx.try_recv() {
            Ok(byte) => match byte {
                119 => camera.move_by(&Vector3::new(50.0, 0.0, 0.0)),
                97 => camera.move_by(&Vector3::new(0.0, -50.0, 0.0)),
                115 => camera.move_by(&Vector3::new(-50.0, 0.0, 0.0)),
                100 => camera.move_by(&Vector3::new(0.0, 50.0, 0.0)),
                _ => (),
            },
            Err(_) => (),
        }

        terminal_screen.render_scene_to_screen_area(&scene, &camera).unwrap();
        tx.send(true).unwrap();
        
        scene.point_lights[0].move_by(&Vector3::new(0.0, 10.0, 0.0));
        scene.point_lights[0].move_to_pos(Vector3::new(
            300.0 * param_t.cos(),
            300.0 * param_t.sin(),
            100.0 * (param_t/2_f64).sin(),
        ));
        //camera.move_by(&Vector3::new(-0.8, 0.0, 0.0));
        println!("camera pos: {:?}", camera.eyepoint);
        param_t += 0.03;
        thread::sleep(Duration::from_millis(20));
    }
    
}

fn test() -> (Sender<bool>, Receiver<u8>) {
    let (tx1, rx1) = std::sync::mpsc::channel();
    let (tx2, rx2) = std::sync::mpsc::channel();
    thread::spawn(move || {
        let mut stdin: Stdin = io::stdin();
        loop {
            match rx2.try_recv() {
                Ok(allow_read) => if allow_read {
                    let mut buf: [u8; 1] = [0];
                    stdin.read_exact(&mut buf).unwrap();
                    tx1.send(buf[0]).unwrap();
                },
                Err(_) => (),
            }
            
        }
    });
    (tx2, rx1)
    /*
    let mut buf: [u8; 4] = [0,0,0,0];
    let mut stdin: Stdin = io::stdin();
    stdin.read_exact(&mut buf).unwrap();

    dbg!(buf);*/
}