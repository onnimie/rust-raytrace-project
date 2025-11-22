use std::io::{Result, Write, Stdout};
//use std::thread;
//use std::time::Duration;

use crate::camera::Camera;
use crate::math::vector::Vector3;
use crate::raytracing;
use crate::scene::Scene;


pub struct TerminalScreen {
    w: u64,
    h: u64,
    line_to_monospace_aspect_ratio: f64,
    output: Stdout,
}

impl TerminalScreen {
    pub fn new(output: Stdout, w: u64, h: u64, line_to_monospace_aspect_ratio: f64) -> Self {
        Self {
            w,
            h,
            line_to_monospace_aspect_ratio,
            output,
        }
    }

    pub fn init_screen_area(&mut self) -> Result<()> {
        self.output.write_all(b"\x1B[2J\x1B[H")?;

        //let wst: String = String::from_iter(['='; W].iter());
        let wst: String = "=".repeat(self.w as usize);
        let wbuf: &[u8] = wst.as_bytes();

        //let hst: String = String::from("|") + &String::from_iter([' '; W-2].iter()) + "|";
        let hst: String = String::from("|") + &" ".repeat((self.w-2) as usize) + "|";
        let hbuf: &[u8] = hst.as_bytes();

        self.output.write_all(wbuf)?;
        self.output.write_all(b"\n")?;
        for _i in 0..self.h {
            self.output.write_all(hbuf)?;
            self.output.write_all(b"\n")?;
        }

        self.output.write_all(wbuf)?;
        self.output.flush()
    }

    pub fn render_scene_to_screen_area(
        &mut self,
        scene: &Scene,
        camera: &Camera) -> Result<()> {

            self.output.write_all(b"\x1B[2;2H")?;

            let width_height_aspect_ratio: f64 = (self.w as f64)/(self.h as f64);
            let true_aspect_ratio: f64 = width_height_aspect_ratio * self.line_to_monospace_aspect_ratio;

            // go through all rows (h rows)
            //      init row buffer (charactes to draw on the row)
            //      for each pixel in a row (w pixels in a row)
            //          generate a ray for the pixel
            //          trace the ray, and find the RayHit
            //          decide what to draw based on RayHit
            //          push that character to the row buffer
            //      write the row to stdout, and move to the next row

            for j in 0..self.h {
                let mut row_buf: Vec<u8> = Vec::new();
                for i in 0..(self.w-2) {
                    let normalized_screen_coord: (f64, f64) = map_terminal_pos_to_normalized_screen_coord((i,j), self.w, self.h);

                    let ray: raytracing::Ray = camera.create_ray_for_screenpos(normalized_screen_coord, true_aspect_ratio);
                    let color_hit: Option<Vector3<f64>> = ray.trace(scene);

                    match color_hit {
                        Some(color) => {
                            let buf = map_color_to_terminal_character(color);
                            row_buf.write_all(&buf)?
                        },
                        None => row_buf.write_all(b".")?,
                    };
                }
                self.output.write_all(&row_buf)?;
                self.output.write_all(b"\x1B[1E\x1B[1C")?;
            }
            self.output.write_all(format!("\x1B[{};0H", self.h+2).as_bytes())?;
            self.output.flush()
    }

}

// ([0..w],[0..h]) to (-1,1)^2
fn map_terminal_pos_to_normalized_screen_coord(pixelpos: (u64,u64), w: u64, h: u64) -> (f64,f64) {
    let x: f64 = ((pixelpos.0 as f64) / (w as f64) * 2_f64) - 1_f64;
    let y: f64 = (((h as f64)-(pixelpos.1 as f64)) / (h as f64) * 2_f64) - 1_f64;
    (x,y)
}

const CHARS: &str = ".-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@@@";
fn map_color_to_terminal_character(color: Vector3<f64>) -> [u8; 1] {
    //.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@
    // 90 chars
    let n: usize = CHARS.len();

    let gray: f64 = 0.2126*color.x + 0.7152*color.y + 0.00722*color.z;

    if gray > 1.0 {
        dbg!(gray);
    }

    let i: usize = ((gray * (n as f64)).floor()) as usize;

    [CHARS.as_bytes()[i]]
}