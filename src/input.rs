use std::io::{self, Stdin, Read};//, Stdout};
use std::sync::mpsc::{Sender, Receiver};
use std::thread;

use crate::camera::Camera;
use crate::scene::Scene;
use crate::math::vector::Vector3;


pub fn init_input_thread_channels() -> (Sender<bool>, Receiver<u8>) {
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

pub fn handle_input(input_byte: u8, _scene: &mut Scene, camera: &mut Camera) {
    match input_byte {
        /* w */ 119 => camera.move_by(&Vector3::new(50.0, 0.0, 0.0)),
        /* a */ 97 => camera.move_by(&Vector3::new(0.0, -50.0, 0.0)),
        /* s */ 115 => camera.move_by(&Vector3::new(-50.0, 0.0, 0.0)),
        /* d */ 100 => camera.move_by(&Vector3::new(0.0, 50.0, 0.0)),
        _ => (),
    }
}