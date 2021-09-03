use rand::Rng;

use std::f64::consts::PI;
use std::fmt;
use std::ops;

#[derive(Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec3({}, {}, {})", self.0, self.1, self.2)
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self(0.0, 0.0, 0.0)
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }
    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn assign(&mut self, v: &Vec3) {
        self.0 = v.0;
        self.1 = v.1;
        self.2 = v.2;
    }

    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let s = 1e-8;
        return (self.0.abs() < s) && (self.1.abs() < s) && (self.2.abs() < s);
    }

    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn random_minmax(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

// Unary -
impl ops::Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

// []
impl ops::Index<usize> for Vec3 {
    type Output = f64;
    #[inline]
    fn index(self: &Vec3, i: usize) -> &Self::Output {
        match i {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!(),
        }
    }
}

// [] mut
impl ops::IndexMut<usize> for Vec3 {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!(),
        }
    }
}

// +
impl ops::Add for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

// -
impl ops::Sub for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

// * Vec3
impl ops::Mul<&Vec3> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

// Vec3 * scalar
impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

// scalar * Vec3
impl ops::Mul<&Vec3> for f64 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl ops::Div<&Vec3> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: &Vec3) -> Self::Output {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

// / scalar
impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

// +=
impl ops::AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0 + rhs.0;
        self.1 = self.1 + rhs.1;
        self.2 = self.2 + rhs.2;
    }
}

// -=
impl ops::SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.0 - rhs.0;
        self.1 = self.1 - rhs.1;
        self.2 = self.2 - rhs.2;
    }
}

// *= Vec3
impl ops::MulAssign<&Vec3> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: &Vec3) {
        self.0 = self.0 * rhs.0;
        self.1 = self.1 * rhs.1;
        self.2 = self.2 * rhs.2;
    }
}

// *= scalar
impl ops::MulAssign<f64> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        self.0 = self.0 * rhs;
        self.1 = self.1 * rhs;
        self.2 = self.2 * rhs;
    }
}

// /= Vec3
impl ops::DivAssign<&Vec3> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: &Vec3) {
        self.0 = self.0 / rhs.0;
        self.1 = self.1 / rhs.1;
        self.2 = self.2 / rhs.2;
    }
}

// /= scalar
impl ops::DivAssign<f64> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        self.0 = self.0 / rhs;
        self.1 = self.1 / rhs;
        self.2 = self.2 / rhs;
    }
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
    v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2
}

pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3(
        v1.1 * v2.2 - v1.2 * v2.1,
        -(v1.0 * v2.2 - v1.2 * v2.0),
        v1.0 * v2.1 - v1.1 * v2.0,
    )
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    v / v.length()
}

pub fn random_unit_vector() -> Vec3 {
    let mut rng = rand::thread_rng();
    let phi: f64 = rng.gen_range(0.0..2.0 * PI);
    let z: f64 = rng.gen_range(-1.0..=1.0);
    let rz = f64::powf(1.0, 1. / 3.);
    let rxy = rz * (1.0 - z * z).sqrt();
    return Vec3::new(rxy * phi.cos(), rxy * phi.sin(), rz * z);
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let phi: f64 = rng.gen_range(0.0..2.0 * PI);
    let z: f64 = rng.gen_range(-1.0..=1.0);
    let r: f64 = rng.gen_range(0.0..=1.0);
    let rz = r.powf(1.0 / 3.0);
    let rxy = rz * (1.0 - z * z).sqrt();
    return Vec3::new(rxy * phi.cos(), rxy * phi.sin(), rz * z);
}

#[allow(dead_code)]
pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if dot(&in_unit_sphere, normal) > 0.0
    // In the same hemisphere as the normal
    {
        in_unit_sphere
    } else {
        -&in_unit_sphere
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - &(2.0 * dot(v, n) * n)
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(&-uv, n).min(1.0);
    let r_out_perp = etai_over_etat * &(uv + &(cos_theta * n));
    let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs()).sqrt() * n;
    &r_out_perp + &r_out_parallel
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    let phi: f64 = rng.gen_range(0.0..2.0 * PI);
    let r: f64 = rng.gen_range(0.0..=1.0);
    let rxy = r.sqrt();
    return Vec3::new(rxy * phi.cos(), rxy * phi.sin(), 0.0);
}
