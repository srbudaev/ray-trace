pub mod color;
pub mod camera;
pub mod light;
pub mod hittable;
pub mod hittable_list;
pub mod common;
pub mod material;

pub use color::Color;
pub use camera::Camera;
pub use light::{PointLight, compute_light};
pub use hittable::{HitRecord, Hittable};
pub use hittable_list::HittableList;
pub use common::*;
pub use material::{Material, NumberType, Lambertian, Metal, LambertianNoise, Striped};

