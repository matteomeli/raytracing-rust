pub use color::Rgb;
pub use hittable::{HitResult, Hittable};
pub use ray::Ray;
pub use sphere::Sphere;
pub use vec3::{cross, dot, Point3, Vec3};

mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;
