mod color;
mod vec3;

use std::io;
use color::Color;

fn main() {
    const IMAGE_WIDTH: i32 = 1280;
    const IMAGE_HEIGHT: i32 = 720;

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in 0..IMAGE_HEIGHT {
        eprint!("\rScanlines in progress: {} / {} ", j + 1, IMAGE_HEIGHT);

        for i in 0..IMAGE_WIDTH { 
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = (IMAGE_HEIGHT - j + 1) as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;

            let color = Color::new(r, g, b);
            color::write_color(&mut io::stdout(), color);
        }
    }

    eprint!("\nDone. \n");
}