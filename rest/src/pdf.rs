use crate::hittable::Hittable;
use crate::onb::ONB;
use crate::vec3::*;
use rand::Rng;
use std::f64;

pub fn random_cosine_direction() -> Vec3 {
    let mut rng = rand::thread_rng();
    let r1 = rng.gen::<f64>();
    let r2 = rng.gen::<f64>();
    let z = (1.0 - r2).sqrt();
    let phi = 2.0 * f64::consts::PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();
    Vec3::new(x, y, z)
}

pub enum PDF<'a> {
    Cosine {
        uvw: ONB,
    },
    Hittable {
        origin: Vec3,
        hittable: &'a Box<dyn Hittable>,
    },
    Mixture {
        p: &'a PDF<'a>,
        q: &'a PDF<'a>,
    },
}

impl<'a> PDF<'a> {
    pub fn cosine(w: &Vec3) -> Self {
        PDF::Cosine {
            uvw: ONB::build_from_w(w),
        }
    }

    pub fn hittable(hittable: &'a Box<dyn Hittable>, origin: &Vec3) -> Self {
        PDF::Hittable {
            origin: origin.clone(),
            hittable: hittable,
        }
    }

    pub fn mixture(p: &'a PDF, q: &'a PDF) -> Self {
        PDF::Mixture { p, q }
    }

    pub fn value(&self, direction: &Vec3) -> f64 {
        match self {
            PDF::Cosine { uvw } => {
                let cosine = dot(&unit_vector(direction), &uvw.w());
                if cosine > 0.0 {
                    cosine / f64::consts::PI
                } else {
                    1.0
                }
            }
            PDF::Hittable { origin, hittable } => hittable.pdf_value(origin, direction),
            PDF::Mixture { p, q } => 0.5 * p.value(direction) + 0.5 * q.value(direction),
        }
    }

    pub fn generate(&self) -> Vec3 {
        match self {
            PDF::Cosine { uvw } => uvw.local(&random_cosine_direction()),
            PDF::Hittable { origin, hittable } => hittable.random(origin),
            PDF::Mixture { p, q } => {
                let mut rng = rand::thread_rng();
                if rng.gen::<bool>() {
                    p.generate()
                } else {
                    q.generate()
                }
            }
        }
    }
}
