use crate::{
    color::{self, Rgb, WHITE},
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

    pub fn trace(&self, ray: &Ray) -> Rgb {
        if let Some(hit_result) = self.hit(ray, 0.0, f64::MAX) {
            // Use normal to shade the surface of the sphere
            // Map normal components from (-1, 1) to (0, 1) to obtain a RGB color
            return 0.5 * (Rgb::from(hit_result.normal) + WHITE);
        }

        let unit_direction = ray.direction.to_unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * color::WHITE + t * Rgb::new(0.5, 0.7, 1.0)
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitResult> {
        let mut hit_result = HitResult::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for hittable in self.hittables.iter() {
            if let Some(hit) = hittable.hit(ray, t_min, closest_so_far) {
                hit_anything = true;
                closest_so_far = hit.t;
                hit_result = hit;
            }
        }

        if hit_anything == false {
            None
        } else {
            Some(hit_result)
        }
    }
}
