use raytracer::{Camera, Vec3, World};

use std::fs::File;
use std::io::Write;
use std::path::Path;

use rand::prelude::*;

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("out.ppm");
    let mut file = File::create(&path)?;

    let nx = 400;
    let ny = 200;
    let ns = 50;

    println!(
        "Generating {}x{} image with {} samples per pixel... ",
        nx, ny, ns
    );

    // Write PPM header
    file.write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())?;

    let world = World::random();

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let vertical_fov = 20.0;
    let aspect_ratio = nx as f32 / ny as f32;
    let aperture = 0.1;
    let distance_to_focus = 10.0;
    let time_range = (0.0, 1.0);
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        vertical_fov,
        aspect_ratio,
        aperture,
        distance_to_focus,
        time_range,
    );

    for y in (0..ny).rev() {
        for x in 0..nx {
            let mut col = Vec3::default();
            for _ in 0..ns {
                let u = (x as f32 + random::<f32>()) / nx as f32;
                let v = (y as f32 + random::<f32>()) / ny as f32;
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

    println!("Done!");

    Ok(())
}
