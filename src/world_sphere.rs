use crate::{my_vec3::MyVec3, rayinfo::RayInfo, ray::Ray, world_element::{Intersect}, material::Material, scatter::ScatteringType};

// This debug attribute implements fmt::Debug which will allow us
// to print the struct using {:?}
#[derive(Debug)]

pub struct WESphere {
    pub c: MyVec3,
    pub r: f64,
    pub material: Material
}

impl Intersect for WESphere {
    fn intersect(&self, r: &Ray, min_scale: f64, max_scale: f64) -> (bool, RayInfo)
    {
        let sqrt = f64::sqrt; 

        let a = r.direction.squared_length();
        let h = r.direction.dot(r.p - self.c);   	// b = 2h
        let c = (r.p - self.c).squared_length() - 1.0 * self.r * self.r;

        let s = h * h - a * c;

        let rfake = RayInfo{intersect: MyVec3{x:0.0, y:0.0, z:0.0}, normal: MyVec3{x:0.0, y:0.0, z:0.0}, ds: 0.0, is_front: false, material: Material{surface: ScatteringType::DiffuseScattering, attenuation: MyVec3{x: 0.5, y: 0.5, z: 0.5}, metal_fuzz: None, index_of_refraction: None}};

        if s >= 0.0
        {
            let mut ds = (-h - sqrt(s)) / a;

            if ds < min_scale || ds > max_scale
            {
                ds = (-h + sqrt(s)) / a;
                if ds < min_scale || ds > max_scale
                {
                    return (false, rfake);
                }
            }

            let intersect = r.at(ds);

            let mut normal = (intersect - self.c) / self.r;		// This vector is already normalized to length = 1, so no explicit normalization step is necessary
            //let mut normal = intersect - self.c;
            //normal.normalize();

            let is_front  = normal.dot(r.direction) < 0.0;
            normal = if is_front {normal} else {-1.0 * normal};

            let ray_info = RayInfo{intersect, normal, ds, is_front, material: self.material};

            return (true, ray_info);
        }

        return(false, rfake);
    }

}
