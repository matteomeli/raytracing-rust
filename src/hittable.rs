use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

use std::rc::Rc;

pub struct HitResult {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
}

impl HitResult {
    pub fn new(t: f32, point: Vec3, normal: Vec3, material: Rc<dyn Material>) -> Self {
        HitResult {
            t,
            point,
            normal,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult>;
}
