use crate::{my_vec3::MyVec3, rayinfo::RayInfo, ray::Ray, material::Material, world_sphere::{WESphere, WEMovingSphere}};

pub trait Intersect {
    fn intersect(&self, ray: &Ray, min_scale: f64, max_scale: f64, cast_time: f64) -> (bool, RayInfo);
}

pub struct WorldElement {
    pub objects: Vec<Box<dyn Intersect + Send + Sync>>
}

impl WorldElement 
{
    pub fn intersect_all(&self, ray: &Ray, min_scale: f64, max_scale: f64, cast_time: f64) -> (bool, RayInfo)
    {
        let mut ray_scale_closest = max_scale;
        let mut f_any_intersect   = false;
        let mut info              = RayInfo::default();

        for object in &self.objects
        {
            let (f_intersect, ray_info) = object.intersect(ray, min_scale, max_scale, cast_time);

            if f_intersect
            {
                f_any_intersect = true;
                if ray_info.ds < ray_scale_closest
                {
                    ray_scale_closest = ray_info.ds;
                    info = ray_info;
                }
            }
        }

        return (f_any_intersect, info);
    }

    pub fn add_sphere(&mut self, x: f64, y: f64, z:f64, r: f64, material: Material)
    {
        self.objects.push(Box::new(WESphere{c: MyVec3 {x, y, z}, r, material}));
    }

    pub fn add_moving_sphere(&mut self, x: f64, y: f64, z:f64, r: f64, material: Material, speed: f64, direction: MyVec3)
    {
        let moving_sphere = WEMovingSphere::new(WESphere{c: MyVec3 {x, y, z}, r, material}, speed, direction); 

        self.objects.push(Box::new(moving_sphere));
    }
}
