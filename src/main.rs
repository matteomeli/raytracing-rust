use raytracer::Vec3;

fn main() {
    let nx = 200;
    let ny = 100;
    print!("P3\n{} {}\n255\n", nx, ny);
    for y in (0..ny).rev() {
        for x in 0..nx {
            let col = Vec3::new(x as f32 / nx as f32, y as f32 / ny as f32, 0.2);
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
