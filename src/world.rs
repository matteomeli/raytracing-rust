use crate::{
    color::{self, Rgb},
    hittable::{HitResult, Hittable},
    ray::Ray,
};

#[derive(Default)]
pub struct World {
    pub hittables: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn add_hittable<H: Hittable>(&mut self, hittable: H) {
        self.hittables.push(Box::new(hittable))
    }

    pub fn trace(&self, ray: Ray, bounces_left: u32) -> Rgb {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if bounces_left == 0 {
            return color::BLACK;
        }

        if let Some(hit_result) = self.hit(&ray, 0.001, f64::MAX) {
            if let Some(scatter_result) = hit_result.material.scatter(&ray, &hit_result) {
                return scatter_result.attenuation
                    * self.trace(scatter_result.scattered_ray, bounces_left - 1);
            }

            return color::BLACK;
        }

        let unit_direction = ray.direction.to_unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * color::WHITE + t * Rgb::new(0.5, 0.7, 1.0)
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        let mut hit_result: Option<HitResult> = None;
        let mut closest_so_far = t_max;

        for hittable in self.hittables.iter() {
            if let Some(hit) = hittable.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_result.replace(hit);
            }
        }

        hit_result
    }
}
