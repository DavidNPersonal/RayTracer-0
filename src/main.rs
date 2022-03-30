use std::cmp;
use image::{self, RgbImage, ImageBuffer};
use clap::Parser;

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

/*
 * Command-line argument parser
 */
/// Ray Tracing in One Weekend (Rust implementation)
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Horizontal resolution (width) of output image in pixels
    #[clap(long, default_value_t = 1200)]
    image_width: u32,

    /// Vertical resolution (width) of output image in pixels
    #[clap(long, default_value_t = 800)]
    image_height: u32,

    /// Samples per pixel
    #[clap(short, long, default_value_t = 10)]
    samples_per_pixel: u32,

    /// Ray bounce depth (maximum number of ray bounces before declared black)
    #[clap(short, long, default_value_t = 50)]
    ray_bounce_depth: u32,

    /// Number of threads to use (in the range 1 to 32 - out-of-range values will be clamped to this range)
    #[clap(short, long, default_value_t = 8)]
    number_of_threads: u32,
}

/*
 * Entry point
 */
fn main() 
{
    /*
     * Read command-line arguments before setting up parameters
     */

    let args = Args::parse();

    const MIN_NUMBER_OF_THREADS: u32 = 1;
    const MAX_NUMBER_OF_THREADS: u32 = 32;
    const MIN_SAMPLES_PER_PIXEL: u32 = 1;
    const MAX_SAMPLES_PER_PIXEL: u32 = 2000;

    // Main code starts here
    let image_width:  u32    = cmp::max(1, args.image_width);
    let image_height: u32    = cmp::max(1, args.image_height);
    let colour_channels: u32 = 3;

    let aspect_ratio: f64           = image_width as f64 / image_height as f64;
    let field_of_view_vertical: f64 = (20.0/90.0) * std::f64::consts::PI / 2.0;

    let samples_per_pixel: u32     = u32::clamp(args.samples_per_pixel, MIN_SAMPLES_PER_PIXEL, MAX_SAMPLES_PER_PIXEL);
    let max_ray_bounce_depth: u32  = cmp::max  (1, args.ray_bounce_depth);
    let number_of_threads: u32     = u32::clamp(args.number_of_threads, MIN_NUMBER_OF_THREADS, MAX_NUMBER_OF_THREADS);

    /*
    * Create the camera and viewport for our render
    */

    let camera_location  = MyVec3 { x:  13.0, y: 2.0, z: 3.0};
    let camera_target    = MyVec3 { x:   0.0, y: 0.0, z: 0.0};
    let camera_up        = MyVec3 { x:   0.0, y: 1.0, z: 0.0};

    let focus_distance  = 10.0;
    let aperture        = 0.1;
    let exposure_length = 1.0;

    let camera: Camera = Camera::new(camera_location, None, Some(camera_target), Some(camera_up), Some(focus_distance), Some(aperture), Some(exposure_length), aspect_ratio, field_of_view_vertical).unwrap();

    /* 
    * Prepare the world
    */

    let world_element = create_world();

    /* 
    * Render
    */
    let renderer = Renderer::new(image_width, image_height, colour_channels, samples_per_pixel, max_ray_bounce_depth, camera, world_element);

    let bitmap = render(renderer, number_of_threads);

    // Image buffer
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    let mut idx = 0;
    // Write out the final image to a file
    for (_x, _y, pixel) in img.enumerate_pixels_mut()
    {
        let rd = bitmap[idx]; idx += 1;
        let gn = bitmap[idx]; idx += 1;
        let bl = bitmap[idx]; idx += 1;

        *pixel = image::Rgb([rd, gn, bl])
    }

    img.save("test.jpg").unwrap();
    
}
