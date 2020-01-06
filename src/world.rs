use crate::hittable::{HitResult, Hittable};
use crate::material::{Dielectric, Lambertian, Metallic};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

use rand::prelude::*;

use std::f32;

#[derive(Default)]
pub struct World {
    hittables: Vec<Box<dyn Hittable + Send + Sync>>,
}

impl World {
    pub fn new(hittables: Vec<Box<dyn Hittable + Send + Sync>>) -> Self {
        World { hittables }
    }

    pub fn random() -> Self {
        let n = 500;
        let mut hittables: Vec<Box<dyn Hittable + Send + Sync>> = Vec::with_capacity(n);

        hittables.push(Box::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian::new(Vec3::new(0.5, 0.5, 0.5)),
        )));

        for a in -11..11 {
            for b in -11..11 {
                let random_material_chooser = random::<f32>();
                let center = Vec3::new(
                    a as f32 + 0.9 * random::<f32>(),
                    0.2,
                    b as f32 + 0.9 * random::<f32>(),
                );
                if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    if random_material_chooser < 0.8 {
                        hittables.push(Box::new(Sphere::new(
                            center,
                            0.2,
                            Lambertian::new(Vec3::new(
                                random::<f32>() * random::<f32>(),
                                random::<f32>() * random::<f32>(),
                                random::<f32>() * random::<f32>(),
                            )),
                        )))
                    } else if random_material_chooser < 0.95 {
                        hittables.push(Box::new(Sphere::new(
                            center,
                            0.2,
                            Metallic::new(
                                Vec3::new(
                                    0.5 * (1.0 + random::<f32>()),
                                    0.5 * (1.0 + random::<f32>()),
                                    0.5 * (1.0 + random::<f32>()),
                                ),
                                0.5 * random::<f32>(),
                            ),
                        )))
                    } else {
                        hittables.push(Box::new(Sphere::new(center, 0.2, Dielectric::new(1.5))));
                    }
                }
            }
        }

        hittables.push(Box::new(Sphere::new(
            Vec3::new(0.0, 1.0, 0.0),
            1.0,
            Dielectric::new(1.5),
        )));
        hittables.push(Box::new(Sphere::new(
            Vec3::new(-4.0, 1.0, 0.0),
            1.0,
            Lambertian::new(Vec3::new(0.4, 0.2, 0.1)),
        )));
        hittables.push(Box::new(Sphere::new(
            Vec3::new(4.0, 1.0, 0.0),
            1.0,
            Metallic::new(Vec3::new(0.7, 0.6, 0.5), 0.0),
        )));

        World::new(hittables)
    }

    pub fn add(&mut self, hittable: Box<dyn Hittable + Send + Sync>) {
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
