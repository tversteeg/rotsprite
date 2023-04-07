use image::{Rgba, RgbaImage};
use rotsprite::rotsprite;

fn main() {
    // Open the image
    let img = image::open("examples/threeforms.png").unwrap();
    let width = img.width() as usize;
    let image: &RgbaImage = img
        .as_rgba8()
        .expect("Could not convert image to RGBA8 array");

    let pixels: Vec<Rgba<u8>> = image.pixels().copied().collect();
    let unfound_color = Rgba([0, 0, 0, 0]);
    let rotation_angle: f64 = 45.0; //Rotate in increments of 15 degrees
    let (rotated_width, rotated_height, rotated) = rotsprite(
        &pixels,
        &unfound_color, // The color for pixels that couldn't be found
        width,
        rotation_angle,
    )
    .expect("Could not rotate sprite");

    let rotated_image = RgbaImage::from_fn(rotated_width as u32, rotated_height as u32, |x, y| {
        rotated[rotated_width * y as usize + x as usize]
    });
    rotated_image
        .save("rotated.png")
        .expect("Failed to save the image");
}
