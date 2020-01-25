use crate::ray::Ray;
use crate::vec3::{self, Vec3};

use rand::prelude::*;
use std::f32;

#[derive(Debug)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32,
    time_range: (f32, f32),
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        up: Vec3,
        vertical_fov: f32, // degrees
        aspect_ratio: f32,
        aperture: f32,
        focus_distance: f32,
        time_range: (f32, f32),
    ) -> Self {
        let lens_radius = aperture / 2.0;
        let theta_radians = vertical_fov * f32::consts::PI / 180.0;
        let half_height = (theta_radians / 2.0).tan();
        let half_width = aspect_ratio * half_height;
        let origin = look_from;
        let w = Vec3::unit_from(look_from - look_at);
        let u = Vec3::unit_from(vec3::cross(&up, &w));
        let v = vec3::cross(&w, &u);
        let lower_left_corner = origin
            - half_width * focus_distance * u
            - half_height * focus_distance * v
            - focus_distance * w;
        let horizontal = 2.0 * half_width * focus_distance * u;
        let vertical = 2.0 * half_height * focus_distance * v;
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
            time_range,
        }
    }

    pub fn ray_at(&self, s: f32, t: f32) -> Ray {
        let random_in_lens_disk = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * random_in_lens_disk.x + self.v * random_in_lens_disk.y;
        let time: f32 =
            self.time_range.0 + random::<f32>() * (self.time_range.1 - self.time_range.0);
        Ray::with_time(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            time,
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
            u: Vec3::new(1.0, 0.0, 0.0),
            v: Vec3::new(0.0, 1.0, 0.0),
            lens_radius: 0.0,
            time_range: (0.0, 0.0),
        }
    }
}
