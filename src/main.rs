extern crate image;

mod vec3;

use std::fs::File;
use std::path::Path;

use vec3::Vec3;

fn main() {
    let width = 200;
    let height = 100;

    let mut image_buf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in image_buf.enumerate_pixels_mut() {
        let x = x as f32;
        let y = (height - y) as f32;
        let w = width as f32;
        let h = height as f32;
        let color = Vec3::new(x/w, y/h, 0.2);
        *pixel = image::Rgb(color.rgb());
    }

    let ref mut fout = File::create(&Path::new("image.png")).unwrap();
    let _ = image::ImageRgb8(image_buf).save(fout, image::PNG);
}