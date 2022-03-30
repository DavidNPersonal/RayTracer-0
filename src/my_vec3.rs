
use std::ops;

use crate::common::{uniform_random, random_in_interval};

//
#[derive(Debug, Copy, Clone, Default)]

pub struct MyVec3 {
    pub x: f64, 
    pub y: f64, 
    pub z: f64
}

impl MyVec3 {
    pub fn squared_length(&self) -> f64
    {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn length(&self) -> f64
    {
        f64::sqrt(self.squared_length())
    }

    pub fn normalize(&mut self)
    {
        let recip_length = 1.0 / self.length();

        self.x = self.x * recip_length;
        self.y = self.y * recip_length;
        self.z = self.z * recip_length;
    }

    pub fn dot(&self, rhs: MyVec3) -> f64
    {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: MyVec3) -> MyVec3
    {
        MyVec3{x: self.y * rhs.z - self.z * rhs.y, y: self.z * rhs.x - self.x * rhs.z, z: self.x * rhs.y - self.y * rhs.x}
    }
}


impl ops::Add for MyVec3 
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self 
    {
        return Self {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl ops::Sub for MyVec3 
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self 
    {
        return Self {x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl ops::Mul for MyVec3 
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self 
    {
        return Self {x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z}
    }
}


impl ops::Div<f64> for MyVec3 
{
    type Output = Self;

    fn div(self, rhs: f64) -> Self 
    {
        return Self {x: self.x / rhs, y: self.y / rhs, z: self.z / rhs}
    }
}


impl ops::Mul<MyVec3> for f64
{
    type Output = MyVec3;

    fn mul(self, rhs: MyVec3) -> MyVec3 
    {
        return MyVec3 {x: rhs.x * self, y: rhs.y * self, z: rhs.z * self};
    }
}


pub fn random_vec3() -> MyVec3
{
    MyVec3{x: uniform_random(), y: uniform_random(), z: uniform_random()}
}


pub fn random_in_interval_vec3(min: f64, max: f64) -> MyVec3
{
    MyVec3{x: random_in_interval(min, max), y: random_in_interval(min, max), z: random_in_interval(min, max)}
}


pub fn vec3_normalize(v: MyVec3) -> MyVec3
{
    MyVec3 {x: v.x / v.length(), y: v.y / v.length(), z: v.z / v.length()}
}
