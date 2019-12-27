use crate::hittable::HitResult;
use crate::ray::Ray;
use crate::vec3::{self, Vec3};

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

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Metal {
            albedo,
            fuzz: if fuzz <= 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
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
