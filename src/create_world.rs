use crate::{world_element::WorldElement, material::{Material, self}, common::{uniform_random, random_in_interval}, my_vec3::{MyVec3, random_vec3, random_in_interval_vec3}, scatter::ScatteringType};

pub fn create_world() -> WorldElement
{
    let mut world_element = WorldElement{objects: vec![]};

    world_element.add_sphere( 0.0, -1000.0, 0.0, 1000.0, material::NEUTRAL_GREY);

    // Small spheres, scattered about the ground
    for m in -11..=11
    {
        for n in -11..=11
        {
            let material_chooser = uniform_random();
            let sphere_centre    = MyVec3 {x: m as f64 + 0.9 * uniform_random(), y: 0.2, z: n as f64 + 0.9 * uniform_random()};

            if (sphere_centre - MyVec3{x:4.0, y:0.2, z:0.0}).length() > 0.9
            {
                if material_chooser < 0.8
                {
                    // Choose a diffuse material
                    let gain                    = random_vec3() * random_vec3();
                    let random_diffuse_material = Material{surface: ScatteringType::DiffuseScattering, gain, metal_fuzz: None, index_of_refraction: None};

                    world_element.add_sphere(sphere_centre.x, sphere_centre.y, sphere_centre.z, 0.2, random_diffuse_material);
                }
                else if material_chooser < 0.95
                {
                    // Choose a metallic material
                    let gain = random_in_interval_vec3(0.5, 1.0);
                    let fuzz = random_in_interval     (0.0, 0.5);

                    let random_metallic_material = Material{surface: ScatteringType::MetallicScattering, gain, metal_fuzz: Some(fuzz), index_of_refraction: None};
                    world_element.add_sphere(sphere_centre.x, sphere_centre.y, sphere_centre.z, 0.2, random_metallic_material);
                }
                else
                {
                    // Refractive material (glass)
                    world_element.add_sphere(sphere_centre.x, sphere_centre.y, sphere_centre.z, 0.2, material::GLASS);
                }
            }
        }
    }

    // Large spheres, centered
    let diffuse_material_large  = Material{surface: ScatteringType::DiffuseScattering,  gain: MyVec3{x:0.4, y: 0.2, z: 0.1}, metal_fuzz: None,      index_of_refraction: None};
    let metallic_material_large = Material{surface: ScatteringType::MetallicScattering, gain: MyVec3{x:0.7, y: 0.6, z: 0.5}, metal_fuzz: Some(0.0), index_of_refraction: None};

    world_element.add_sphere( 0.0, 1.0, 0.0, 1.0, material::GLASS);
    world_element.add_sphere(-4.0, 1.0, 0.0, 1.0, diffuse_material_large);
    //world_element.add_sphere( 4.0, 1.0, 0.0, 1.0, metallic_material_large);
    world_element.add_moving_sphere( 4.0, 1.0, 0.0, 1.0, metallic_material_large, 0.25, MyVec3{x: 1.0, y: 0.0, z: 0.0});

    return world_element;
}