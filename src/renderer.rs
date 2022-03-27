use crate::{ray::Ray, camera::Camera, my_vec3::MyVec3, world_element::WorldElement, scatter::scatter};

use rand::Rng;
use crossbeam_utils::thread;

// A structure created to workaround one of Rust's deficiencies (not being able to use different elements of a vector as function parameters in multiple functions)
#[derive(Copy, Clone)]
pub struct RenderScope
{
    begin_line: u32,
    number_of_lines: u32
}

pub struct Renderer
{
    image_width: u32,
    image_height: u32, 
    colour_channels: u32, 
    samples_per_pixel: u32, 
    max_ray_bounce_depth: u32, 
    camera: Camera, 
    world_element: WorldElement
}

impl Renderer 
{
    pub fn new(image_width: u32, image_height: u32, colour_channels: u32, samples_per_pixel: u32, max_ray_bounce_depth: u32, camera: Camera, world_element: WorldElement) -> Renderer
    {
        Renderer{
            image_width,
            image_height,
            colour_channels, 
            samples_per_pixel, 
            max_ray_bounce_depth, 
            camera, 
            world_element
        }
    }
}

pub fn render(rdr: Renderer, target_number_of_threads: u32) -> Vec<u8>
{
    let viewport        = rdr.camera.viewport;

    let horizontal_step   =        (viewport.width  as f64 / rdr.image_width  as f64) * viewport.horizontal_vector;
    let vertical_step     = -1.0 * (viewport.height as f64 / rdr.image_height as f64) * viewport.vertical_vector;

    let mut number_of_threads = target_number_of_threads;
    let mut lines_per_thread  = (rdr.image_height as f64 / number_of_threads as f64).ceil() as u32;

    // Take care of the degenerate case where there are as many threads as horizontal lines in the image by reducing the number of threads
    while lines_per_thread * (number_of_threads - 1) >= rdr.image_height
    {
        number_of_threads = number_of_threads - 1;
        lines_per_thread  = (rdr.image_height as f64 / number_of_threads as f64).ceil() as u32;
    }

    /*
     * Temporary memory locations to which each thread will write (one per thread)
     * Concatenated to give the full image at the end of the function
     */

    let mut submap: Vec<Vec<u8>> = Vec::with_capacity(number_of_threads as usize);
    for _ in 0..number_of_threads
    {
        submap.push(vec![0; (rdr.image_width * rdr.colour_channels * lines_per_thread) as usize]);
    }

    /*
     * Prepare the thread rendering parameters ahead of time to allow for iterators to be used
     * as a way to get around the ownership problem of accessing the elements of a vector directly
     */

    let mut render_scope: Vec<RenderScope> = Vec::with_capacity(number_of_threads as usize);
    let mut break_line:           Vec<u32> = Vec::with_capacity(number_of_threads as usize);

    // Make a list of the number of lines of the image to process per thread, first (number_of_threads - 1)
    // are added, then the final thread is allocated all remaining lines (this will be a different number
    // lines where the total number of lines of the image is not divisible by the number of threads)
    for n in 1..number_of_threads       
    {
        break_line.push(lines_per_thread * n);
    }
    break_line.push(rdr.image_height);
    break_line.dedup();

    let mut begin_line: u32 = 0;
    for end_line in break_line
    {
        render_scope.push(RenderScope{begin_line, number_of_lines: end_line - begin_line});

        begin_line = end_line;
    }

    /*
     * Multi-threaded rendering
     */

    thread::scope(|scope| {

        for (x, z) in submap.iter_mut().zip(&mut render_scope)
        {
            scope.spawn(|_| {   
                render_lines(&rdr, horizontal_step, vertical_step, &mut *x, *z);
            });
        }

    }).unwrap();
    
    /*
     * Write the final image out to a vector once all the threads have terminated (guaranteed by thread::scope)
     */

    let mut bitmap = Vec::with_capacity((rdr.image_width * rdr.colour_channels * lines_per_thread * number_of_threads) as usize);

    for x in submap.iter() 
    {
        bitmap.extend(x);
    }

    bitmap.resize((rdr.image_width * rdr.image_height * rdr.colour_channels) as usize, 0);

    return bitmap;
}


pub fn render_lines(rdr: &Renderer, horizontal_step: MyVec3, vertical_step: MyVec3, bitmap: &mut [u8], rsc: RenderScope)
{
    let mut rng = rand::thread_rng();

    let begin_line      = rsc.begin_line;
    let number_of_lines = rsc.number_of_lines;

    for y in 0..number_of_lines
    {
        let mut bitmap_idx: usize = (y * rdr.image_width * rdr.colour_channels) as usize;

        let viewport_row  = rdr.camera.viewport.reference_corner + (begin_line + y) as f64 * vertical_step;

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

                    let scatter_direction = scatter(r, &ray_info);

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