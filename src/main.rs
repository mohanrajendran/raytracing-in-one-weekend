extern crate image;

use std::fs::File;
use std::path::Path;

fn main() {
    let width = 200;
    let height = 100;

    let mut image_buf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in image_buf.enumerate_pixels_mut() {
        let x = x as f32;
        let y = (height - y) as f32;
        let w = width as f32;
        let h = height as f32;
        let r = (255.9 * x / w) as u8;
        let g = (255.9 * y / h) as u8;
        let b = (255.9 * 0.2) as u8;
        *pixel = image::Rgb([r, g, b]);
    }

    let ref mut fout = File::create(&Path::new("image.png")).unwrap();
    image::ImageRgb8(image_buf).save(fout, image::PNG);
}