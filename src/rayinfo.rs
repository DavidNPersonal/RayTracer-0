use crate::my_vec3::MyVec3;
use crate::material::Material;

// This debug attribute implements fmt::Debug which will allow us
// to print the struct using {:?}
#[derive(Debug)]

pub struct RayInfo 
{
    pub intersect: MyVec3, 
    pub normal:    MyVec3, 
    pub ds:        f64,
    pub is_front:  bool,

    pub material: Material
}
