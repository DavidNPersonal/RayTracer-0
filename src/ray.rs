use crate::my_vec3::MyVec3;

// This debug attribute implements fmt::Debug which will allow us
// to print the struct using {:?}
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Ray
{
    pub p:  MyVec3,
    pub direction: MyVec3
}

impl Ray {
    //pub fn origin(&self) -> MyVec3
    //{
    //    return self.p;
    //}

    //pub fn direction(&self) -> MyVec3
    //{
    //    return self.direction;
    //}
    
    pub fn at(&self, ds: f64) -> MyVec3
    {
        return self.p + ds * self.direction;
    }
}
