use std::rc::Rc;
use crate::core::{HittableList, Color, common};
use crate::core::material::{Material, NumberType, LambertianNoise, Metal, Striped};
use crate::shapes::{Sphere, Plane};
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

