use crate::{
    hittable::{HitResult, Hittable},
    ray::Ray,
    vec3::{dot, Point3},
};

pub struct Sphere {
    pub centre: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(centre: Point3, radius: f64) -> Self {
        Sphere { centre, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        let oc = ray.origin - self.centre;
        let a = ray.direction.length_squared();
        let half_b = dot(&oc, &ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // Find the closest root in the acceptable range
        let discriminant_sqrt = discriminant.sqrt();
        let mut root = (-half_b - discriminant_sqrt) / a;
        if root < t_min || root > t_max {
            root = (-half_b + discriminant_sqrt) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - self.centre) / self.radius;
        let front_face = dot(&ray.direction, &outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Some(HitResult::new(point, normal, root, front_face))
    }
}
