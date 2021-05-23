use core::time::Duration;
use std::{
    io::{self, Write},
    thread,
};

use raymond::{dot, Point3, Ray, Rgb, Vec3};

fn hit_sphere(centre: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - centre;
    let a = ray.direction.length_squared();
    let half_b = dot(&oc, &ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        // Closes hit point is smallest t
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(ray: &Ray) -> Rgb {
    let sphere_centre = Point3::new(0.0, 0.0, -1.0);
    let sphere_radius = 0.5;

    // Test sphere hit
    let t = hit_sphere(sphere_centre, sphere_radius, ray);
    if t > 0.0 {
        // Use normal to shade the surface of the sphere
        let n = (ray.at(t) - sphere_centre).to_unit();
        // Map normal components from (-1, 1) to (0, 1) to obtain a RGB color
        return 0.5 * Rgb::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }

    let unit_direction = ray.direction.to_unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Rgb::white() + t * Rgb::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i64;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;
    let origin = Point3::zero();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

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
            let color = ray_color(&ray);
            println!("{}", color);
        }

        // Add small sleep to make completion progress actually show up ;)
        thread::sleep(Duration::from_millis(5));
    }

    eprintln!("\nDone.");
}
