use std::rc::Rc;
use crate::core::{Color, common};
use crate::core::material::{Material, NumberType, LambertianNoise, Solid, Striped, Lambertian};
use crate::shapes::{Sphere, Plane, Cylinder, Cuboid};
use crate::math::vec3::{Point3, Vec3};
use crate::scene::Scene;

pub fn random_scene() -> Scene {
    let mut scene = Scene::new();

    // Ground material with a subtle Perlin noise texture (marble-like)
    // Increased scale for finer (smaller) features
    let ground_material = Rc::new(LambertianNoise::new(Color::new(0.099, 0.172, 0.095), 150.0)); //original: 0.5, 0.5, 0.5
    scene.add_object(Box::new(Plane::new(
        Point3::new(0.0, 0.0, 0.0),    // A point on the plane (the origin)
        Vec3::new(0.0, 1.0, 0.0),      // The normal vector (pointing straight up)
        ground_material,
    )));

    let mut count_balls = 0;
    let nbr_of_balls = 16;
    for a in 0..6 {
        for b in 0..3 {
            if count_balls <= nbr_of_balls {
                let center = Point3::new(
                    a as f64 -0.5 + 0.7 * common::random_double(),
                    0.2,
                    b as f64 + 0.7 * common::random_double(),
                );

                let color = common::random_billiard_color(count_balls);

                let fuzz = 0.01;
                let spot_dir = crate::math::vec3::random_unit_vector();

                let number_type = if count_balls % 2 == 0 {
                    NumberType::Line
                } else {
                    NumberType::Circle
                };

                let billiard_material: Rc<dyn Material> = if color.is_spots {
                    Rc::new(Striped::new(color.color, fuzz, center, number_type))
                } else {
                    Rc::new(Solid::new(color.color, fuzz, center, spot_dir, number_type))
                };

                scene.add_object(Box::new(Sphere::new(center, 0.2, billiard_material)));

                count_balls += 1;
            }
        }
    }
    
    // Set camera for random scene
    let lookat = Point3::new(2.0, 0.0, 1.5);
    scene.set_camera(
        Point3::new(8.0, 0.85, -0.5),
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        0.04,
        3.0,
    );
    
    // Add light
    scene.add_light(crate::core::PointLight::new(
        Point3::new(0.0, 5.0, 4.0),
        Color::new(1.0, 0.85, 0.6),
        0.75,
    ));
    
        // Add light
    scene.add_light(crate::core::PointLight::new(
        Point3::new(2.0, 3.0, 10.0),
        Color::new(1.0, 0.85, 0.6),
        0.75,
    ));
    scene
}

// ========== AUDIT SCENES ==========

/// Scene 1: Only a sphere
pub fn scene_sphere() -> Scene {
    let mut scene = Scene::new();

    // Ground plane
    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    scene.add_object(Box::new(Plane::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        ground_material,
    )));

    // Single sphere
    let sphere_material = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3))); // Red sphere
    scene.add_object(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),  // Center above ground
        0.5,                          // Radius
        sphere_material,
    )));

    // Set camera
    scene.set_camera(
        Point3::new(3.0, 2.0, 3.0),
        Point3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        0.02,
        6.0,
    );

    // Add light
    scene.add_light(crate::core::PointLight::new(
        Point3::new(0.0, 5.0, 4.0),
        Color::new(1.0, 0.85, 0.6),
        1.75,
    ));

    scene
}

/// Scene 2: Plane + Cube with lower brightness
pub fn scene_plane_cube() -> Scene {
    let mut scene = Scene::new();

    // Ground plane
    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    scene.add_object(Box::new(Plane::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        ground_material,
    )));

    // Cube
    let cube_material = Rc::new(Lambertian::new(Color::new(0.4, 0.6, 0.8))); // Blue cube
    scene.add_object(Box::new(Cuboid::new(
        Point3::new(-0.5, 0.0, -0.5),  // Min corner
        Point3::new(0.5, 1.0, 0.5),    // Max corner
        cube_material,
    )));

    // Set camera
    scene.set_camera(
        Point3::new(4.0, 2.0, 4.0),
        Point3::new(0.0, 0.5, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        0.03,
        4.0,
    );

    // Add light with lower brightness for scene 2
    scene.add_light(crate::core::PointLight::new(
        Point3::new(0.0, 5.0, 4.0),
        Color::new(2.0, 1.85, 1.6),
        0.75,  // Lower brightness
    ));

    scene
}

