fn main() {
    let nx = 200;
    let ny = 100;
    print!("P3\n{} {}\n255\n", nx, ny);
    for y in (0..ny).rev() {
        for x in 0..nx {
            let r = x as f32 / nx as f32;
            let g = y as f32 / ny as f32;
            let b = 0.2;
            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
