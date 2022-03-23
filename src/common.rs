
use rand::Rng;

use crate::my_vec3::MyVec3;

// Uniform in the interval [0, 1)
pub fn uniform_random() -> f64
{
    let mut rng = rand::thread_rng();

    rng.gen::<f64>()
}

pub fn random_in_interval(min: f64, max: f64) -> f64
{
    min + (max-min) * uniform_random()
}

// Uniform within a circle (excluding on the circumference)
pub fn uniform_within_unit_circle() -> MyVec3
{
    let mut rng = rand::thread_rng();

    let mut p = MyVec3 {x: rng.gen::<f64>(), y: rng.gen::<f64>(), z: 0.0};

    while p.squared_length() >= 1.0
    {
        p = MyVec3 {x: rng.gen::<f64>(), y: rng.gen::<f64>(), z: 0.0};
    }

    return p;
}