use rand::Rng;


pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

