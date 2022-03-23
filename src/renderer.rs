use crate::{ray::Ray, camera::{Camera, Viewport}, my_vec3::MyVec3, world_element::WorldElement, scatter};

use rand::Rng;
//use std::thread;
use crossbeam_utils::thread;

pub struct Renderer
{
    image_width: u32,
    image_height: u32, 
    clrdepth: u32, 
    samples_per_pixel: u32, 
    max_ray_bounce_depth: u32, 
    camera: Camera, 
    world_element: WorldElement
}

impl Renderer 
{
    pub fn new(image_width: u32, image_height: u32, clrdepth: u32, samples_per_pixel: u32, max_ray_bounce_depth: u32, camera: Camera, world_element: WorldElement) -> Renderer
    {
        Renderer{
            image_width,
            image_height,
            clrdepth, 
            samples_per_pixel, 
            max_ray_bounce_depth, 
            camera, 
            world_element
        }
    }
}

    pub fn render(rdr: Renderer) -> Vec<u8>
    {
        let viewport: Viewport = rdr.camera.viewport;

        let horizontal_step   =        (viewport.width  as f64 / rdr.image_width  as f64) * viewport.horizontal_vector;
        let vertical_step     = -1.0 * (viewport.height as f64 / rdr.image_height as f64) * viewport.vertical_vector;

        let clrdepth    = rdr.clrdepth;
        let image_width = rdr.image_width;

        let number_of_threads = 4; 
        let lines_per_thread  = rdr.image_height / number_of_threads;

        let mut break_line: Vec<u32> = Vec::new();
        for n in 1..number_of_threads       // Actually goes from 1 to (number_of_threads - 1), this is important as we do not want the final value to be written in case the image height is not exacly divisible by the number of threads
        {
            break_line.push(lines_per_thread * n);
        }
        break_line.push(rdr.image_height);
        break_line.dedup();                 // Remove duplicates, which should not happen, but just in case

        let mut bitmap: Vec<u8> = vec![0; (rdr.image_width * rdr.image_height * rdr.clrdepth) as usize];
        
        let mut start: u32 = 0;

        
        //thread::scope(|scope| {
            for end in break_line
            {
                let first_element = (start * clrdepth * image_width) as usize;
                let final_element = (end *   clrdepth * image_width) as usize;
                let number_of_lines = end - start;

                start = end;

                //scope.spawn( |_| {
                    let a = first_element;
                    let b = final_element;
                    let c = number_of_lines;

                    render_lines(&rdr, horizontal_step, vertical_step, &mut bitmap[a..b], a as u32, c as u32);
                //});
                
            }
        //}).unwrap();

        return bitmap;
    }


    pub fn render_lines(rdr: &Renderer, horizontal_step: MyVec3, vertical_step: MyVec3, bitmap: &mut [u8], first_line: u32, number_of_lines: u32)
    {
        let mut rng = rand::thread_rng();

        for y in 0..number_of_lines
        {
            let mut bitmap_idx: usize = (y * rdr.image_width * rdr.clrdepth) as usize;

            let viewport_row  = rdr.camera.viewport.reference_corner + (first_line + y) as f64 * vertical_step;

            for x in 0..rdr.image_width
            {
                let mut final_colour = MyVec3{x:0.0, y:0.0, z:0.0};

                let viewport_current = viewport_row + x as f64 * horizontal_step;

                for _s in 0..rdr.samples_per_pixel
                {
                    let mut total_attenuation = MyVec3{x:1.0, y:1.0, z:1.0};
                    let mut ray_bounce           = 0;

                    let     viewport_offset   = (rng.gen::<f64>() - 0.5) * vertical_step + (rng.gen::<f64>() - 0.5) * horizontal_step;

                    // Cast the initial ray from the camera
                    let mut r = rdr.camera.generate_ray(viewport_current + viewport_offset);

                    loop {
                        let (f_intersect, ray_info) = rdr.world_element.intersect_all(&r, 0.001, f64::INFINITY);

                        if f_intersect == false || ray_bounce > rdr.max_ray_bounce_depth
                        {
                            break;
                        }

                        let scatter_direction = scatter::scatter(r, &ray_info);

                        r = Ray{p: ray_info.intersect, direction: scatter_direction};

                        total_attenuation = total_attenuation * ray_info.material.attenuation;

                        ray_bounce = ray_bounce + 1;
                    }

                    // Colour is determined by the ray's final direction (i.e. the ray which is the source of the light which comes from the background in this case)
                    let mut unit_direction = r.direction;
                    unit_direction.normalize();
                    let t = 0.5 * (unit_direction.y + 1.0);

                    let sky_box_colour = (1.0 - t) * MyVec3{x: 1.0, y: 1.0, z: 1.0} + t * MyVec3{x: 0.5, y: 0.7, z: 1.0};

                    final_colour = final_colour + total_attenuation * sky_box_colour;
                }

                final_colour = final_colour / rdr.samples_per_pixel as f64;

                // Coarse gamma adjust
                let rd = f64::clamp(f64::sqrt(final_colour.x), 0.0, 0.999);
                let gn = f64::clamp(f64::sqrt(final_colour.y), 0.0, 0.999);
                let bl = f64::clamp(f64::sqrt(final_colour.z), 0.0, 0.999);

                bitmap[bitmap_idx] = (256.0 * rd) as u8; bitmap_idx = bitmap_idx + 1;
                bitmap[bitmap_idx] = (256.0 * gn) as u8; bitmap_idx = bitmap_idx + 1;
                bitmap[bitmap_idx] = (256.0 * bl) as u8; bitmap_idx = bitmap_idx + 1;
            }
        }
    }