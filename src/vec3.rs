use std::f32;
use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Clone, Copy, Debug)]
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

    pub fn dot(&self, other: Vec3) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn length(&self) -> f32 {
        self.dot(*self).sqrt()
    }

    pub fn unit(&self) -> Vec3 {
        let len = self.length();
        if len == 0.0 {
            Vec3::new(0.0, 0.0, 0.0)
        } else {
            *self / len
        }
    }

    pub fn rgb(&self) -> [u8; 3] {
        let r = (255.9 * self.0) as u8;
        let g = (255.9 * self.1) as u8;
        let b = (255.9 * self.2) as u8;
        [r, g, b]
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, m: f32) -> Vec3 {
        Vec3::new(self.0 * m, self.1 * m, self.2 * m)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, m: f32) -> Vec3 {
        self * (1.0 / m)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.0, -self.1, -self.2)
    }
}

#[derive(Clone, Copy)]
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