use std::fmt;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Copy, Clone, Debug, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn zero() -> Self {
        Default::default()
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn from(value: f32) -> Self {
        Vec3::new(value, value, value)
    }

    pub fn unit_from(v: Vec3) -> Self {
        let length = v.length();
        debug_assert!(length != 0.0, "Division by 0.");
        v / length
    }

    // As color
    pub fn r(&self) -> f32 {
        self.x
    }
    pub fn g(&self) -> f32 {
        self.y
    }
    pub fn b(&self) -> f32 {
        self.z
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn make_unit(&mut self) {
        let l = self.length();
        debug_assert!(l != 0.0, "Division by 0.");
        let k = 1.0 / l;
        self.x *= k;
        self.y *= k;
        self.z *= k;
    }

    pub fn dot(&self) -> f32 {
        dot(self, self)
    }

    pub fn cross(&self, other: &Self) -> Self {
        cross(self, other)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z);
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        debug_assert!(rhs.x != 0.0, "Division by 0.");
        debug_assert!(rhs.y != 0.0, "Division by 0.");
        debug_assert!(rhs.z != 0.0, "Division by 0.");
        Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        debug_assert!(rhs != 0.0, "Division by 0.");
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        debug_assert!(rhs.x != 0.0, "Division by 0.");
        debug_assert!(rhs.y != 0.0, "Division by 0.");
        debug_assert!(rhs.z != 0.0, "Division by 0.");
        *self = Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z);
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        debug_assert!(rhs != 0.0, "Division by 0.");
        let k = 1.0 / rhs;
        *self = Vec3::new(self.x * k, self.y * k, self.z * k);
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z);
    }
}
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs);
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 3, "Index out of range.");
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => unimplemented!(),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 3, "Index out of range.");
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => unimplemented!(),
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z);
    }
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3::new(
        v1.y * v2.z - v1.z * v2.y,
        v1.z * v2.x - v1.x * v2.z,
        v1.x * v2.y - v1.y * v2.x,
    )
}
