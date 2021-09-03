use crate::aabb::AABB;
use crate::hittable::*;
use crate::material::Isotropic;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec3::Vec3;
use rand::Rng;
use std::f64;

pub struct ConstantMedium<H: Hittable, T: Texture> {
    boundary: H,
    neg_inv_density: f64,
    phase_function: Isotropic<T>,
}

impl<H: Hittable, T: Texture> ConstantMedium<H, T> {
    pub fn new(boundary: H, density: f64, texture: T) -> Self {
        ConstantMedium {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Isotropic::new(texture),
        }
    }
}

impl<H: Hittable, T: Texture> Hittable for ConstantMedium<H, T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rng = rand::thread_rng();
        if let Some(mut hit1) = self.boundary.hit(&ray, -f64::MAX, f64::MAX) {
            if let Some(mut hit2) = self.boundary.hit(&ray, hit1.t + 0.0001, f64::MAX) {
                if hit1.t < t_min {
                    hit1.t = t_min
                }
                if hit2.t > t_max {
                    hit2.t = t_max
                }
                if hit1.t < hit2.t {
                    let ray_length = ray.direction().length();
                    let distance_inside_boundary = (hit2.t - hit1.t) * ray_length;
                    let hit_distance = self.neg_inv_density * rng.gen::<f64>().ln();
                    if hit_distance < distance_inside_boundary {
                        let t = hit1.t + hit_distance / ray_length;
                        return Some(HitRecord {
                            p: ray.at(t),
                            normal: Vec3::new(1.0, 0.0, 0.0), // arbitrary
                            material: &self.phase_function,
                            t,
                            u: 0.0,
                            v: 0.0,
                            front_face: true,
                        });
                    }
                }
            }
        }
        None
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.boundary.bounding_box(t0, t1)
    }
}
