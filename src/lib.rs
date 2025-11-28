pub mod math;
pub mod core;
pub mod shapes;
pub mod scene;
pub mod scenes;
pub mod cli;
pub mod ppm;

pub use math::{Vec3, Point3, Ray};
pub use core::{Color, Camera, PointLight, Hittable, HitRecord, HittableList};
pub use shapes::{Sphere, Plane, Cuboid};
pub use scene::render::ray_color;
pub use scenes::random_scene;
pub use cli::Config;
pub use ppm::write_color;

