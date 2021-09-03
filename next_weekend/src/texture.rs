use crate::perlin::Perlin;
use crate::vec3::*;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

#[derive(Clone)]
pub struct SolidColor {
    color: Vec3,
}

impl SolidColor {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        SolidColor {
            color: Vec3::new(r, g, b),
        }
    }
    pub fn from_color(c: Color) -> Self {
        SolidColor {
            color: Vec3::new(c.x(), c.y(), c.z()),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        self.color.clone()
    }
}

#[derive(Clone)]
pub struct CheckerTexture<T: Texture, U: Texture> {
    odd: T,
    even: U,
}

impl<T: Texture, U: Texture> CheckerTexture<T, U> {
    #[allow(dead_code)]
    pub fn new(odd: T, even: U) -> Self {
        CheckerTexture { odd, even }
    }
}

impl<T: Texture, U: Texture> Texture for CheckerTexture<T, U> {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = f64::sin(10.0 * p.x()) * f64::sin(10.0 * p.y()) * f64::sin(10.0 * p.z());
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

#[derive(Clone)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        NoiseTexture {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Vec3 {
        &Vec3::new(0.5, 0.5, 0.5)
            * (1.0 + f64::sin(self.scale * p.z() + 10.0 * self.noise.turb(&p, 7)))
    }
}

#[derive(Clone)]
pub struct ImageTexture {
    data: Vec<u8>,
    nx: u32,
    ny: u32,
}

impl ImageTexture {
    pub fn new(data: Vec<u8>, nx: u32, ny: u32) -> Self {
        ImageTexture { data, nx, ny }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Vec3) -> Vec3 {
        let nx = self.nx as usize;
        let ny = self.ny as usize;
        let mut i = (u * nx as f64) as usize;
        let mut j = ((1.0 - v) * ny as f64) as usize;
        if i > nx - 1 {
            i = nx - 1
        }
        if j > ny - 1 {
            j = ny - 1
        }
        let idx = 3 * i + 3 * nx * j;
        let r = self.data[idx] as f64 / 255.0;
        let g = self.data[idx + 1] as f64 / 255.0;
        let b = self.data[idx + 2] as f64 / 255.0;
        Vec3::new(r, g, b)
    }
}
