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
            let sphere_centre = MyVec3 {x: m as f64 + 0.9 * uniform_random(), y: 0.2, z: n as f64 + 0.9 * uniform_random()};

            if (sphere_centre - MyVec3{x:4.0, y:0.2, z:0.0}).length() > 0.9
            {
                if material_chooser < 0.8
                {
                    // Choose a diffuse material
                    let attenuation = random_vec3() * random_vec3();
                    let random_diffuse_material = Material{surface: ScatteringType::DiffuseScattering, attenuation, metal_fuzz: None, index_of_refraction: None};

                    world_element.add_sphere(sphere_centre.x, sphere_centre.y, sphere_centre.z, 0.2, random_diffuse_material);
                }
                else if material_chooser < 0.95
                {
                    // Choose a metallic material
                    let attenuation = random_in_interval_vec3(0.5, 1.0);
                    let fuzz          = random_in_interval(0.0, 0.5);

                    let random_metallic_material = Material{surface: ScatteringType::MetallicScattering, attenuation, metal_fuzz: Some(fuzz), index_of_refraction: None};
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
    let diffuse_material_large  = Material{surface: ScatteringType::DiffuseScattering,  attenuation: MyVec3{x:0.4, y: 0.2, z: 0.1}, metal_fuzz: None,      index_of_refraction: None};
    let metallic_material_large = Material{surface: ScatteringType::MetallicScattering, attenuation: MyVec3{x:0.7, y: 0.6, z: 0.5}, metal_fuzz: Some(0.0), index_of_refraction: None};

    world_element.add_sphere( 0.0, 1.0, 0.0, 1.0, material::GLASS);
    world_element.add_sphere(-4.0, 1.0, 0.0, 1.0, diffuse_material_large);
    world_element.add_sphere( 4.0, 1.0, 0.0, 1.0, metallic_material_large);


    //const MATERIAL_GROUND: Material = Material{surface: ScatteringType::DiffuseScattering,  attenuation: MyVec3 {x: 0.8, y: 0.8, z: 0.0}, metal_fuzz: 0.0, index_of_refraction: 1.0};
    //const MATERIAL_CENTER: Material = Material{surface: ScatteringType::DiffuseScattering,  attenuation: MyVec3 {x: 0.1, y: 0.2, z: 0.5}, metal_fuzz: 0.0, index_of_refraction: 1.0};
    //#[allow(dead_code)]
    //const MATERIAL_LEFT:   Material = Material{surface: ScatteringType::RefractiveScattering, attenuation: MyVec3 {x: 1.0, y: 1.0, z: 1.0}, metal_fuzz: 0.0, index_of_refraction: 1.5};
    //const MATERIAL_RIGHT:  Material = Material{surface: ScatteringType::MetallicScattering,   attenuation: MyVec3 {x: 0.8, y: 0.6, z: 0.2}, metal_fuzz: 0.0, index_of_refraction: 1.0};

    /*
    world_element.add_sphere( 0.0, -100.5, -1.0, 100.0, MATERIAL_GROUND);
    world_element.add_sphere( 0.0, 0.0, -1.0, 0.5, MATERIAL_CENTER);
    //world_element.add_sphere(-1.0, 0.0, -1.0, 0.5, MATERIAL_LEFT);
    world_element.add_sphere( 1.0, 0.0, -1.0, 0.5, MATERIAL_RIGHT);

    //world_element.add_sphere( 0.0, 0.0, -1.0, 0.5, MATERIAL_GLASS);//MATERIAL_CENTER);
    world_element.add_sphere(-1.0, 0.0, -1.0, 0.5, material::GLASS);//MATERIAL_LEFT);
    world_element.add_sphere(-1.0, 0.0, -1.0, -0.4, material::GLASS);//MATERIAL_LEFT);
    //world_element.add_sphere( 1.0, 0.0, -1.0, 0.5, MATERIAL_RIGHT);
    */

    //let Rtt = f64::cos(std::f64::consts::PI / 4.0);
    //world_element.add_sphere( -Rtt, 0.0, -1.0, Rtt, material::PURE_BLUE);
    //world_element.add_sphere( Rtt, 0.0, -1.0, Rtt, material::PURE_RED);

    //world_element.add_sphere( 0.0, -100.5, -1.0, 100.0, MATERIAL_GROUND);
    //world_element.add_sphere( 0.0,    0.0, -1.0,   0.5,   MATERIAL_CENTER);
    //world_element.add_sphere(-1.0,    0.0, -1.0,   0.5,   MATERIAL_LEFT);
    //world_element.add_sphere(-1.0,    0.0, -1.0, -0.45, MATERIAL_LEFT);
    //world_element.add_sphere( 1.0,    0.0, -1.0,   0.5,   MATERIAL_RIGHT);

    return world_element;
}