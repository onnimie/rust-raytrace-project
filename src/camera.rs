use std::os::windows::io::HandleOrInvalid;

use crate::math::matrix::Matrix4x4;
use crate::math::vector::Vector3;
use crate::raytracing::Ray;

pub struct Camera {
    pub eyepoint: Vector3<f64>,
    pub dir: Vector3<f64>,
    horizontal: Vector3<f64>,
    up: Vector3<f64>, // (vertical)
    tmin: f64,
    fov_vertical: f64,
}

// Assume camera screen has coordinates in [-1,1]^2,
pub fn map_terminal_pixel_pos_to_screen_coord(pixelpos: (u64,u64), w: u64, h: u64) -> (f64,f64) {
    return (0.0,0.0);
}

impl Camera {
    pub fn new(eyepoint: Vector3<f64>,
        dir: Vector3<f64>,
        horizontal: Vector3<f64>,
        up: Vector3<f64>, tmin: f64,
        fov_vertical: f64) -> Self {
            Self {
                eyepoint,
                dir,
                horizontal,
                up,
                tmin,
                fov_vertical,
            }
    }

    pub fn rotate(&mut self, _rotation: Matrix4x4) {
        // apply rotation matrix to dir, horizontal and up
    }

    pub fn move_to_pos(&mut self, new_pos: Vector3<f64>) {
        self.eyepoint = new_pos;
    }

    pub fn move_by(&mut self, deltas: &Vector3<f64>) {
        self.eyepoint.add(&deltas);
    }

    pub fn create_ray_for_screenpos(&self, screen_pos: (f64,f64), aspect_ratio: f64) -> Ray {

        let distance_to_screen_origo: f64 = 1.0/((self.fov_vertical/2.0).tan());
        let screen_origo: Vector3<f64> = self.eyepoint.added(&self.dir.scaled(distance_to_screen_origo));

        let screen_offset: Vector3<f64> = self.horizontal.scaled(screen_pos.0 * aspect_ratio).added(&self.up.scaled(screen_pos.1));
        let ray_end_point_on_screen: Vector3<f64> = screen_origo.added(&screen_offset);

        let raydir: Vector3<f64> = ray_end_point_on_screen.subtracted(&self.eyepoint).normalized();

        let ray: Ray = Ray::new(self.eyepoint, raydir, self.tmin);
        ray
    }
}