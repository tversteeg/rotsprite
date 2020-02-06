use blit::*;
use minifb::*;

const WIDTH: usize = 250;
const HEIGHT: usize = 250;

const MASK_COLOR: u32 = 0xFF00FF;

fn main() {
    let mut buffer: Vec<u32> = vec![0x00FFFFFF; WIDTH * HEIGHT];

    let options = WindowOptions {
        scale: Scale::X2,
        ..WindowOptions::default()
    };
    let mut window = Window::new(
        "Rotsprite Example - ESC to exit & click to draw",
        WIDTH,
        HEIGHT,
        options,
    )
    .expect("Unable to open window");

    let img = image::open("examples/king-by-buch.png").unwrap();
    let rgb = img.as_rgba8().unwrap();
    rgb.blit(&mut buffer, WIDTH, (0, 0), Color::from_u32(MASK_COLOR));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
