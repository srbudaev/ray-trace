use crate::core::Color;
use crate::core::hittable::HitRecord;
use crate::math::{Ray, Vec3, Point3};
use crate::math::vec3;

pub trait Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

#[derive(Clone, Copy)]
pub enum NumberType {
    Line,
    Circle,
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let scatter_direction = rec.normal + vec3::random_unit_vector();
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct LambertianNoise {
    base_color: Color,
    scale: f64,
}

impl LambertianNoise {
    pub fn new(base_color: Color, scale: f64) -> Self {
        Self { base_color, scale }
    }
}

impl Material for LambertianNoise {
    fn scatter(&self, _r: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let scatter_direction = rec.normal + vec3::random_unit_vector();
        *scattered = Ray::new(rec.p, scatter_direction);
        // Simple noise approximation using position
        let noise = (rec.p.x() * self.scale).sin() * (rec.p.z() * self.scale).cos();
        *attenuation = self.base_color * (0.5 + 0.5 * noise);
        true
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
    center: Point3,
    spot_dir: Vec3,
    number_type: NumberType,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64, center: Point3, spot_dir: Vec3, number_type: NumberType) -> Self {
        Self {
            albedo,
            fuzz: f64::min(fuzz, 1.0),
            center,
            spot_dir,
            number_type,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = vec3::reflect(r.direction().unit_vector(), rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.fuzz * vec3::random_in_unit_sphere());
        *attenuation = self.albedo;
        true
    }
}

pub struct Striped {
    albedo: Color,
    fuzz: f64,
    center: Point3,
    number_type: NumberType,
}

impl Striped {
    pub fn new(albedo: Color, fuzz: f64, center: Point3, number_type: NumberType) -> Self {
        Self {
            albedo,
            fuzz: f64::min(fuzz, 1.0),
            center,
            number_type,
        }
    }
}

impl Material for Striped {
    fn scatter(&self, r: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = vec3::reflect(r.direction().unit_vector(), rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.fuzz * vec3::random_in_unit_sphere());
        *attenuation = self.albedo;
        true
    }
}

