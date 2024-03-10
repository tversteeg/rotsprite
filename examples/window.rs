use std::time::SystemTime;

use blit::{prelude::Size, Blit, BlitBuffer, BlitOptions, ToBlitBuffer};
use pixel_game_lib::{
    canvas::Canvas,
    vek::{Extent2, Vec2},
    window::{Input, KeyCode, WindowConfig},
    PixelGame,
};
use rotsprite::Rotsprite;

/// Mask color of the source image.
const MASK_COLOR: u32 = 0xFF_FF_FF;

/// State wrapping the rotation.
struct State {
    /// Current rotation of the sprite.
    rotation: f64,
    /// Last time since rotation.
    last_time: SystemTime,
    /// The actual image.
    img: BlitBuffer,
    /// The last rotated variant.
    rotated: BlitBuffer,
}

impl PixelGame for State {
    fn update(&mut self, input: &Input, _mouse_pos: Option<Vec2<usize>>, _dt: f32) -> bool {
        // Rotate every second
        if self.last_time.elapsed().unwrap().as_secs_f64() >= 1.0 {
            self.last_time = SystemTime::now();

            self.rotation += 15.0;

            // Rotate the sprite
            self.rotated = self
                .img
                .rotsprite((self.rotation / 15.0).round() * 15.0)
                .unwrap();

            println!(
                "Rotated sprite in {}ms",
                self.last_time.elapsed().unwrap().as_millis()
            );
        }

        // Exit when escape is pressed
        input.key_pressed(KeyCode::Escape)
    }

    fn render(&mut self, canvas: &mut Canvas<'_>) {
        // Reset the canvas
        canvas.fill(0xFFFFFFFF);

        // Draw the rotated sprite
        let canvas_size = Size::new(canvas.width(), canvas.height());
        self.rotated
            .blit(canvas.raw_buffer(), canvas_size, &BlitOptions::new());
    }
}

/// Open the empty window.
fn main() {
    // Load the image from disk
    let img = image::load_from_memory(include_bytes!("./threeforms.png"))
        .unwrap()
        .to_rgb8()
        .to_blit_buffer_with_mask_color(MASK_COLOR);
    let rotated = img.clone();

    // Active modifiable state
    let state = State {
        rotation: 0.0,
        last_time: SystemTime::now(),
        img,
        rotated,
    };

    // Window configuration with huge pixels
    let window_config = WindowConfig {
        buffer_size: Extent2::new(200, 200),
        scaling: 2,
        ..Default::default()
    };

    state.run(window_config).expect("Error running game");
}
