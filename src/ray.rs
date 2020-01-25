use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug, Default)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray::with_time(origin, direction, 0.0)
    }

    pub fn with_time(origin: Vec3, direction: Vec3, time: f32) -> Self {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
