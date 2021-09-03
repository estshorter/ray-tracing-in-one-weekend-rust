use crate::aabb::AABB;
use crate::hittable::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::*;
use rand::Rng;

#[derive(Clone)]
pub enum Plane {
    YZ,
    ZX,
    XY,
}

#[derive(Clone)]
pub struct AARect<M: Material> {
    plane: Plane,
    a0: f64,
    a1: f64,
    b0: f64,
    b1: f64,
    k: f64,
    material: M,
}

fn get_axis(plane: &Plane) -> (usize, usize, usize) {
    match plane {
        Plane::YZ => (0, 1, 2),
        Plane::ZX => (1, 2, 0),
        Plane::XY => (2, 0, 1),
    }
}

impl<M: Material> AARect<M> {
    pub fn new(plane: Plane, a0: f64, a1: f64, b0: f64, b1: f64, k: f64, material: M) -> Self {
        AARect {
            plane,
            a0,
            a1,
            b0,
            b1,
            k,
            material,
        }
    }
}

impl<M: Material> Hittable for AARect<M> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let (k_axis, a_axis, b_axis) = match &self.plane {
            Plane::YZ => (0, 1, 2),
            Plane::ZX => (1, 2, 0),
            Plane::XY => (2, 0, 1),
        };
        let t = (self.k - ray.origin()[k_axis]) / ray.direction()[k_axis];
        if t < t_min || t > t_max {
            return None;
        }

        let a = ray.origin()[a_axis] + t * ray.direction()[a_axis];
        let b = ray.origin()[b_axis] + t * ray.direction()[b_axis];
        if a < self.a0 || a > self.a1 || b < self.b0 || b > self.b1 {
            None
        } else {
            let u = (a - self.a0) / (self.a1 - self.a0);
            let v = (b - self.b0) / (self.b1 - self.b0);
            let p = ray.at(t);
            let mut normal = Vec3::default();
            normal[k_axis] = 1.0;
            let outward_normal = match &self.plane {
                Plane::YZ => Vec3::new(1.0, 0.0, 0.0),
                Plane::ZX => Vec3::new(0.0, 1.0, 0.0),
                Plane::XY => Vec3::new(0.0, 0.0, 1.0),
            };

            Some(HitRecord::new(
                p,
                &self.material,
                t,
                u,
                v,
                ray,
                outward_normal,
            ))
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        let min = Vec3::new(self.a0, self.b0, self.k - 0.0001);
        let max = Vec3::new(self.a1, self.b1, self.k + 0.0001);
        Some(AABB { min, max })
    }

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        if let Some(hit) = self.hit(&Ray::new(o.clone(), v.clone(), 0.0), 0.001, f64::MAX) {
            let area = (self.a1 - self.a0) * (self.b1 - self.b0);
            let distance_squared = hit.t.powi(2) * v.length_squared();
            let cosine = f64::abs(dot(&v, &hit.normal).abs() / v.length());
            if true {
                distance_squared / (cosine * area)
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let mut rng = rand::thread_rng();
        let (k_axis, a_axis, b_axis) = get_axis(&self.plane);
        let mut random_point = Vec3::default();
        random_point[a_axis] = rng.gen_range(self.a0..=self.a1);
        random_point[b_axis] = rng.gen_range(self.b0..=self.b1);
        random_point[k_axis] = self.k;
        random_point - o
    }
}
