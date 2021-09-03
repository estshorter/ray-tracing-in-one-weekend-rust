// mod vec3;
use crate::vec3::{Point3, Vec3};
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            orig: Default::default(),
            dir: Default::default(),
        }
    }
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Ray { orig, dir }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Vec3 {
        &self.orig + &(t * &self.dir)
    }
}
