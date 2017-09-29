extern crate image;
extern crate rand;

mod camera;
mod geometry;
mod material;
mod vec3;

use rand::random;
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

fn random_scene() -> Vec<Box<Hitable>> {
    let mut world: Vec<Box<Hitable>> = Vec::new();
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Lambertian::new(Vec3::new(
            random::<f32>() * random::<f32>(),
            random::<f32>() * random::<f32>(),
            random::<f32>() * random::<f32>(),
        ))),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f32;
            let b = b as f32;
            let choose_mat = random::<f32>();
            let center = Vec3::new(a + 0.9 * random::<f32>(), 0.2, b + 0.9 * random::<f32>());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    world.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Lambertian::new(Vec3::new(
                            random::<f32>() * random::<f32>(),
                            random::<f32>() * random::<f32>(),
                            random::<f32>() * random::<f32>(),
                        ))),
                    )));
                } else if choose_mat < 0.95 {
                    world.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Metal::new(
                            Vec3::new(
                                0.5 * (1.0 + random::<f32>()),
                                0.5 * (1.0 + random::<f32>()),
                                0.5 * (1.0 + random::<f32>()),
                            ),
                            0.5 * random::<f32>(),
                        )),
                    )));
                } else {
                    world.push(Box::new(
                        Sphere::new(center, 0.2, Box::new(Dielectric::new(1.5))),
                    ));
                }
            }
        }
    }

    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(Dielectric::new(1.5)),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));

    world
}

fn main() {
    let width = 640;
    let height = 360;
    let samples = 250;

    let world = random_scene();

    let mut image_buf = image::ImageBuffer::new(width, height);
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        (width as f32) / (height as f32),
        aperture,
        dist_to_focus,
    );

    for (x, y, pixel) in image_buf.enumerate_pixels_mut() {
        let x = x as f32;
        let y = (height - y) as f32;
        let w = width as f32;
        let h = height as f32;
        let mut col = Vec3::new(0.0, 0.0, 0.0);
        for _ in 0..samples {
            let u = (x + random::<f32>()) / w;
            let v = (y + random::<f32>()) / h;
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