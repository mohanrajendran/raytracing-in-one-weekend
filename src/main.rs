extern crate image;

mod ray;
mod vec3;

use std::fs::File;
use std::path::Path;

use ray::Ray;
use vec3::Vec3;

fn color(ray: &Ray) -> Vec3 {
    let dir = ray.direction().normal();
    let t = 0.5*(dir.y() + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0-t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let width = 200;
    let height = 100;

    let lower_left_corner = Vec3::new(-2.0, -1.0, 1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let mut image_buf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in image_buf.enumerate_pixels_mut() {
        let x = x as f32;
        let y = (height - y) as f32;
        let w = width as f32;
        let h = height as f32;
        let u = x/w;
        let v = y/h;
        let r = Ray::new(origin, lower_left_corner + horizontal*u + vertical*v);
        let col = color(&r);
        *pixel = image::Rgb(col.rgb());
    }

    let ref mut fout = File::create(&Path::new("image.png")).unwrap();
    let _ = image::ImageRgb8(image_buf).save(fout, image::PNG);
}