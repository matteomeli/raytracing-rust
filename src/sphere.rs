use crate::hittable::{HitResult, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Vec3};

use std::rc::Rc;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        let oc = ray.origin - self.center;
        let a = dot(&ray.direction, &ray.direction);
        let half_b = dot(&ray.direction, &oc);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut t = (-half_b - root) / a;
            if t > t_min && t < t_max {
                let point = ray.at(t);
                let normal = (point - self.center) / self.radius;
                Some(HitResult::new(t, point, normal, Rc::clone(&self.material)))
            } else {
                t = (-half_b + root) / a;
                if t > t_min && t < t_max {
                    let point = ray.at(t);
                    let normal = (point - self.center) / self.radius;
                    Some(HitResult::new(t, point, normal, Rc::clone(&self.material)))
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}
