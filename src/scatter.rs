use crate::{my_vec3::MyVec3, ray::Ray, rayinfo::RayInfo, common::{uniform_random, random_point_in_unit_sphere}};

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum ScatteringType
{
    DiffuseScattering,
    MetallicScattering,
    RefractiveScattering
}


pub fn scatter(r: Ray, ray_info: &RayInfo) -> MyVec3
{
    match ray_info.material.surface
    {
        ScatteringType::DiffuseScattering    => diffuse_scatter   (r, ray_info.normal),
        ScatteringType::MetallicScattering   => metallic_scatter  (r, ray_info.normal, ray_info.material.metal_fuzz.unwrap_or(0.0)),
        ScatteringType::RefractiveScattering => refractive_scatter(r, ray_info.normal, ray_info.is_front, 0.0, ray_info.material.index_of_refraction.unwrap_or(0.0)),
    }
}

pub fn diffuse_scatter(_ray: Ray, normal: MyVec3) -> MyVec3
{
    // Generate the scattering ray, unless the ray is very small (thus causing numerical errors and possible NaNs) in which case we regenerate
    let mut scatter_offset = random_point_in_unit_sphere();

    // Random point on the unit circle
    scatter_offset.normalize();

    // Generate the scattering ray, unless the ray is very small (thus causing numerical errors and possible NaNs) in which case we just use the normal
    let mut scatter_direction = normal + scatter_offset;

    if scatter_direction.squared_length() < 1e-12
    {
        scatter_direction = normal;
    }

    return scatter_direction;
}

pub fn metallic_scatter(ray: Ray, normal: MyVec3, fuzz_extent: f64) -> MyVec3
{
    let ray_to_normal_projection = ray.direction.dot(normal) * normal;
    let mut scatter_direction    = ray.direction - 2.0 * ray_to_normal_projection;

    // Fuzzy reflections
    let fuzz_vector = fuzz_extent * random_point_in_unit_sphere();
    
    // Normalize before adding fuzz so that the fuzz is consistently applied for all input angles
    // Consider removing for speed
    scatter_direction.normalize();

    scatter_direction = scatter_direction + fuzz_vector;

    // Ensure that the generated ray points out, if not replace with the normal
    scatter_direction = if normal.dot(scatter_direction) > 0.0 {scatter_direction} else {normal};

    return scatter_direction;
}

pub fn refractive_scatter(ray: Ray, normal: MyVec3, is_front: bool, reflectivity: f64, refractive_index: f64) -> MyVec3
{
    // Some reflections and some refraction
    let x = uniform_random();
    
    if x < reflectivity  // Fixed surface reflection
    {
        return reflect(ray, normal);
    }
    else        // Refraction or reflection dependent on the angle of incidence (since total internal reflection can sometimes occur rather than refraction when going from high refractive indices to low)
    {
        let eta = if is_front {1.0 / refractive_index} else {refractive_index};

        let mut ray_unit_vector = ray.direction;

        ray_unit_vector.normalize();

        let cos_incident_angle = f64::min (-1.0 * ray_unit_vector.dot(normal), 1.0);
        let sin_incident_angle = f64::sqrt(1.0 - cos_incident_angle * cos_incident_angle);

        // Check for total internal reflection, and also provide a probability of reflection based on the incidence angle (some reflections, some refractions dependent on the angle)
        if eta * sin_incident_angle > 1.0 || schlick_approximation(cos_incident_angle, eta) > uniform_random()
        {
            return reflect(ray, normal);
        }
        else
        {
            let c2 = eta * (ray_unit_vector + cos_incident_angle * normal);
            let c1 = -1.0 * f64::sqrt(f64::abs(1.0 - c2.squared_length())) * normal;

            return c1 + c2
        }
    }

    // Used to approximate the percentage of light (rays) reflected from a refractive surface based on the incident angle
    fn schlick_approximation(cos_incident_angle: f64, index_of_refraction: f64) -> f64
    {
        let mut r0 = (1.0 - index_of_refraction) / (1.0 + index_of_refraction);
        
        r0 = r0 * r0;
        
        r0 + (1.0 - r0) * f64::powi(1.0 - cos_incident_angle, 5)
    }

    fn reflect(ray: Ray, normal: MyVec3) -> MyVec3
    {
        ray.direction - 2.0 * ray.direction.dot(normal) * normal
    }
}
