use crate::hittable::HitResult;
use crate::ray::Ray;
use crate::vec3::{self, Vec3};

use rand::prelude::*;

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &HitResult,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray: &Ray,
        hit: &HitResult,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let target = hit.point + hit.normal + Vec3::random_in_unit_sphere();
        *scattered = Ray::new(hit.point, target - hit.point);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metallic {
    albedo: Vec3,
    fuzz: f32,
}

impl Metallic {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Metallic {
            albedo,
            fuzz: if fuzz <= 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metallic {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &HitResult,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = vec3::reflect(Vec3::unit_from(ray.direction), hit.normal);
        *scattered = Ray::new(
            hit.point,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );
        *attenuation = self.albedo;
        vec3::dot(&scattered.direction, &hit.normal) > 0.0
    }
}

pub struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &HitResult,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = vec3::reflect(ray.direction, hit.normal);
        *attenuation = Vec3::from(1.0); // Glassy surfaces absorb nothing.

        let (outward_normal, ni, nt, cosine) = if vec3::dot(&ray.direction, &hit.normal) > 0.0 {
            (
                -hit.normal,
                self.refraction_index,
                1.0,
                self.refraction_index * vec3::dot(&ray.direction, &hit.normal)
                    / ray.direction.length(),
            )
        } else {
            (
                hit.normal,
                1.0,
                self.refraction_index,
                -vec3::dot(&ray.direction, &hit.normal) / ray.direction.length(),
            )
        };

        let (reflection_probe, refracted) =
            if let Some(refracted) = vec3::refract(ray.direction, outward_normal, ni, nt) {
                (schlick(cosine, self.refraction_index), Some(refracted))
            } else {
                (1.0, None)
            };

        if random::<f32>() < reflection_probe {
            *scattered = Ray::new(hit.point, reflected);
        } else if let Some(r) = refracted {
            *scattered = Ray::new(hit.point, r);
        }

        true
    }
}

fn schlick(cosine: f32, refraction_index: f32) -> f32 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
