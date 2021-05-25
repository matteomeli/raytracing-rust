use std::{
    f64,
    io::{self, Write},
    rc::Rc,
};

use raymond::{
    camera::Camera,
    color::{self, write_rgb, Rgb},
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
    vec3::Point3,
    world::World,
};

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i64;
    let samples_per_pixel = 100;
    let max_bounces = 50;

    // World
    let mut world = World::default();

    let material_ground = Rc::new(Lambertian::new(Rgb::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Rgb::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Rgb::new(0.8, 0.6, 0.2), 0.0));

    world.add_hittable(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add_hittable(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    ));
    world.add_hittable(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::clone(&material_left) as _,
    ));
    world.add_hittable(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.4,
        Rc::clone(&material_left) as _,
    ));
    world.add_hittable(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));

    // Camera
    let camera = Camera::default();

    // Rendering

    // Print completion progress to standard error
    let mut stderr = io::stderr();

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        let progress = (image_height - j) as f32 / image_height as f32 * 100.0;
        // Could use eprintln!() macro here to write to stderr,
        // but carriage return '\r' doesn't seem to work with it.
        stderr
            .write(format!("\rCompletion: {:.1}%", progress).as_bytes())
            .unwrap();

        for i in 0..image_width {
            let mut pixel_color = color::BLACK;
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rand::random::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (image_height - 1) as f64;
                let ray = camera.ray(u, v);
                pixel_color += world.trace(ray, max_bounces);
            }
            write_rgb(&pixel_color, samples_per_pixel);
        }
    }

    eprintln!("\nDone.");
}
