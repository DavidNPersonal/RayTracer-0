use crate::{my_vec3::MyVec3, rayinfo::RayInfo, ray::Ray, material::Material, scatter::ScatteringType, world_sphere::WESphere};

pub trait Intersect {
    fn intersect(&self, r: &Ray, min_scale: f64, max_scale: f64) -> (bool, RayInfo);
}

pub struct WorldElement {
    pub objects: Vec<Box<dyn Intersect + Send + Sync>>
}

impl WorldElement 
{
    pub fn intersect_all(&self, r: &Ray, min_scale: f64, max_scale: f64) -> (bool, RayInfo)
    {
        let mut ray_scale_closest: f64 = max_scale;
        let mut f_any_intersect = false;
        let mut info = RayInfo{intersect: MyVec3{x:0.0, y:0.0, z:0.0}, normal: MyVec3{x:0.0, y:0.0, z:0.0}, ds: 0.0, is_front: false, material: Material{surface: ScatteringType::DiffuseScattering, attenuation: MyVec3{x: 0.5, y: 0.5, z: 0.5}, metal_fuzz: None, index_of_refraction: None}};

        for object in &self.objects
        {
            let (f_intersect, ray_info) = object.intersect(r, min_scale, max_scale);

            if f_intersect{
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
}
