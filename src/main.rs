use crate::geometry::vec3::{Vec3, Point3, dot, cross, unit_v};

pub mod geometry;

use image::{RgbImage, Rgb};
use std::io::{stdout, Write};

fn main() {
    // Image
    let image_width = 256;
    let image_height = 256;
    let total_pixels = image_width * image_height;

    // Rendering
    let mut image = RgbImage::new(image_width, image_height);

    for y in 0..image_height {
        for x in 0..image_width {

            let r = (x as f32 / (image_width - 1) as f32 * 255.999) as u8;
            let g = (y as f32 / (image_height - 1) as f32 * 255.999) as u8;
            let b = ((x + y) as f32 / (image_width + image_height - 2) as f32 * 255.999) as u8;

            let pixel = Rgb([r, g, b]);
            image.put_pixel(x, y, pixel);

            // Real time progress bar
            let progress = ((y * image_width + x) as f32 / total_pixels as f32) * 100.0;

            print!("\rProgress: [{:<50}] {:.2}% ", "=".repeat((progress / 2.0) as usize), progress);
            stdout().flush().unwrap();
        }
    }
    print!("\r{:<100}", "");
    println!();

    image.save("output.png").expect("Failed to save image");

    println!("Image saved as output.png");

    // Tests
    let v = Vec3::new(1.0, 2.0, 3.0);
    println!("{}", v.length_squared());
    println!("{}", v.length());
    println!("{}", v);
    let p = Point3::new(2.0, 2.0, 4.0);
    println!("{}", p);
    let u = Vec3::new(1.0, 0.0, 1.0);
    let w = v + u;
    println!("{}", w);
    let x = w - u;
    println!("{}", x);

    println!("{}", dot(p, u));
    println!("{}", cross(p, u));
    println!("{}", unit_v(x));
}
