use crate::hittable::*;
use crate::material::*;
use crate::ray::Ray;
use crate::vec3::*;

pub struct Sphere<M: Material> {
    pub center: Vec3,
    pub radius: f64,
    material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vec3, radius: f64, material: M) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - &self.center;
        let a = r.direction().length_squared();
        let half_b = dot(&oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = (&p - &self.center) / self.radius;
        Some(HitRecord::new(p, t, &self.material, r, outward_normal))
    }
}
