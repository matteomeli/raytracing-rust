use crate::ray::Ray;
use crate::vec3::{self, Vec3};

use std::f32;

#[derive(Debug)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        up: Vec3,
        vertical_fov: f32, // degrees
        aspect_ratio: f32,
    ) -> Self {
        let theta_radians = vertical_fov * f32::consts::PI / 180.0;
        let half_height = (theta_radians / 2.0).tan();
        let half_width = aspect_ratio * half_height;
        let origin = look_from;
        let w = Vec3::unit_from(look_from - look_at);
        let u = Vec3::unit_from(vec3::cross(&up, &w));
        let v = vec3::cross(&w, &u);
        let lower_left_corner = origin - half_width * u - half_height * v - w;
        let horizontal = 2.0 * half_width * u;
        let vertical = 2.0 * half_height * v;
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn ray_at(&self, s: f32, t: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            origin: Vec3::default(),
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
        }
    }
}
