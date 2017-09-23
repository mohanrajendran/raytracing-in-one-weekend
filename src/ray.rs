use vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    o: Vec3,
    d: Vec3,
}

impl Ray {
    pub fn new(o: Vec3, d: Vec3) -> Self {
        Ray { o: o, d: d }
    }

    pub fn origin(&self) -> Vec3 {
        self.o
    }
    pub fn direction(&self) -> Vec3 {
        self.d
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.o + self.d * t
    }
}