use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug, Default)]
pub struct HitResult {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
}

impl HitResult {
    pub fn new(point: Point3, normal: Vec3, t: f64) -> Self {
        HitResult { point, normal, t }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitResult>;
}