/// Scene 3: All objects (sphere, cube, cylinder, plane)
pub fn scene_all_objects() -> Scene {
    let mut scene = Scene::new();

    // Ground plane
    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    scene.add_object(Box::new(Plane::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        ground_material,
    )));

    // Sphere 
    let sphere_material = Rc::new(Lambertian::new(Color::new(0.8, 0.2, 0.2))); // Red
    scene.add_object(Box::new(Sphere::new(
        Point3::new(-1.5, 0.6, 0.0),  
        0.6,                          
        sphere_material,
    )));

    // Cube 
    let cube_material = Rc::new(Lambertian::new(Color::new(0.2, 0.2, 0.8))); // Blue
    scene.add_object(Box::new(Cuboid::new(
        Point3::new(-0.6, 0.0, -0.6),  
        Point3::new(0.6, 1.2, 0.6),    
        cube_material,
    )));

    // Cylinder 
    let cylinder_material = Rc::new(Lambertian::new(Color::new(0.2, 0.8, 0.2))); // Green
    scene.add_object(Box::new(Cylinder::new(
        Point3::new(1.5, 0.0, 0.0),  
        Vec3::new(0.0, 1.0, 0.0),
        0.6,                            
        1.8,                          
        cylinder_material,
    )));

    // Set camera - wider FOV for all objects
    scene.set_camera(
        Point3::new(4.0, 2.5, 4.0),
        Point3::new(0.5, 0.6, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        30.0,  // Wider FOV
        0.02,
        6.0,
    );

    // Add light
    scene.add_light(crate::core::PointLight::new(
        Point3::new(0.0, 5.0, 4.0),
        Color::new(1.0, 0.85, 0.6),
        0.75,
    ));

    scene
}

/// Scene 4: Same as scene 3, but camera position is different
pub fn scene_all_objects_alt_camera() -> Scene {
    let mut scene = scene_all_objects();
    
    // Change camera position - different angle
    scene.set_camera(
        Point3::new(-4.0, 2.5, 4.0),  // Camera from the other side
        Point3::new(0.0, 0.6, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        30.0,  // Wider FOV
        0.02,
        6.0,
    );
    
    scene
}

/// Scene 5: Custom scene with parameters from CLI
pub fn scene_custom_with_params(
    spheres: &[crate::cli::SphereSpec],
    camera_pos: Option<(f64, f64, f64)>,
    camera_lookat: Option<(f64, f64, f64)>,
) -> Scene {
    let mut scene = Scene::new();

    // Ground plane
    let ground_material = Rc::new(Lambertian::new(Color::new(0.4, 0.5, 0.1)));
    scene.add_object(Box::new(Plane::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        ground_material,
    )));

    // Add spheres from command line arguments
    for sphere_spec in spheres {
        let material: Rc<dyn crate::core::Material> = match sphere_spec.material {
            crate::cli::MaterialType::Lambertian => {
                Rc::new(Lambertian::new(Color::new(
                    sphere_spec.r,
                    sphere_spec.g,
                    sphere_spec.b,
                )))
            }
            crate::cli::MaterialType::Glass { index_of_refraction } => {
                Rc::new(crate::core::material::Dielectric::new(
                    index_of_refraction,
                    Color::new(sphere_spec.r, sphere_spec.g, sphere_spec.b),
                ))
            }
            crate::cli::MaterialType::Translucent { opacity } => {
                Rc::new(crate::core::material::Translucent::new(
                    Color::new(sphere_spec.r, sphere_spec.g, sphere_spec.b),
                    opacity,
                ))
            }
        };
        scene.add_object(Box::new(Sphere::new(
            Point3::new(sphere_spec.x, sphere_spec.y, sphere_spec.z),
            sphere_spec.radius,
            material,
        )));
    }

    // Set camera with provided or default values
    let cam_pos = camera_pos.unwrap_or((5.0, 1.0, 6.0));
    let cam_lookat = camera_lookat.unwrap_or((0.0, 0.8, 0.0));
    
    scene.set_camera(
        Point3::new(cam_pos.0, cam_pos.1, cam_pos.2),
        Point3::new(cam_lookat.0, cam_lookat.1, cam_lookat.2),
        Vec3::new(0.0, 1.0, 0.0),
        35.0,
        0.02,
        7.5,
    );

    // Add lights
    scene.add_light(crate::core::PointLight::new(
        Point3::new(0.0, 5.0, 4.0),
        Color::new(1.0, 0.85, 0.6),
        0.75,
    ));

    scene
}

/// Scene 5: Custom scene - wrapper for backwards compatibility
pub fn scene_custom() -> Scene {
    scene_custom_with_params(&[], None, None)
}

