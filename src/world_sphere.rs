use crate::{my_vec3::{MyVec3, vec3_normalize}, rayinfo::RayInfo, ray::Ray, world_element::{Intersect}, material::Material};

// This sphere can only move in a straight line, and does not stop
// Elements not declared pub in order to force the use of new to instantiate (thereby normalizing direction at the time of creation)
pub struct WEMovingSphere {
    sphere_zero: WESphere,
    speed:       f64,
    direction:   MyVec3
}

impl WEMovingSphere {
    pub fn new(sphere_zero: WESphere, speed: f64, direction: MyVec3) -> WEMovingSphere
    {
        WEMovingSphere { sphere_zero, speed, direction: vec3_normalize(direction) }
    }
}

impl Intersect for WEMovingSphere {
    fn intersect(&self, ray: &Ray, min_scale: f64, max_scale: f64, cast_time: f64) -> (bool, RayInfo)
    {
        // Determine the position of the sphere at the cast time of the ray, and intersect
        let c = self.sphere_zero.c + (cast_time * self.speed) * self.direction;
        let r = self.sphere_zero.r;

        let sphere_t = WESphere{ c, r, material: self.sphere_zero.material };

        return sphere_t.intersect(ray, min_scale, max_scale, cast_time);
    }
}

// Non-moving sphere
pub struct WESphere {
    pub c:        MyVec3,
    pub r:        f64,
    pub material: Material
}

impl Intersect for WESphere {
    fn intersect(&self, ray: &Ray, min_scale: f64, max_scale: f64, _cast_time: f64) -> (bool, RayInfo)
    {
        let sqrt = f64::sqrt; 

        let a = ray.direction.squared_length();
        let h = ray.direction.dot(ray.p - self.c);   	// b = 2h
        let c = (ray.p - self.c).squared_length() - 1.0 * self.r * self.r;

        let s = h * h - a * c;

        if s >= 0.0
        {
            let mut ds = (-h - sqrt(s)) / a;

            if ds < min_scale || ds > max_scale
            {
                ds = (-h + sqrt(s)) / a;
                if ds < min_scale || ds > max_scale
                {
                    return (false, RayInfo::default());
                }
            }

            let intersect = ray.at(ds);

            let mut normal = (intersect - self.c) / self.r;		// This vector is already normalized to length = 1, so no explicit normalization step is necessary
            //let mut normal = intersect - self.c;
            //normal.normalize();

            let is_front  = normal.dot(ray.direction) < 0.0;
            normal        = if is_front {normal} else {-1.0 * normal};

            let ray_info  = RayInfo{intersect, normal, ds, is_front, material: self.material};

            return (true, ray_info);
        }

        return(false, RayInfo::default());
    }

}
