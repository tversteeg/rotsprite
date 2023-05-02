use blit::{prelude::Size, Blit, BlitOptions, ToBlitBuffer};
use pixels::{PixelsBuilder, SurfaceTexture};
use rotsprite::Rotsprite;
use web_time::SystemTime;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const MASK_COLOR: u32 = 0xFF_FF_FF;

// Window settings
const DST_SIZE: Size = Size {
    width: 200,
    height: 200,
};

async fn run() {
    // Load the image from disk
    let img = image::load_from_memory(include_bytes!("./threeforms.png"))
        .unwrap()
        .to_rgb8()
        .to_blit_buffer_with_mask_color(MASK_COLOR);
    log::info!("Loaded RGBA image with size {:?}", img.size());

    // Setup a winit window
    let size = LogicalSize::new(
        DST_SIZE.width as f64 * 2.0 + 10.0,
        DST_SIZE.height as f64 * 2.0 + 10.0,
    );
    let event_loop = EventLoop::new();
    let mut window_builder = WindowBuilder::new()
        .with_title("Rotsprite")
        .with_inner_size(size);

    // Setup the WASM canvas if running on the browser
    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowBuilderExtWebSys;

        window_builder = window_builder.with_canvas(Some(wasm::setup_canvas()));
    }

    let window = window_builder.build(&event_loop).unwrap();

    let mut pixels = {
        let surface_texture =
            SurfaceTexture::new(DST_SIZE.width * 2 + 10, DST_SIZE.height * 2 + 10, &window);
        PixelsBuilder::new(DST_SIZE.width, DST_SIZE.height, surface_texture)
            .clear_color(pixels::wgpu::Color {
                r: 0.3,
                g: 0.1,
                b: 0.3,
                a: 1.0,
            })
            .build_async()
            .await
    }
    .unwrap();

    // Rotate a tiny bit every frame
    let mut rotation = 0.0f64;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let mut buffer = bytemuck::cast_slice_mut(pixels.frame_mut());
                buffer.fill(0);

                let now = SystemTime::now();

                // Rotate the sprite
                let rotated_blit_buffer = img.rotsprite((rotation / 15.0).round() * 15.0).unwrap();

                log::info!("Rotated sprite in {}ms", now.elapsed().unwrap().as_millis());

                // Draw the rotated sprite
                rotated_blit_buffer.blit(&mut buffer, DST_SIZE, &BlitOptions::new());

                rotation += 0.5;

                // Blit draws the pixels in RGBA format, but the pixels crate expects BGRA, so convert it
                pixels.frame_mut().chunks_exact_mut(4).for_each(|color| {
                    let (r, g, b, a) = (color[0], color[1], color[2], color[3]);

                    color[0] = b;
                    color[1] = g;
                    color[2] = r;
                    color[3] = a;
                });

                if let Err(err) = pixels.render() {
                    log::error!("Pixels error:\n{err}");
                }
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

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Info).expect("error initializing logger");

        wasm_bindgen_futures::spawn_local(run());
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        pollster::block_on(run());
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use wasm_bindgen::JsCast;
    use web_sys::HtmlCanvasElement;

    /// Attach the winit window to a canvas.
    pub fn setup_canvas() -> HtmlCanvasElement {
        log::debug!("Binding window to HTML canvas");

        let window = web_sys::window().unwrap();

        let document = window.document().unwrap();
        let body = document.body().unwrap();
        body.style().set_css_text("text-align: center");

        let canvas = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();

        canvas.set_id("canvas");
        body.append_child(&canvas).unwrap();
        canvas.style().set_css_text("display:block; margin: auto");

        let header = document.create_element("h2").unwrap();
        header.set_text_content(Some("Rotsprite"));
        body.append_child(&header).unwrap();

        canvas
    }
}
