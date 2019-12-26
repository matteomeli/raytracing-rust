use raytracer::{Camera, Sphere, Vec3, World};

use std::fs::File;
use std::io::Write;
use std::path::Path;

use rand::prelude::*;

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("out/scene.ppm");
    let mut file = File::create(&path)?;

    let nx = 200;
    let ny = 100;
    let ns = 100;

    // Write PPM header
    file.write(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())?;

    let mut world = World::default();
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    let camera = Camera::default();

    let mut rng = thread_rng();

    for y in (0..ny).rev() {
        for x in 0..nx {
            let mut col = Vec3::default();
            for _ in 0..ns {
                let u = (x as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (y as f32 + rng.gen::<f32>()) / ny as f32;
                let ray = camera.ray_at(u, v);
                col += world.color(&ray);
            }
            col /= ns as f32;

            let ir = (255.9 * col[0]) as i32;
            let ig = (255.9 * col[1]) as i32;
            let ib = (255.9 * col[2]) as i32;

            // Write pixel color
            file.write(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
        }
    }

    Ok(())
}
