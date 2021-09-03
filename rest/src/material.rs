use crate::hittable::HitRecord;
use crate::pdf::*;
use crate::ray::Ray;
use crate::rtweekend::random_double;
use crate::texture::*;
use crate::vec3::*;

pub enum ScatterRecord<'a> {
    Specular {
        specular_ray: Ray,
        attenuation: Vec3,
    },
    Scatter {
        pdf: PDF<'a>,
        attenuation: Vec3,
    },
}

pub trait Material: Sync + Send {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn scattering_pdf(&self, _ray: &Ray, _hit: &HitRecord, _scattered: &Ray) -> f64 {
        1.0
    }

    fn emitted(&self, _ray: &Ray, _hit: &HitRecord) -> Vec3 {
        Vec3::default()
    }
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
    #[allow(dead_code)]
    pub fn from_color(c: Color) -> Self {
        Lambertian {
            albedo: SolidColor::from_color(c),
        }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord::Scatter {
            pdf: PDF::cosine(&hit.normal),
            attenuation: self.albedo.value(hit.u, hit.v, &hit.p),
        })
    }

    fn scattering_pdf(&self, _ray: &Ray, hit: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = dot(&hit.normal, &unit_vector(&scattered.direction()));
        if cosine < 0. {
            0.
        } else {
            cosine / std::f64::consts::PI
        }
    }
}

#[derive(Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    #[allow(dead_code)]
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let mut reflected = reflect(&unit_vector(&ray.direction()), &hit.normal);
        if self.fuzz > 0.0 {
            reflected += self.fuzz * random_in_unit_sphere()
        };
        if dot(&reflected, &hit.normal) > 0.0 {
            Some(ScatterRecord::Specular {
                specular_ray: Ray::new(hit.p, reflected, ray.time()),
                attenuation: self.albedo,
            })
        } else {
            None
        }
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = dot(&-&unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_double() {
                reflect(&unit_direction, &rec.normal)
            } else {
                refract(&unit_direction, &rec.normal, refraction_ratio)
            };
        return Some(ScatterRecord::Specular {
            specular_ray: Ray::new(rec.p.clone(), direction, r_in.time()),
            attenuation: Vec3::new(1.0, 1.0, 1.0),
        });
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
    fn emitted(&self, _r_in: &Ray, rec: &HitRecord) -> Vec3 {
        if rec.front_face {
            self.emit.value(rec.u, rec.v, &rec.p)
        } else {
            Color::default()
        }
    }
}

// #[derive(Clone)]
// pub struct Isotropic<T: Texture> {
//     albedo: T,
// }

// impl<T: Texture> Isotropic<T> {
//     pub fn new(albedo: T) -> Self {
//         Isotropic { albedo }
//     }
// }

// impl<T: Texture> Material for Isotropic<T> {
//     fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)> {
//         let scattered = Ray::new(hit.p, random_in_unit_sphere(), ray.time());
//         Some((self.albedo.value(hit.u, hit.v, &hit.p), scattered))
//     }
// }
