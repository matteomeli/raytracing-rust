use std::{
    fmt,
    ops::{Add, Mul},
};
/// A RGB color
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn black() -> Self {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Self {
        Color::new(1.0, 1.0, 1.0)
    }

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(rhs.r * self, rhs.g * self, rhs.b * self)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl fmt::Display for Color {
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
