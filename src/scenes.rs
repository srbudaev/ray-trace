use std::rc::Rc;
use crate::core::{HittableList, Color, common};
use crate::core::material::{Material, NumberType, LambertianNoise, Metal, Striped, Lambertian};
use crate::shapes::{Sphere, Plane, Cylinder, Cuboid};
use crate::math::vec3::{Point3, Vec3};

pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    // Ground material with a subtle Perlin noise texture (marble-like)
    // Increased scale for finer (smaller) features
    let ground_material = Rc::new(LambertianNoise::new(Color::new(0.099, 0.172, 0.095), 150.0)); //original: 0.5, 0.5, 0.5
    world.add(Box::new(Plane::new(
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

                world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));

                count_balls += 1;
            }
        }
    }
    world
}

// ========== AUDIT SCENES ==========

/// Scene 1: Only a sphere
pub fn scene_sphere() -> HittableList {
    let mut world = HittableList::new();

    // Ground plane
    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Plane::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        ground_material,
    )));

    // Single sphere
    let sphere_material = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3))); // Red sphere
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),  // Center above ground
        0.5,                          // Radius
        sphere_material,
    )));

    world
}

/// Scene 2: Plane + Cube with lower brightness (brightness controlled in main.rs)
pub fn scene_plane_cube() -> HittableList {
    let mut world = HittableList::new();

    // Ground plane
    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Plane::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        ground_material,
    )));

    // Cube
    let cube_material = Rc::new(Lambertian::new(Color::new(0.4, 0.6, 0.8))); // Blue cube
    world.add(Box::new(Cuboid::new(
        Point3::new(-0.5, 0.0, -0.5),  // Min corner
        Point3::new(0.5, 1.0, 0.5),    // Max corner
        cube_material,
    )));

    world
}

/// Scene 3: All objects (sphere, cube, cylinder, plane)
pub fn scene_all_objects() -> HittableList {
    let mut world = HittableList::new();

    // Ground plane
    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Plane::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        ground_material,
    )));

    // Sphere - слева, ближе к камере
    let sphere_material = Rc::new(Lambertian::new(Color::new(0.8, 0.2, 0.2))); // Red
    world.add(Box::new(Sphere::new(
        Point3::new(-1.5, 0.6, 0.0),  // Выше и ближе
        0.6,                           // Больше размер
        sphere_material,
    )));

    // Cube - в центре
    let cube_material = Rc::new(Lambertian::new(Color::new(0.2, 0.2, 0.8))); // Blue
    world.add(Box::new(Cuboid::new(
        Point3::new(-0.6, 0.0, -0.6),  // Центр смещен немного назад
        Point3::new(0.6, 1.2, 0.6),    // Выше куб
        cube_material,
    )));

    // Cylinder - справа, дальше
    let cylinder_material = Rc::new(Lambertian::new(Color::new(0.2, 0.8, 0.2))); // Green
    world.add(Box::new(Cylinder::new(
        Point3::new(1.5, 0.0, 0.0),    // Справа
        Vec3::new(0.0, 1.0, 0.0),
        0.6,                            // Больше радиус
        1.8,                            // Выше
        cylinder_material,
    )));

    world
}

/// Scene 4: Same as scene 3, but camera position will be different (handled in main.rs)
pub fn scene_all_objects_alt_camera() -> HittableList {
    // Same scene as scene_all_objects, camera position differs in main.rs
    scene_all_objects()
}

