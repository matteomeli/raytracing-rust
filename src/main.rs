use core::time::Duration;
use std::{
    f64,
    io::{self, Write},
    thread,
};

use raymond::{
    ray::Ray,
    sphere::Sphere,
    vec3::{Point3, Vec3, ZERO},
    world::World,
};

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i64;

    // World
    let mut world = World::default();
    world.add_hittable(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add_hittable(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;
    let origin = ZERO;
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Rendering

    // Print completion progress to standard error
    let mut stderr = io::stderr();
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        let progress = (image_height - j) as f32 / image_height as f32 * 100.0;
        // Could use eprintln!() macro here to write to sterr,
        // but carriage return '\r' doesn't seem to work with it.
        stderr
            .write(format!("\rCompletion: {:.1}%", progress).as_bytes())
            .unwrap();

        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let color = world.trace(&ray);
            println!("{}", color);
        }

        // Add small sleep to make completion progress actually show up ;)
        thread::sleep(Duration::from_millis(5));
    }

    eprintln!("\nDone.");
}
