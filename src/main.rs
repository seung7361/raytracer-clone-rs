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

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (259.999 * b) as i32;

            print!("{} {} {}\n", ir, ig, ib);
        }
    }

    eprint!("\nDone. \n");
}