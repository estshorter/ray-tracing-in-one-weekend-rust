use crate::aabb::AABB;
use crate::hittable::*;
use crate::ray::Ray;
use crate::vec3::*;

pub struct Translate<H: Hittable> {
    hitable: H,
    offset: Vec3,
}

impl<H: Hittable> Translate<H> {
    pub fn new(hitable: H, offset: Vec3) -> Self {
        Translate { hitable, offset }
    }
}

impl<H: Hittable> Hittable for Translate<H> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.origin() - &self.offset, *ray.direction(), ray.time());
        self.hitable.hit(&moved_ray, t_min, t_max).map(|mut hit| {
            hit.p += self.offset;
            hit
        })
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.hitable.bounding_box(t0, t1).map(|mut b| {
            b.min += self.offset;
            b.max += self.offset;
            b
        })
    }
}
