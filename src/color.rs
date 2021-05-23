use std::{
    fmt,
    ops::{Add, Mul},
};
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

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            (255.999 * self.r) as i32,
            (255.999 * self.g) as i32,
            (255.999 * self.b) as i32
        )
    }
}
