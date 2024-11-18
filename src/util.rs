use rand::Rng;

pub fn random_f64() -> f64 {
    rand::thread_rng().gen()
}

pub fn random_f64_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}