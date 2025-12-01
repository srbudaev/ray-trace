use std::rc::Rc;
use crate::core::{Color, common};
use crate::core::material::{Material, NumberType, LambertianNoise, Metal, Striped, Lambertian};
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

    // Add a small cube to the scene
   // let box_material = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
   // world.add(Box::new(Cuboid::new(
   //     Point3::new(-1.5, 0.0, 0.5),
   //     Point3::new(-1.1, 0.4, 0.9),
    //    box_material,
   // )));

    let mut count_balls = 0;
    let nbr_of_balls = 16;
    for a in -1..3 {
        for b in -1..4 {
            if count_balls <= nbr_of_balls {
                let center = Point3::new(
                    a as f64 + 0.7 * common::random_double(),
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

                let sphere_material: Rc<dyn Material> = if color.is_spots {
                    Rc::new(Striped::new(color.color, fuzz, center, number_type))
                } else {
                    Rc::new(Metal::new(color.color, fuzz, center, spot_dir, number_type))
                };

                scene.add_object(Box::new(Sphere::new(center, 0.2, sphere_material)));

                count_balls += 1;
            }
        }
    }
    
    // Set camera for random scene
    let lookat = Point3::new(0.85, 0.2, 1.35);
    scene.set_camera(
        Point3::new(13.0, 2.0, 3.0),
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
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
        0.75,
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
        Point3::new(5.0, 5.0, 5.0),
        Point3::new(0.0, 0.5, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        0.02,
        6.0,
    );

    // Add light with lower brightness for scene 2
    scene.add_light(crate::core::PointLight::new(
        Point3::new(0.0, 5.0, 4.0),
        Color::new(1.0, 0.85, 0.6),
        0.3,  // Lower brightness
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

    // Sphere - слева, ближе к камере
    let sphere_material = Rc::new(Lambertian::new(Color::new(0.8, 0.2, 0.2))); // Red
    scene.add_object(Box::new(Sphere::new(
        Point3::new(-1.5, 0.6, 0.0),  // Выше и ближе
        0.6,                           // Больше размер
        sphere_material,
    )));

    // Cube - в центре
    let cube_material = Rc::new(Lambertian::new(Color::new(0.2, 0.2, 0.8))); // Blue
    scene.add_object(Box::new(Cuboid::new(
        Point3::new(-0.6, 0.0, -0.6),  // Центр смещен немного назад
        Point3::new(0.6, 1.2, 0.6),    // Выше куб
        cube_material,
    )));

    // Cylinder - справа, дальше
    let cylinder_material = Rc::new(Lambertian::new(Color::new(0.2, 0.8, 0.2))); // Green
    scene.add_object(Box::new(Cylinder::new(
        Point3::new(1.5, 0.0, 0.0),    // Справа
        Vec3::new(0.0, 1.0, 0.0),
        0.6,                            // Больше радиус
        1.8,                            // Выше
        cylinder_material,
    )));

    // Set camera - wider FOV for all objects
    scene.set_camera(
        Point3::new(4.0, 2.5, 4.0),
        Point3::new(0.0, 0.6, 0.0),
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

