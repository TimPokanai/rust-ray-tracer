use image::{RgbImage, Rgb};

fn main() {
    // Image
    let image_width = 256;
    let image_height = 256;

    // Rendering
    let mut image = RgbImage::new(image_width, image_height);

    for y in 0..image_height {
        for x in 0..image_width {

            let r = (x as f32 / (image_width - 1) as f32 * 255.999) as u8;
            let g = (y as f32 / (image_height - 1) as f32 * 255.999) as u8;
            let b = ((x + y) as f32 / (image_width + image_height - 2) as f32 * 255.999) as u8;

            let pixel = Rgb([r, g, b]);
            image.put_pixel(x, y, pixel);
        }
    }

    image.save("output.png").expect("Failed to save image");

    println!("Image saved as output.png");
}
