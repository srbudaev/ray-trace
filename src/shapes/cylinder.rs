use std::rc::Rc;

use crate::core::hittable::{HitRecord, Hittable};
use crate::core::material::Material;
use crate::math::ray::Ray;
use crate::math::vec3::{Point3, Vec3};

pub struct Cylinder {
    base: Point3,      // Center of the bottom circle
    axis: Vec3,        // Direction vector (pointing from base to top)
    radius: f64,
    height: f64,
    mat: Rc<dyn Material>,
}

impl Cylinder {
    pub fn new(base: Point3, axis: Vec3, radius: f64, height: f64, mat: Rc<dyn Material>) -> Self {
        Self {
            base,
            axis: axis.unit_vector(),
            radius,
            height,
            mat,
        }
    }

    /// Check if a point is within the cylinder's height bounds
    fn is_within_height(&self, point: Point3) -> bool {
        let base_to_point = point - self.base;
        let projection = base_to_point.dot(&self.axis);
        projection >= 0.0 && projection <= self.height
    }

    /// Get the normal at a given point on the cylinder
    fn get_normal(&self, point: Point3) -> Vec3 {
        let base_to_point = point - self.base;
        let projection = base_to_point.dot(&self.axis);
        
        const EPS: f64 = 1e-4;
        
        // Check if we hit the bottom cap
        if projection.abs() < EPS {
            return -self.axis;
        }
        
        // Check if we hit the top cap
        if (projection - self.height).abs() < EPS {
            return self.axis;
        }
        
        // Hit the curved side: normal is perpendicular to axis
        let axis_point = self.base + projection * self.axis;
        (point - axis_point).unit_vector()
    }

    /// Check for intersection with the top and bottom circular caps
    fn hit_caps(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<(f64, Point3, Vec3)> {
        let d_dot_axis = r.direction().dot(&self.axis);
        
        // If ray is parallel to caps, no intersection
        if d_dot_axis.abs() < 1e-8 {
            return None;
        }
        
        let mut closest_t = t_max;
        let mut hit_found = false;
        let mut hit_normal = Vec3::default();
        let mut hit_point = Point3::default();
        
        // Check bottom cap
        let oc_bottom = r.origin() - self.base;
        let t_bottom = -oc_bottom.dot(&self.axis) / d_dot_axis;
        
        if t_bottom >= t_min && t_bottom < closest_t {
            let point = r.at(t_bottom);
            let base_to_point = point - self.base;
            let radial_dist_sq = (base_to_point - base_to_point.dot(&self.axis) * self.axis).length_squared();
            
            if radial_dist_sq <= self.radius * self.radius {
                closest_t = t_bottom;
                hit_point = point;
                hit_normal = -self.axis;
                hit_found = true;
            }
        }
        
        // Check top cap
        let top_center = self.base + self.height * self.axis;
        let oc_top = r.origin() - top_center;
        let t_top = -oc_top.dot(&self.axis) / d_dot_axis;
        
        if t_top >= t_min && t_top < closest_t {
            let point = r.at(t_top);
            let top_to_point = point - top_center;
            let radial_dist_sq = (top_to_point - top_to_point.dot(&self.axis) * self.axis).length_squared();
            
            if radial_dist_sq <= self.radius * self.radius {
                closest_t = t_top;
                hit_point = point;
                hit_normal = self.axis;
                hit_found = true;
            }
        }
        
        if hit_found {
            Some((closest_t, hit_point, hit_normal))
        } else {
            None
        }
    }
}

impl Hittable for Cylinder {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.base;
        
        // Project ray direction and origin offset onto the plane perpendicular to axis
        let d_dot_axis = r.direction().dot(&self.axis);
        let oc_dot_axis = oc.dot(&self.axis);
        
        let d_perp = r.direction() - d_dot_axis * self.axis;
        let oc_perp = oc - oc_dot_axis * self.axis;
        
        // Solve quadratic equation for intersection with infinite cylinder
        let a = d_perp.dot(&d_perp);
        
        // Avoid division by zero
        if a < 1e-8 {
            // Ray is parallel to axis, only check caps
            if let Some((t, point, normal)) = self.hit_caps(r, t_min, t_max) {
                rec.t = t;
                rec.p = point;
                rec.mat = Some(self.mat.clone());
                rec.set_face_normal(r, normal);
                return true;
            }
            return false;
        }
        
        let b = 2.0 * oc_perp.dot(&d_perp);
        let c = oc_perp.dot(&oc_perp) - self.radius * self.radius;
        
        let discriminant = b * b - 4.0 * a * c;
        
        let mut best_t = t_max;
        let mut hit_found = false;
        let mut hit_point = Point3::default();
        let mut hit_normal = Vec3::default();
        
        // Check side intersection if discriminant is valid
        if discriminant >= 0.0 {
            let sqrt_d = discriminant.sqrt();
            let mut root = (-b - sqrt_d) / (2.0 * a);
            
            // Check first root
            if root >= t_min && root <= t_max {
                let point = r.at(root);
                if self.is_within_height(point) {
                    best_t = root;
                    hit_point = point;
                    hit_normal = self.get_normal(point);
                    hit_found = true;
                }
            }
            
            // Check second root if first didn't work or if it's closer
            root = (-b + sqrt_d) / (2.0 * a);
            if root >= t_min && root <= t_max && root < best_t {
                let point = r.at(root);
                if self.is_within_height(point) {
                    best_t = root;
                    hit_point = point;
                    hit_normal = self.get_normal(point);
                    hit_found = true;
                }
            }
        }
        
        // Check caps and compare with side intersection
        if let Some((t, point, normal)) = self.hit_caps(r, t_min, t_max) {
            if t < best_t {
                best_t = t;
                hit_point = point;
                hit_normal = normal;
                hit_found = true;
            }
        }
        
        if hit_found {
            rec.t = best_t;
            rec.p = hit_point;
            rec.mat = Some(self.mat.clone());
            rec.set_face_normal(r, hit_normal);
            return true;
        }
        
        false
    }
}


