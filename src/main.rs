use core::time::Duration;
use std::{
    io::{self, Write},
    thread,
};

fn main() {
    let height = 256;
    let width = 256;

    // Print completion progress to standard error
    let mut stderr = io::stderr();

    println!("P3\n{} {}\n255", width, height);

    for j in (0..height).rev() {
        let progress = (height - j) as f32 / height as f32 * 100.0;
        // Could use eprintln!() macro here to write to sterr,
        // but carriage return '\r' doesn't seem to work with it.
        stderr
            .write(format!("\rCompletion: {:.1}%", progress).as_bytes())
            .unwrap();

        for i in 0..width {
            let r = i as f64 / (width - 1) as f64;
            let g = j as f64 / (height - 1) as f64;
            let b = 0.25;

            let r = (r * 255.999) as i32;
            let g = (g * 255.999) as i32;
            let b = (b * 255.999) as i32;

            println!("{} {} {}", r, g, b);
        }

        // Add small sleep to make completion progress actually show up ;)
        thread::sleep(Duration::from_millis(5));
    }

    eprintln!("\nDone.");
}
