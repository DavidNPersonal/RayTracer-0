
// The bounding box is defined as the space between six planes
// by making these planes aligned with the axes, each of the six
// planes may be described by a single co-ordinate
// It may be easiest to think of these as "front/back", "left/right",
// "top/bottom" for conceptual understanding, although of course
// in a scene, these designations are decided by the camera position and
// direction
// A further advantage is that intersection with axis-aligned planes is 
// computationally (and conceptually) very simple

use crate::{world_element::Intersect, ray::Ray, rayinfo::RayInfo, common::order_pair};

struct WEBoundingBox {

    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64

}

impl Intersect for WEBoundingBox {
    fn intersect(&self, ray: &Ray, _min_scale: f64, _max_scale: f64, _cast_time: f64) -> (bool, RayInfo)
    {
        // Any non-axis aligned ray will intersect every plane
        // The intersect for each is calculated as the range of ray distances which are between two parallel planes
        // An intersect with the interior of the bounding box occurs where there is a one or a range of values
        // for the distance of the ray which lies between each of the three pairs of bounding planes

        // Take care of NaN and Inf
        // Note that +/- Inf does not technically need to be managed as it will give correct behaviour but the
        // performance hit is very small due to the rarity of rays which have exactly 0.0 in some axis
        if ray.direction.x == 0.0 || ray.direction.y == 0.0 || ray.direction.z == 0.0
        {
            // Declare a hit since bounding boxes are only used to determine whether or not there is  
            // potentially an object within the box (thus it is a small overhead to return a hit, as
            // the ray is then tested individually against objects within the box)
            return(true, RayInfo::default());
        }

        // x0 = p.x + k * direction.x for some k, k is the ray scaling factor
        // This equation extends to each of the six planes

        let k_x0 = (self.x0 - ray.p.x) / ray.direction.x;
        let k_x1 = (self.x1 - ray.p.x) / ray.direction.x;

        let k_y0 = (self.y0 - ray.p.y) / ray.direction.y;
        let k_y1 = (self.y1 - ray.p.y) / ray.direction.y;

        let k_z0 = (self.z0 - ray.p.z) / ray.direction.z;
        let k_z1 = (self.z1 - ray.p.z) / ray.direction.z;

        // Intersect if there exists at least one value of k which is contained within each interval [k_x0, k_x1], [k_y0, k_y1], and [k_z0, k_z1]
        // Begin by ordering the scaling factors since we do not know where the ray is emanating from relative to the box (e.g. inside or outside)
        let (a0, b0) = order_pair(k_x0, k_x1);
        let (a1, b1) = order_pair(k_y0, k_y1);
        let (a2, b2) = order_pair(k_z0, k_z1);

        // Determine the upper and lower bounds on k based on the lowest and highest values in each interval
        let lower_bound = f64::max(f64::max(a0, a1), a2);
        let upper_bound = f64::min(f64::min(b0, b1), b2);

        if lower_bound < upper_bound
        {
            return(true, RayInfo::default());
        }        
        else
        {
            return(false, RayInfo::default());
        }
    }
}