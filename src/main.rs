extern crate image;
extern crate rand;

mod camera;
mod geometry;
mod material;
mod vec3;

use std::f32;
use std::fs::File;
use std::path::Path;

use camera::Camera;
use geometry::{Hitable, Sphere};
use material::{Lambertian, Metal, Dielectric};
use vec3::{Ray, Vec3};

fn color(ray: Ray, world: &Hitable, depth: u8) -> Vec3 {
    match world.hit(ray, 0.0001, f32::MAX) {
        Some(hit) => {
            match hit.material.scatter(ray, hit) {
                Some(scatter) => {
                    if depth < 50 {
                        scatter.attenuation * color(scatter.scattered, world, depth + 1)
                    } else {
                        Vec3::new(0.0, 0.0, 0.0)
                    }
                }
                None => Vec3::new(0.0, 0.0, 0.0),
            }
        }
        None => {
            let dir = ray.direction().unit();
            let t = 0.5 * (dir.y() + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn main() {
    let width = 500;
    let height = 250;
    let samples = 100;

    let mut world: Vec<Box<Hitable>> = Vec::new();
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Box::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3)),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Box::new(Dielectric::new(1.5)),
    )));

    let mut image_buf = image::ImageBuffer::new(width, height);
    let lookfrom = Vec3::new(-2.0, 2.0, 1.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        45.0,
        2.0,
        2.0,
        (lookfrom - lookat).length(),
    );

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
            col = col + color(r, &world, 0);
        }
        col = col / (samples as f32);
        col = Vec3::new(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());
        *pixel = image::Rgb(col.rgb());
    }

    let ref mut fout = File::create(&Path::new("image.png")).unwrap();
    let _ = image::ImageRgb8(image_buf).save(fout, image::PNG);
}