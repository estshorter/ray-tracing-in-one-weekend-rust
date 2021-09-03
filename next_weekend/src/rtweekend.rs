use rand::distributions::uniform::SampleRange;
use rand::Rng;
// Constants
pub const INFINITY: f64 = f64::INFINITY;

// Utility Functions
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

pub fn random_double() -> f64 {
    rand::thread_rng().gen()
}

pub fn random_double_range<R: SampleRange<f64>>(range: R) -> f64 {
    rand::thread_rng().gen_range(range)
}
