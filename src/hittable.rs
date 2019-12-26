use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug, Default)]
pub struct HitResult {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, result: &mut HitResult) -> bool;
}

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
