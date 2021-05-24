use std::ops::{Add, AddAssign, Mul};

use crate::vec3::Vec3;
/// A RGB color
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Rgb {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Rgb {
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Rgb { r, g, b }
    }
}

pub static BLACK: Rgb = Rgb::new(0.0, 0.0, 0.0);
pub static WHITE: Rgb = Rgb::new(1.0, 1.0, 1.0);

impl Mul<Rgb> for f64 {
    type Output = Rgb;

    fn mul(self, rhs: Rgb) -> Self::Output {
        Rgb::new(rhs.r * self, rhs.g * self, rhs.b * self)
    }
}

impl Add for Rgb {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Rgb::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl AddAssign for Rgb {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl From<Vec3> for Rgb {
    fn from(v: Vec3) -> Self {
        Rgb::new(v.x, v.y, v.z)
    }
}

pub fn write_rgb(color: &Rgb, samples_per_pixel: i32) {
    let mut r = color.r;
    let mut g = color.g;
    let mut b = color.b;

    // Divide the color by the number of samples and gamma-correct for gamma=2.0 (equivalent to 1/2 exp or sqrt).
    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    println!(
        "{} {} {}",
        (256.0 * r.clamp(0.0, 0.999)) as i32,
        (256.0 * g.clamp(0.0, 0.999)) as i32,
        (256.0 * b.clamp(0.0, 0.999)) as i32
    )
}
