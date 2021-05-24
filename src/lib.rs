pub mod camera;
pub mod color;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec3;
pub mod world;

pub fn rand_in_range(min: f64, max: f64) -> f64 {
    min + (max - min) * rand::random::<f64>()
}
