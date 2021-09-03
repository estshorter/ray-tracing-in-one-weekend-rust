use crate::aabb::*;
use crate::hittable::*;
use crate::material::*;
use crate::ray::Ray;
use crate::sphere::get_sphere_uv;
use crate::vec3::*;
pub struct MovingSphere<M: Material> {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    material: M,
}

impl<M: Material> MovingSphere<M> {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: M,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f64) -> Vec3 {
        &self.center0
            + &(((time - self.time0) / (self.time1 - self.time0)) * (&self.center1 - &self.center0))
    }
}

impl<M: Material> Hittable for MovingSphere<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let center = self.center(r.time());
        let oc = r.origin() - &center;
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
        let outward_normal = (&p - &center) / self.radius;
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

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        let radius = Vec3::new(self.radius, self.radius, self.radius);
        let min0 = self.center(t0) - &radius;
        let max0 = self.center(t0) + &radius;
        let min1 = self.center(t1) - &radius;
        let max1 = self.center(t1) + &radius;
        let aabb0 = AABB::new(min0, max0);
        let aabb1 = AABB::new(min1, max1);
        Some(surrounding_box(&aabb0, &aabb1))
    }
}
