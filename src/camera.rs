use crate::{common::uniform_within_unit_circle, my_vec3::{MyVec3, vec3_normalize}, ray::Ray};

#[derive(Debug, Copy, Clone)]
pub struct Viewport
{
    pub distance: f64,
    pub width:    f64,
    pub height:   f64,

    pub up: MyVec3,

    pub horizontal_vector: MyVec3,
    pub vertical_vector:   MyVec3,

    pub reference_corner: MyVec3,
}

impl Viewport
{
    pub fn new(distance: f64,
               height: f64,
               width: f64,
               up: MyVec3,
               horizontal_vector: MyVec3,
               vertical_vector: MyVec3,
               camera_location: MyVec3,
               camera_direction: MyVec3)
               -> Viewport
    {
        /*
         * -1.0 multipliers are explained by the following:
         *    +ve horizonatal is right (as seen by the camera)
         *    +ve vertical    is up    (as seen by the camera)
         */
        let horizontal_offset = -1.0 * (width as f64 / 2.0) * horizontal_vector;
        let vertical_offset   = (height as f64 / 2.0) * vertical_vector;

        // This is the upper left corner of the viewport
        let reference_corner  = camera_location + distance * camera_direction + horizontal_offset + vertical_offset;

        Viewport { distance,
                   width,
                   height,
                   up,
                   horizontal_vector,
                   vertical_vector,
                   reference_corner }
    }
}

pub struct Camera
{
    pub location:  MyVec3,
    pub direction: MyVec3,

    pub aspect_ratio: f64,

    pub focus_distance:  Option<f64>,
    pub lens_radius:     Option<f64>,
    pub exposure_length: Option<f64>,

    pub viewport: Viewport,
}

impl Camera
{
    pub fn new(location: MyVec3,
               camera_direction: Option<MyVec3>,
               camera_target: Option<MyVec3>,
               camera_up: Option<MyVec3>,
               focus_distance: Option<f64>,
               aperture: Option<f64>,
               exposure_length: Option<f64>,
               aspect_ratio: f64,
               vertical_fov: f64)
               -> Result<Camera, String>
    {

        // Derive the viewport parameters before initializing the camera
        let up                = vec3_normalize(camera_up.unwrap_or(MyVec3 { x: 0.0, y: 1.0, z: 0.0 }));
        let viewport_distance = focus_distance.unwrap_or(1.0);

        let direction: MyVec3 = vec3_normalize(
                                match (camera_direction, camera_target)
                                {
                                   (Some(camera_direction), None) => { camera_direction }
                                   (None, Some(target))           => { target - location }
                                   (Some(_), Some(_))             => { return Err("Camera fn new: Both camera direction and target position specified. Only one of camera direction and target position should be specified when defining camera position, orientation, and direction".to_string()); }
                                   (None, None)                   => { return Err("Camera fn new: No camera direction or target position specified. One of camera direction and target position must be specified when defining camera position, orientation, and direction".to_string()); }
                               }
        );

        let lens_radius = match aperture
        {
            None           => { None }
            Some(aperture) => { Some(aperture / 2.0) }
        };
        

        // Calculate the horizontal and vertical orientation of the viewport as unit vectors; used to calculate ray target locations on the viewport
        let horizontal_vector = vec3_normalize(up.cross(-1.0 * direction));
        let vertical_vector   = vec3_normalize(-1.0 * direction.cross(horizontal_vector));

        let viewport_height   = 2.0 * f64::abs(viewport_distance) * f64::tan(vertical_fov / 2.0);

        let viewport          = Viewport::new(viewport_distance,
                                              viewport_height,
                                              aspect_ratio * viewport_height,
                                              up,
                                              horizontal_vector,
                                              vertical_vector,
                                              location,
                                              direction);

        Ok(Camera { aspect_ratio, location, direction, focus_distance, lens_radius, exposure_length, viewport })
    }


    pub fn generate_ray(&self, viewport_coord: MyVec3, cast_time: f64) -> Ray
    {
        match self.lens_radius
        {
            // Pinhole camera
            None => { return Ray { p: self.location, direction: viewport_coord - self.location, cast_time } }

            Some(lens_radius) =>
            {
                // Simple lens
                let lens_centre     = self.location;
                let lens_offset     = lens_radius * uniform_within_unit_circle();
                let lens_ray_origin = lens_centre + lens_offset.x * self.viewport.horizontal_vector + lens_offset * self.viewport.vertical_vector;

                return Ray { p: lens_ray_origin, direction: viewport_coord - lens_ray_origin, cast_time };
            }
        }
    }
}
