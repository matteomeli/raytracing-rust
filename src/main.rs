use raytracer::{Camera, Dielectric, Lambertian, Metal, Sphere, Vec3, World};

use std::fs::File;
use std::io::Write;
use std::path::Path;

use rand::prelude::*;

use std::rc::Rc;

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("out/scene.ppm");
    let mut file = File::create(&path)?;

    let nx = 1024;
    let ny = 512;
    let ns = 100;

    // Write PPM header
    file.write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())?;

    let mut world = World::default();
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Rc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0)),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::new(Dielectric::new(1.5)),
    )));

    let camera = Camera::new(
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        45.0,
        nx as f32 / ny as f32,
    );
    //println!("{:?}", camera);

    let mut rng = thread_rng();

    for y in (0..ny).rev() {
        for x in 0..nx {
            let mut col = Vec3::default();
            for _ in 0..ns {
                let u = (x as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (y as f32 + rng.gen::<f32>()) / ny as f32;
                let ray = camera.ray_at(u, v);
                col += world.color(&ray, 0);
            }
            col /= ns as f32;

            // Gamma correction (gamma 2)
            col = Vec3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());

            let ir = (255.9 * col[0]) as i32;
            let ig = (255.9 * col[1]) as i32;
            let ib = (255.9 * col[2]) as i32;

            // Write pixel color
            file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
        }
    }

    Ok(())
}
