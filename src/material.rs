use crate::{
    color::{self, Rgb},
    hittable::HitResult,
    ray::Ray,
    vec3::{dot, reflect, refract, Vec3},
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
            fuzziness: fuzziness.clamp(f64::MIN, 1.0),
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

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<ScatterResult> {
        let attenuation = color::WHITE;
        let refraction_ratio = if hit_result.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction.to_unit();
        let cos_theta = f64::min(dot(&(-unit_direction), &hit_result.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let should_not_refract = reflectance(cos_theta, refraction_ratio) > rand::random();
        let refracted_direction = if cannot_refract || should_not_refract {
            reflect(&unit_direction, &hit_result.normal)
        } else {
            refract(&unit_direction, &hit_result.normal, refraction_ratio)
        };

        let scattered_ray = Ray::new(hit_result.point, refracted_direction);

        Some(ScatterResult::new(attenuation, scattered_ray))
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Shlick's approximation for reflectance
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
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
