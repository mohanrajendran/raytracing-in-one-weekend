#![allow(dead_code)]
#[derive(Debug)]
pub struct Vec3(f32, f32, f32);

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Vec3 {
            0: e0,
            1: e1,
            2: e2,
        }
    }

    pub fn x(&self) -> f32 {
        self.0
    }
    pub fn y(&self) -> f32 {
        self.1
    }
    pub fn z(&self) -> f32 {
        self.2
    }

    pub fn rgb(&self) -> [u8; 3] {
        let r = (255.9 * self.0) as u8;
        let g = (255.9 * self.1) as u8;
        let b = (255.9 * self.2) as u8;
        [r, g, b]
    }
}
