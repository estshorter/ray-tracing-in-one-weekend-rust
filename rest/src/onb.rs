use crate::vec3::*;

pub struct ONB {
    axis: [Vec3; 3],
}

impl ONB {
    pub fn build_from_w(n: &Vec3) -> Self {
        let w = unit_vector(n);
        let a = if w.x().abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = unit_vector(&cross(&w, &a));
        let u = cross(&w, &v);
        ONB { axis: [u, v, w] }
    }

    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }
    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }
    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }

    pub fn local(&self, a: &Vec3) -> Vec3 {
        a.x() * self.u() + a.y() * self.v() + a.z() * self.w()
    }
}
