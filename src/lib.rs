pub mod math;
pub mod core;
pub mod shapes;
pub mod scene;
pub mod scenes;
pub mod cli;
pub mod ppm;

pub use math::{Vec3, Point3, Ray};
pub use core::{Color, Camera, PointLight, Hittable, HitRecord, HittableList};
pub use core::{Lambertian, Solid, LambertianNoise, Striped, Material};
pub use shapes::{Sphere, Plane, Cuboid, Cylinder};
pub use scene::render::ray_color;
pub use scene::{Scene, RenderSettings};
pub use scenes::{random_scene, scene_sphere, scene_plane_cube, scene_all_objects, scene_all_objects_alt_camera};
pub use cli::Config;
pub use ppm::write_color;

