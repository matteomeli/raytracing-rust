use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug, Default)]
pub struct HitResult {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, result: &mut HitResult) -> bool;
}
