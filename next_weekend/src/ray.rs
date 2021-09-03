use crate::vec3::{Point3, Vec3};
pub struct Ray {
    orig: Point3,
    dir: Vec3,
    time: f64,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3, time: f64) -> Self {
        Ray { orig, dir, time }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }
    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn at(&self, t: f64) -> Vec3 {
        &self.orig + &(t * &self.dir)
    }
}
