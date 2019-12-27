use crate::hittable::{HitResult, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

use std::f32;

#[derive(Default)]
pub struct World {
    hittables: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn new(hittables: Vec<Box<dyn Hittable>>) -> Self {
        World { hittables }
    }

    pub fn new_random() -> Self {
        // TODO: Generate random world.
        unimplemented!()
    }

    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.hittables.push(hittable);
    }

    pub fn color(&self, ray: &Ray, depth: i32) -> Vec3 {
        // Ignore hits very close to 0.
        if let Some(hit) = self.hit(ray, 0.001, f32::MAX) {
            let mut attenuation = Vec3::default();
            let mut scattered = Ray::default();
            // Limit to 50 rebounces.
            if depth < 50
                && hit
                    .material
                    .scatter(ray, &hit, &mut attenuation, &mut scattered)
            {
                attenuation * self.color(&scattered, depth + 1)
            } else {
                Vec3::default()
            }
        } else {
            let unit_direction = Vec3::unit_from(ray.direction);
            let t = 0.5 * (unit_direction.y) + 1.0;
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

impl Hittable for World {
    // Computes closest hit
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        let mut result = None;
        let mut closest_hit_distance = t_max;
        for hittable in self.hittables.iter() {
            if let Some(hit) = hittable.hit(ray, t_min, closest_hit_distance) {
                closest_hit_distance = hit.t;
                result.replace(hit);
            }
        }
        result
    }
}
