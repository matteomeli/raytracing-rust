use crate::{
    color::Rgb,
    hittable::HitResult,
    ray::Ray,
    vec3::{dot, reflect, Vec3},
};

pub struct Lambertian {
    pub albedo: Rgb,
}

impl Lambertian {
    pub fn new(albedo: Rgb) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_result: &HitResult) -> Option<ScatterResult> {
        let mut scatter_direction = hit_result.normal + Vec3::random_unit_vector();
        // Catch degenerate scatter direction
        if scatter_direction.is_near_zero() {
            scatter_direction = hit_result.normal;
        }
        let scattered = Ray::new(hit_result.point, scatter_direction);
        Some(ScatterResult::new(self.albedo, scattered))
    }
}

pub struct Metal {
    pub albedo: Rgb,
    pub fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Rgb, fuzziness: f64) -> Self {
        Metal {
            albedo,
            fuzziness: fuzziness.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<ScatterResult> {
        let reflected_direction = reflect(&ray.direction.to_unit(), &hit_result.normal);
        let scattered_ray = Ray::new(
            hit_result.point,
            reflected_direction + self.fuzziness * Vec3::random_in_unit_sphere(),
        );
        if dot(&scattered_ray.direction, &hit_result.normal) > 0.0 {
            Some(ScatterResult::new(self.albedo, scattered_ray))
        } else {
            None
        }
    }
}

#[derive(Debug, Default)]
pub struct ScatterResult {
    pub attenuation: Rgb,
    pub scattered_ray: Ray,
}

impl ScatterResult {
    pub fn new(attenuation: Rgb, scattered_ray: Ray) -> Self {
        ScatterResult {
            attenuation,
            scattered_ray,
        }
    }
}

pub trait Material: 'static {
    fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<ScatterResult>;
}
