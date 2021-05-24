use crate::{
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct HitResult<'a> {
    pub point: Point3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub t: f64,
    pub front_face: bool,
}

impl<'a> HitResult<'a> {
    pub fn new(
        point: Point3,
        normal: Vec3,
        material: &'a dyn Material,
        t: f64,
        front_face: bool,
    ) -> Self {
        HitResult {
            point,
            normal,
            material,
            t,
            front_face,
        }
    }
}

pub trait Hittable: 'static {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitResult>;
}
