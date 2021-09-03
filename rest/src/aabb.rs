use crate::ray::Ray;
use crate::vec3::Vec3;

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let min = Vec3::new(
        f64::min(box0.min.x(), box1.min.x()),
        f64::min(box0.min.y(), box1.min.y()),
        f64::min(box0.min.z(), box1.min.z()),
    );
    let max = Vec3::new(
        f64::max(box0.max.x(), box1.max.x()),
        f64::max(box0.max.y(), box1.max.y()),
        f64::max(box0.max.z(), box1.max.z()),
    );
    AABB { min, max }
}

#[derive(Clone)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        AABB { min, max }
    }

    pub fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction()[a];
            let t0 = (self.min[a] - ray.origin()[a]) * inv_d;
            let t1 = (self.max[a] - ray.origin()[a]) * inv_d;
            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}
