use crate::hittable::{HitResult, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

use std::f32;

#[derive(Debug, Default)]
pub struct World<T>
where
    T: Hittable,
{
    hittables: Vec<T>,
}

impl<T> World<T>
where
    T: Hittable,
{
    pub fn add(&mut self, hittable: T) {
        self.hittables.push(hittable);
    }

    pub fn color(&self, ray: &Ray) -> Vec3 {
        let mut hit = HitResult::default();
        // Ignore hits very close to 0.
        if self.hit(ray, 0.001, f32::MAX, &mut hit) {
            let target = hit.point + hit.normal + Vec3::random_in_unit_sphere();
            let bounced = Ray::new(hit.point, target - hit.point);
            0.5 * self.color(&bounced)
        } else {
            let unit_direction = Vec3::unit_from(ray.direction);
            let t = 0.5 * (unit_direction.y) + 1.0;
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

impl<T> Hittable for World<T>
where
    T: Hittable,
{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, result: &mut HitResult) -> bool {
        let mut hit_result = HitResult::default();
        let mut has_hit_anything = false;
        let mut closest_hit_distance = t_max;
        for hittable in self.hittables.iter() {
            if hittable.hit(ray, t_min, closest_hit_distance, &mut hit_result) {
                has_hit_anything = true;
                closest_hit_distance = hit_result.t;
                *result = hit_result;
            }
        }
        has_hit_anything
    }
}
