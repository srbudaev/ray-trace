use crate::core::{Color, Hittable, HitRecord, PointLight, common};
use crate::math::ray::Ray;

pub fn ray_color(r: &Ray, world: &dyn Hittable, lights: &[PointLight], depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, common::INFINITY, &mut rec) {
        let view_dir = (-r.direction()).unit_vector();

        // --- 1. Calculate Indirect (scattered) light ---
        let mut attenuation = Color::new(0.0, 0.0, 0.0);
        let mut scattered = Ray::default();
        let indirect_light = if let Some(mat) = &rec.mat {
            if mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                attenuation * ray_color(&scattered, world, lights, depth - 1)
            } else {
                Color::new(0.0, 0.0, 0.0)
            }
        } else {
            Color::new(0.0, 0.0, 0.0)
        };

        // --- 2. Calculate Direct light from all light sources ---
        let mut direct_light = Color::new(0.0, 0.0, 0.0);
        for light in lights.iter() {
            const SAMPLES: i32 = 4; // Use a constant for shadow samples
            let mut light_contribution = Color::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES {
                let light_radius = 1.4;
                let jitter = crate::math::vec3::random_in_unit_sphere() * light_radius;
                let sample_pos = light.position + jitter;
                let to_light = sample_pos - rec.p;
                let light_dist = to_light.length();
                let light_dir = to_light / light_dist;
                let shadow_ray = Ray::new(rec.p + rec.normal * 0.001, light_dir);

                if !world.hit(&shadow_ray, 0.001, light_dist - 0.001, &mut HitRecord::new()) {
                    // If the material pattern is black, only add specular highlights.
                    if attenuation.near_zero() {
                        light_contribution += crate::core::light::compute_light(rec.p, rec.normal, view_dir, Color::new(0.0, 0.0, 0.0), light).1; // Specular only
                    } else {
                        let (diffuse, specular) = crate::core::light::compute_light(rec.p, rec.normal, view_dir, attenuation, light);
                        light_contribution += diffuse + specular;
                    }
                }
            }
            direct_light += light_contribution / SAMPLES as f64;
        }

        // --- 3. Combine and return final color ---
        return indirect_light + direct_light;
    }

    //Background gradient
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

