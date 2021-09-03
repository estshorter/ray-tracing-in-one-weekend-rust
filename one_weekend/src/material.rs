use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::rtweekend::*;
use crate::vec3::*;

pub trait Material: Sync + Send {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct UninitMaterial {}
impl Material for UninitMaterial {
    fn scatter(&self, _: &Ray, _: &HitRecord) -> Option<(Vec3, Ray)> {
        None
    }
}
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut scatter_direction = &rec.normal + &random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }
        Some((
            self.albedo.clone(),
            Ray::new(rec.p.clone(), scatter_direction),
        ))
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(&unit_vector(r_in.direction()), &rec.normal);
        let scattered = Ray::new(
            rec.p.clone(),
            reflected + self.fuzz * random_in_unit_sphere(),
        );
        if dot(scattered.direction(), &rec.normal) > 0.0 {
            return Some((self.albedo.clone(), scattered));
        }
        None
    }
}

pub struct Dielectric {
    pub ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let etai_over_etat = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = dot(&-&unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = if (etai_over_etat * sin_theta > 1.0)
            || (random_double() < reflectance(cos_theta, etai_over_etat))
        {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, etai_over_etat)
        };
        return Some((
            Color::new(1.0, 1.0, 1.0),
            Ray::new(rec.p.clone(), direction),
        ));
    }
}

pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
