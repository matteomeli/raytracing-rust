use std::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// A generic vector with 3 elements
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn zero() -> Self {
        Default::default()
    }

    pub fn one() -> Self {
        Vec3::from(1.0)
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn from(e: f32) -> Self {
        Vec3::new(e, e, e)
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, rhs: Self) -> Self {
        *self * rhs
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn to_unit(&self) -> Self {
        let length = self.length();
        debug_assert!(length != 0.0, "Division by 0.");
        let inv = 1.0 / length;
        Vec3 {
            x: self.x * inv,
            y: self.y * inv,
            z: self.z * inv,
        }
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

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        debug_assert!(rhs != 0.0, "Division by 0.");
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        debug_assert!(rhs != 0.0, "Division by 0.");
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod test {
    use super::Vec3;

    #[test]
    fn neg() {
        let zero = Vec3::zero();
        let v1 = -zero;
        assert_eq!(v1, zero);

        let v1 = Vec3::new(1.0, -1.0, 0.0);
        let v2 = -v1;
        assert_eq!(v1, -v2);
    }

    #[test]
    fn add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let zero = Vec3::zero();
        assert_eq!(v1, v1 + zero);
        assert_eq!(v1, zero + v1);

        let v2 = Vec3::new(3.0, 2.0, 1.0);
        let result = Vec3::new(4.0, 4.0, 4.0);
        assert_eq!(v1 + v2, result);
        assert_eq!(v2 + v1, result);
    }

    #[test]
    fn add_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let zero = Vec3::zero();
        v1 += zero;
        assert_eq!(v1, v1);

        let mut v1 = Vec3::new(3.0, 2.0, 1.0);
        let result = Vec3::new(6.0, 4.0, 2.0);
        v1 += v1;
        assert_eq!(v1, result);
    }

    #[test]
    fn sub() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let zero = Vec3::zero();
        assert_eq!(v1, v1 - zero);
        assert_eq!(-v1, zero - v1);

        let v2 = Vec3::new(3.0, 2.0, 1.0);
        assert_eq!(v1 - v2, Vec3::new(-2.0, 0.0, 2.0));
        assert_eq!(v2 - v1, Vec3::new(2.0, 0.0, -2.0));
    }

    #[test]
    fn sub_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let zero = Vec3::zero();
        v1 -= zero;
        assert_eq!(v1, v1);

        let mut v1 = Vec3::new(3.0, 2.0, 1.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        v1 -= v2;
        assert_eq!(v1, Vec3::new(2.0, 0.0, -2.0));
    }

    #[test]
    fn mul_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let zero = Vec3::zero();
        assert_eq!(zero * v1, zero);

        let one = Vec3::one();
        assert_eq!(v1 * one, v1);
    }

    #[test]
    fn mul_assign_vec3() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let zero = Vec3::zero();
        v1 *= zero;
        assert_eq!(v1, zero);

        let mut v1 = Vec3::new(3.0, 2.0, 1.0);
        let one = Vec3::one();
        v1 += one;
        assert_eq!(v1, v1);

        let mut v1 = Vec3::new(0.0, 1.0, 2.0);
        let v2 = Vec3::new(3.0, 4.0, 5.0);
        let result = Vec3::new(0.0, 4.0, 10.0);
        v1 *= v2;
        assert_eq!(v1, result);
    }

    #[test]
    fn mul() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = v1 * 2.0;
        assert_eq!(v2, Vec3::new(2.0, 4.0, 6.0));

        let zero = Vec3::zero();
        assert_eq!(v1 * 0.0, zero);

        assert_eq!(v1 * 1.0, v1);
    }

    #[test]
    fn mul_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        v1 *= 2.0;
        assert_eq!(v1, Vec3::new(2.0, 4.0, 6.0));

        v1 *= 1.0;
        assert_eq!(v1, v1);

        let zero = Vec3::zero();
        v1 *= 0.0;
        assert_eq!(v1, zero);
    }

    #[test]
    fn div() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = v1 / 2.0;
        assert_eq!(v2, Vec3::new(0.5, 0.0, 0.0));
    }

    #[test]
    #[should_panic]
    fn div_by_zero_invalid() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let _ = v1 / 0.0;
    }

    #[test]
    fn div_assign() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        v1 /= 2.0;
        assert_eq!(v1, Vec3::new(0.5, 1.0, 1.5));

        v1 /= 1.0;
        assert_eq!(v1, v1);
    }

    #[test]
    #[should_panic]
    fn div_assign_by_zero_invalid() {
        let mut v1 = Vec3::new(1.0, 0.0, 0.0);
        v1 /= 0.0;
    }
}
