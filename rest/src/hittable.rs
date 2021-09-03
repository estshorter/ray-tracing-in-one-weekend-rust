use crate::aabb::*;
use crate::material::*;
use crate::ray::Ray;
use crate::vec3::*;
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        p: Point3,
        material: &'a dyn Material,
        t: f64,
        u: f64,
        v: f64,
        r: &Ray,
        outward_normal: Vec3,
    ) -> Self {
        let (front_face, normal) = calc_face_normal(r, outward_normal);
        Self {
            p,
            normal,
            material,
            t,
            u,
            v,
            front_face,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
    fn pdf_value(&self, _o: &Vec3, _v: &Vec3) -> f64 {
        0.0
    }
    fn random(&self, _o: &Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

fn calc_face_normal(r: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
    let front_face = dot(r.direction(), &outward_normal) < 0.0;
    let normal = if front_face {
        outward_normal
    } else {
        -outward_normal
    };
    (front_face, normal)
}

pub struct FlipNormals<H: Hittable> {
    hittable: H,
}

impl<H: Hittable> FlipNormals<H> {
    pub fn new(hitable: H) -> Self {
        FlipNormals { hittable: hitable }
    }
}

impl<H: Hittable> Hittable for FlipNormals<H> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(mut rec) = self.hittable.hit(&ray, t_min, t_max) {
            rec.front_face = !rec.front_face;
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.hittable.bounding_box(t0, t1)
    }
}
