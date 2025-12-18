pub mod camera;
pub mod light;
pub mod hittable;
pub mod hittable_list;
pub mod common;
pub mod material;
use crate::math::vec3::Vec3;

pub type Color = Vec3;
pub use camera::Camera;
pub use light::{PointLight, compute_light};
pub use hittable::{HitRecord, Hittable};
pub use hittable_list::HittableList;
pub use common::*;
pub use material::{Material, NumberType, Lambertian, Solid, LambertianNoise, Striped};

