use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::rtweekend::*;
use crate::texture::*;
use crate::vec3::*;

pub trait Material: Sync + Send {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

#[derive(Clone)]
pub struct Lambertian<T: Texture> {
    pub albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(albedo: T) -> Self {
        Lambertian { albedo }
    }
}

impl Lambertian<SolidColor> {
    pub fn from_color(c: Color) -> Self {
        Lambertian {
            albedo: SolidColor::from_color(c),
        }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut scatter_direction = &rec.normal + &random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }
        Some((
            self.albedo.value(rec.u, rec.v, &rec.p),
            Ray::new(rec.p.clone(), scatter_direction, r_in.time()),
        ))
    }
    fn emitted(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        Vec3::default()
    }
}
#[derive(Clone)]

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
            r_in.time(),
        );
        if dot(scattered.direction(), &rec.normal) > 0.0 {
            return Some((self.albedo.clone(), scattered));
        }
        None
    }
    fn emitted(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        Vec3::default()
    }
}

#[derive(Clone)]
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
            Ray::new(rec.p.clone(), direction, r_in.time()),
        ));
    }
    fn emitted(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        Vec3::default()
    }
}

pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

#[derive(Clone)]
pub struct DiffuseLight<T: Texture> {
    emit: T,
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(emit: T) -> Self {
        DiffuseLight { emit }
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _ray: &Ray, _hit: &HitRecord) -> Option<(Vec3, Ray)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.emit.value(u, v, &p)
    }
}

#[derive(Clone)]
pub struct Isotropic<T: Texture> {
    albedo: T,
}

impl<T: Texture> Isotropic<T> {
    pub fn new(albedo: T) -> Self {
        Isotropic { albedo }
    }
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)> {
        let scattered = Ray::new(hit.p, random_in_unit_sphere(), ray.time());
        Some((self.albedo.value(hit.u, hit.v, &hit.p), scattered))
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        Vec3::default()
    }
}
