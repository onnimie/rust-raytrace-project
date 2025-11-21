use std::io::{self, Result, Write, Stdout};
use std::thread;
use std::time::Duration;

use crate::camera::{self, Camera};
use crate::raytracing;
use crate::scene::Scene;




fn clear_screen() -> Result<()> {
    let mut stdout: Stdout = io::stdout();

    thread::sleep(Duration::from_secs(2));

    stdout.write_all(b"\x1B[H").unwrap();
    stdout.write_all(b"\x1B[2J").unwrap();

    stdout.flush().unwrap();


    Ok(())
}


pub fn init_screen_area(stdout: &mut Stdout) -> Result<()> {

    stdout.write_all(b"\x1B[2J\x1B[H")?;

    const W: usize = 202;
    const H: usize = 46;

    let wst: String = String::from_iter(['='; W].iter());
    let wbuf: &[u8] = wst.as_bytes();

    let hst: String = String::from("|") + &String::from_iter([' '; W-2].iter()) + "|";
    let hbuf: &[u8] = hst.as_bytes();

    stdout.write_all(wbuf)?;
    stdout.write_all(b"\n")?;
    for _i in 0..H {
        stdout.write_all(hbuf)?;
        stdout.write_all(b"\n")?;
    }

    stdout.write_all(wbuf)?;
    stdout.flush()
}


pub fn render_scene_to_screen_area(stdout: &mut Stdout, w: u64, h: u64, scene: &Scene, camera: &Camera) -> Result<()> {

    stdout.write_all(b"\x1B[2;2H")?;

    let width_height_aspect_ratio: f64 = (w as f64)/(h as f64);
    let terminal_newline_aspect_ratio: f64 = 1.0/2.0;
    let true_aspect_ratio: f64 = width_height_aspect_ratio * terminal_newline_aspect_ratio;

    // go through all rows (h rows)
    //      init row buffer (charactes to draw on the row)
    //      for each pixel in a row (w pixels in a row)
    //          generate a ray for the pixel
    //          trace the ray, and find the RayHit
    //          decide what to draw based on RayHit
    //          push that character to the row buffer
    //      write the row to stdout, and move to the next row

    for j in 0..h {
        let mut row_buf: Vec<u8> = Vec::new();
        for i in 0..w {
            let normalized_screen_coord: (f64, f64) = camera::map_terminal_pixel_pos_to_screen_coord((i,j), w, h);

            let ray: raytracing::Ray = camera.create_ray_for_screenpos(normalized_screen_coord, true_aspect_ratio);
            let rayhit_option: Option<raytracing::RayHit> = ray.trace(scene);

            match rayhit_option {
                Some(_hit) => row_buf.write_all(b"o")?,
                None => row_buf.write_all(b".")?,
            };
        }
        //println!("{:?}", row_buf);
        stdout.write_all(&row_buf)?;
        stdout.write_all(b"\x1B[1E\x1B[1C")?;
    }
    stdout.write_all(b"\x1B[47;0H")?;
    stdout.flush()
}