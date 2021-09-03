use crate::aabb::*;
use crate::hittable::*;
use crate::ray::Ray;
use crate::vec3::*;
use rand::seq::SliceRandom;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn push(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut rec_opt: Option<HitRecord> = None;

        self.objects.iter().for_each(|object| {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                rec_opt = Some(rec);
            }
        });
        return rec_opt;
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        match self.objects.first() {
            Some(first) => match first.bounding_box(t0, t1) {
                Some(bbox) => self.objects.iter().skip(1).try_fold(bbox, |acc, hittable| {
                    match hittable.bounding_box(t0, t1) {
                        Some(bbox) => Some(surrounding_box(&acc, &bbox)),
                        _ => None,
                    }
                }),
                _ => None,
            },
            _ => None,
        }
    }

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        self.objects.iter().map(|h| h.pdf_value(o, v)).sum::<f64>() / self.objects.len() as f64
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        self.objects
            .choose(&mut rand::thread_rng())
            .unwrap()
            .random(o)
    }
}
