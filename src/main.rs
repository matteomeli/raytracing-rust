use raytracer::{HitResult, Hittable, Ray, Sphere, Vec3, World};

use std::f32;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn color<T: Hittable>(ray: &Ray, hittable: &T) -> Vec3 {
    let mut hit_result = HitResult::default();
    if hittable.hit(ray, 0.0, f32::MAX, &mut hit_result) {
        Vec3::from(0.5) + hit_result.normal * 0.5
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
    let origin = Vec3::default();

    let mut world = World::default();
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    for y in (0..ny).rev() {
        for x in 0..nx {
            let u = x as f32 / nx as f32;
            let v = y as f32 / ny as f32;

            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(&ray, &world);

            let ir = (255.9 * col[0]) as i32;
            let ig = (255.9 * col[1]) as i32;
            let ib = (255.9 * col[2]) as i32;

            // Write pixel color
            file.write(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
        }
    }

    Ok(())
}
