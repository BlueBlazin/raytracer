use crate::vec3::Vec3;

pub struct Ray {
    origin: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        Self { origin, dir }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        let origin = &self.origin;
        let dir = &self.dir;
        origin + (t * dir)
    }
}
