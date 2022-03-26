
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

    loop 
    {
        let p = MyVec3 {x: 2.0 * rng.gen::<f64>() - 1.0, y: 2.0 * rng.gen::<f64>() - 1.0, z: 0.0};

        if p.squared_length() < 1.0
        {
            break p;
        }
    }
}

pub fn random_point_in_unit_sphere() -> MyVec3
{
    let mut rng = rand::thread_rng();

    loop 
    {
        // Random point in the cube [-1,-1,-1] -> [+1, +1, +1]
        let p = MyVec3{x: 2.0 * rng.gen::<f64>() - 1.0, y: 2.0 * rng.gen::<f64>() - 1.0, z: 2.0 * rng.gen::<f64>() - 1.0};

        // Continue choosing random points in the square until the chosen point also lies in a sphere of radius 1 centred at [0, 0, 0]
        if p.squared_length() < 1.0
        {
            break p;
        }
    }
}