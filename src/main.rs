fn main() {
    let height = 256;
    let width = 256;

    println!("P3\n{} {}\n255", width, height);

    for j in (0..height).rev() {
        for i in 0..width {
            let r = i as f64 / (width - 1) as f64;
            let g = j as f64 / (height - 1) as f64;
            let b = 0.25;

            let r = (r * 255.999) as i32;
            let g = (g * 255.999) as i32;
            let b = (b * 255.999) as i32;

            println!("{} {} {}", r, g, b);
        }
    }
}
