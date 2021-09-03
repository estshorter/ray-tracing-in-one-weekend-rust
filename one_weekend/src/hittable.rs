use crate::material::*;
use crate::ray::Ray;
use crate::vec3::*;

pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub t: f64,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        p: Point3,
        t: f64,
        material: &'a dyn Material,
        r: &Ray,
        outward_normal: Vec3,
    ) -> Self {
        let (front_face, normal) = calc_face_normal(r, outward_normal);
        Self {
            p,
            normal,
            material,
            t,
            front_face,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
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
