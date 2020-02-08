use blit::*;
use image::*;
use minifb::*;
use rotsprite::rotsprite;

use std::thread::sleep;
use std::time::Duration;

const WIDTH: usize = 32;
const HEIGHT: usize = 32;

const MASK_COLOR: u32 = 0xFF_00_FF;

fn main() {
    // Open a new window with a framebuffer
    let options = WindowOptions {
        scale: Scale::X4,
        ..WindowOptions::default()
    };
    let mut window = Window::new(
        "Rotsprite Example - ESC to exit & click to draw",
        WIDTH,
        HEIGHT,
        options,
    )
    .expect("Unable to open window");

    // Open the image
    let img = image::open("examples/king-by-buch.png").unwrap();
    // Get the size of the image
    let size = img.dimensions();
    // Create a new buffer for this image that can be passed to the rotate function
    let mut img_buf: Vec<u32> = vec![0xFF_FF_FF; (size.0 * size.1) as usize];
    img.as_rgba8()
        .expect("Could not convert image to RGBA8 array")
        .blit(
            &mut img_buf,
            size.0 as usize,
            (0, 0),
            Color::from_u32(MASK_COLOR),
        );

    // Rotate a tiny bit every frame
    let mut rotation: f64 = 0.0;

    // Create a buffer to render
    let mut buf: Vec<u32> = vec![0; WIDTH * HEIGHT];
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Set the edges to gray pixels
        buf.iter_mut().for_each(|p| *p = 0xAA_AA_AA);

        // Rotate the sprite
        let (rotated_width, rotated_height, rotated) = rotsprite(
            &img_buf,
            // The color for pixels that couldn't be found
            &0xFF_FF_FF_FF,
            img.width() as usize,
            // Rotate in increments of 15 degrees
            (rotation / 15.0).round() * 15.0,
        )
        .expect("Could not rotate sprite");

        // Copy the pixels to the buffer
        for y in 0..rotated_height {
            for x in 0..rotated_width {
                buf[y * WIDTH + x] = rotated[y * rotated_width + x];
            }
        }

        // Render the buffer
        window.update_with_buffer(&buf, WIDTH, HEIGHT).unwrap();

        rotation += 0.5;

        // Don't use 100% CPU
        sleep(Duration::from_millis(12));
    }
}
