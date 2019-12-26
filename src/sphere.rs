use crate::hittable::{HitResult, Hittable};
use crate::ray::Ray;
use crate::vec3::{dot, Vec3};

#[derive(Copy, Clone, Debug, Default)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, result: &mut HitResult) -> bool {
        let oc = ray.origin - self.center;
        let a = dot(&ray.direction, &ray.direction);
        let half_b = dot(&ray.direction, &oc);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut t = (-half_b - root) / a;
            if t > t_min && t < t_max {
                result.t = t;
                result.point = ray.at(result.t);
                result.normal = (result.point - self.center) / self.radius;
                true
            } else {
                t = (-half_b + root) / a;
                if t > t_min && t < t_max {
                    result.t = t;
                    result.point = ray.at(result.t);
                    result.normal = (result.point - self.center) / self.radius;
                    true
                } else {
                    false
                }
            }
        } else {
            false
        }
    }
}
