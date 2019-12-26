use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug, Default)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn zero() -> Self {
        Default::default()
    }

    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn evaluate(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
