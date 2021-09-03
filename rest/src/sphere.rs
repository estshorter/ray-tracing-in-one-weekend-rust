use crate::aabb::AABB;
use crate::hittable::*;
use crate::material::*;
use crate::onb::*;
use crate::ray::Ray;
use crate::vec3::*;
use rand::Rng;
use std::f64;

pub fn get_sphere_uv(p: &Vec3) -> (f64, f64) {
    let phi = p.z().atan2(p.x());
    let theta = p.y().asin();
    let u = 1.0 - (phi + f64::consts::PI) / (2.0 * f64::consts::PI);
    let v = (theta + f64::consts::FRAC_PI_2) / f64::consts::PI;
    (u, v)
}

fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3 {
    let mut rng = rand::thread_rng();
    let r1 = rng.gen::<f64>();
    let r2 = rng.gen::<f64>();
    let z = 1.0 + r2 * ((1.0 - radius.powi(2) / distance_squared).sqrt() - 1.0);
    let phi = 2.0 * f64::consts::PI * r1;
    let x = phi.cos() * (1.0 - z.powi(2)).sqrt();
    let y = phi.sin() * (1.0 - z.powi(2)).sqrt();
    Vec3::new(x, y, z)
}

#[derive(Clone)]
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
        let (u, v) = get_sphere_uv(&outward_normal);
        Some(HitRecord::new(
            p,
            &self.material,
            t,
            u,
            v,
            r,
            outward_normal,
        ))
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        let radius = Vec3::new(self.radius, self.radius, self.radius);
        let min = &self.center - &radius;
        let max = &self.center + &radius;
        Some(AABB { min, max })
    }

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        if let Some(_hit) = self.hit(&Ray::new(o.clone(), v.clone(), 0.0), 0.001, f64::MAX) {
            let cos_theta_max =
                (1.0 - self.radius.powi(2) / (self.center - o).length_squared()).sqrt();
            let solid_angle = 2.0 * f64::consts::PI * (1.0 - cos_theta_max);
            1.0 / solid_angle
        } else {
            0.0
        }
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let direction = self.center - o;
        let distance_squared = direction.length_squared();
        let uvw = ONB::build_from_w(&direction);
        uvw.local(&random_to_sphere(self.radius, distance_squared))
    }
}
