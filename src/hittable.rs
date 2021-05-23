use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug, Default)]
pub struct HitResult {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitResult {
    pub fn new(point: Point3, normal: Vec3, t: f64, front_face: bool) -> Self {
        HitResult {
            point,
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable: 'static {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitResult>;
}
