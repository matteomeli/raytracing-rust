use raytracer::{dot, Ray, Vec3};

use std::fs::File;
use std::io::Write;
use std::path::Path;

fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin - center;
    let a = dot(&ray.direction, &ray.direction);
    let b = 2.0 * dot(&ray.direction, &oc);
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / 2.0 * a
    }
}

fn color(ray: &Ray) -> Vec3 {
    let sphere_center = Vec3::new(0.0, 0.0, -1.0);
    let t = hit_sphere(sphere_center, 0.5, &ray);
    if t > 0.0 {
        let normal = Vec3::unit_from(ray.evaluate(t) - sphere_center);
        Vec3::from(0.5) + normal * 0.5
    } else {
        let unit_direction = Vec3::unit_from(ray.direction);
        let t = 0.5 * (unit_direction.y) + 1.0;
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("out/scene.ppm");
    let mut file = File::create(&path)?;

    let nx = 1024;
    let ny = 512;

    // Write PPM header
    file.write(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())?;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::zero();
    for y in (0..ny).rev() {
        for x in 0..nx {
            let u = x as f32 / nx as f32;
            let v = y as f32 / ny as f32;

            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(&ray);

            let ir = (255.9 * col[0]) as i32;
            let ig = (255.9 * col[1]) as i32;
            let ib = (255.9 * col[2]) as i32;

            // Write pixel color
            file.write(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
        }
    }

    Ok(())
}
