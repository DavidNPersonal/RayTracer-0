use image::{self, RgbImage, ImageBuffer};


// Yes, I know there is a Vec3 crate which is probably more suitable but the goal is to learn Rust so I've implemented my own for practice (to be replaced later)
mod my_vec3;
mod camera;
mod rayinfo;
mod ray;
mod world_element;
mod world_sphere;
mod material;
mod scatter;
mod common;
mod create_world;
mod renderer;

use crate::create_world::create_world;
use crate::my_vec3::MyVec3;
use crate::camera::Camera;
use crate::renderer::{Renderer, render};

fn main() 
{
    println!("Hello, world!");

    // Main code starts here
    let   image_width:  u32 = 1200;
    let   image_height: u32 = 800;
    let   clrdepth: u32     = 3;

    let   aspect_ratio: f64           = image_width as f64 / image_height as f64;
    let   field_of_view_vertical: f64 = (20.0/90.0) * std::f64::consts::PI / 2.0;

    let samples_per_pixel: u32    = 10;//500;
    let max_ray_bounce_depth: u32 = 50;
   
    /*
    * Create the camera and viewport for our render
    */

    let camera_location  = MyVec3 { x:  13.0, y: 2.0, z: 3.0};
    let camera_target    = MyVec3 { x:   0.0, y: 0.0, z: 0.0};
    let camera_up        = MyVec3 { x:   0.0, y: 1.0, z: 0.0};

    let focus_distance = 10.0;
    let aperture       = 0.1;

    let camera: Camera = Camera::new(camera_location, None, Some(camera_target), Some(camera_up), Some(focus_distance), Some(aperture), aspect_ratio, field_of_view_vertical).unwrap();

    /* 
    * Prepare the world
    */

    let world_element = create_world();

    /* 
    * Render
    */
    let renderer: Renderer = Renderer::new(image_width, image_height, clrdepth, samples_per_pixel, max_ray_bounce_depth, camera, world_element);
    let bitmap    = render(renderer);
    
    let mut idx = 0;

    // Image buffer
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    // Write out the final image to a file
    for (_x, _y, pixel) in img.enumerate_pixels_mut()
    {
        let rd = bitmap[idx]; idx = idx + 1;
        let gn = bitmap[idx]; idx = idx + 1;
        let bl = bitmap[idx]; idx = idx + 1;

        *pixel = image::Rgb([rd, gn, bl])
    }

    img.save("test.png").unwrap();
    img.save("test.jpg").unwrap();
    
}
