use rand;
use std::f32;

use vec3::{Ray, Vec3};

#[derive(Debug)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32,
}

fn random_in_unit_disc() -> Vec3 {
    let mut p = Vec3::new(0.0, 0.0, 0.0);
    loop {
        p = Vec3::new(rand::random::<f32>(), rand::random::<f32>(), 0.0) * 2.0 -
            Vec3::new(1.0, 1.0, 0.0);
        if p.dot(p) < 1.0 {
            break;
        }
    }
    p
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = vfov * f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (lookfrom - lookat).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);
        Camera {
            lower_left_corner: lookfrom - (u * half_width + v * half_height + w) * focus_dist,
            horizontal: u * 2.0 * half_width * focus_dist,
            vertical: v * 2.0 * half_height * focus_dist,
            origin: lookfrom,
            u: u,
            v: v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = random_in_unit_disc() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}