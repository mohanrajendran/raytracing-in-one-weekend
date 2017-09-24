extern crate image;
extern crate rand;

mod camera;
mod geometry;
mod ray;
mod sphere;
mod vec3;

use std::f32;
use std::fs::File;
use std::path::Path;

use camera::Camera;
use geometry::{Hit, Hitable};
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

fn color(ray: Ray, world: &Hitable) -> Vec3 {
    match world.hit(ray, 0.0, f32::MAX) {
        Some(hit) => {
            Vec3::new(
                hit.normal.x() + 1.0,
                hit.normal.y() + 1.0,
                hit.normal.z() + 1.0,
            ) * 0.5
        }
        None => {
            let dir = ray.direction().normal();
            let t = 0.5 * (dir.y() + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn main() {
    let width = 400;
    let height = 200;
    let samples = 100;

    let mut world: Vec<Box<Hitable>> = Vec::new();
    world.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let mut image_buf = image::ImageBuffer::new(width, height);
    let camera = Camera::new();

    for (x, y, pixel) in image_buf.enumerate_pixels_mut() {
        let x = x as f32;
        let y = (height - y) as f32;
        let w = width as f32;
        let h = height as f32;
        let mut col = Vec3::new(0.0, 0.0, 0.0);
        for _ in 0..samples {
            let u = (x + rand::random::<f32>()) / w;
            let v = (y + rand::random::<f32>()) / h;
            let r = camera.get_ray(u, v);
            col = col + color(r, &world);
        }
        col = col / (samples as  f32);
        *pixel = image::Rgb(col.rgb());
    }

    let ref mut fout = File::create(&Path::new("image.png")).unwrap();
    let _ = image::ImageRgb8(image_buf).save(fout, image::PNG);
}