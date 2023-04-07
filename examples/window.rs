use blit::{BlitBuffer, BlitExt};
use image::GenericImageView;
use rotsprite::Rotsprite;
use softbuffer::GraphicsContext;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use std::time::Duration;

const BACKGROUND_COLOR: u32 = 0xFF_CC_FF;
const MASK_COLOR: u32 = 0xFF_FF_FF;

fn main() {
    // Load the image from disk
    let img = image::load_from_memory(include_bytes!("./threeforms.png"))
        .unwrap()
        .into_rgb8()
        .to_blit_buffer_with_mask_color(MASK_COLOR);
    log::info!("Loaded RGBA image with size {:?}", img.size());

    // Setup a winit window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // Setup the WASM canvas if running on the browser
    #[cfg(target_arch = "wasm32")]
    wasm::setup_canvas(&window);

    let mut graphics_context = unsafe { GraphicsContext::new(&window, &window) }.unwrap();

    // The pixel buffer to fill
    let mut buffer: Vec<u32> = Vec::new();

    // Rotate a tiny bit every frame
    let mut rotation = 0.0f64;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let width = window.inner_size().width as usize;
                let height = window.inner_size().height as usize;

                // Clear the buffer first
                buffer.fill(BACKGROUND_COLOR);

                // Redraw the whole buffer if it resized
                if buffer.len() != width * height {
                    log::info!("Buffer resized to {width}x{height}, redrawing");

                    // Resize the buffer with empty values
                    buffer.resize(width * height, BACKGROUND_COLOR);
                }

                // Rotate the sprite
                let rotated_blit_buffer = img.rotsprite((rotation / 15.0).round() * 15.0).unwrap();

                // Draw the rotated sprite
                rotated_blit_buffer.blit(&mut buffer, width, (0, 0));

                rotation += 0.5;

                graphics_context.set_buffer(&buffer, width as u16, height as u16);
            }
            Event::MainEventsCleared => {
                // Animate the next frame
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use wasm_bindgen::prelude::*;
    use winit::{platform::web::WindowExtWebSys, window::Window};

    /// Run main on the browser.
    #[wasm_bindgen(start)]
    pub fn run() {
        console_log::init_with_level(log::Level::Debug).expect("error initializing logger");

        super::main();
    }

    /// Attach the winit window to a canvas.
    pub fn setup_canvas(window: &Window) {
        log::debug!("Binding window to HTML canvas");

        let canvas = window.canvas();

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();
        body.style().set_css_text("text-align: center");

        body.append_child(&canvas).unwrap();
        canvas.style().set_css_text("display:block; margin: auto");
        canvas.set_width(600);
        canvas.set_height(400);

        let header = document.create_element("h2").unwrap();
        header.set_text_content(Some("Rotsprite Example"));
        body.append_child(&header).unwrap();
    }
}
