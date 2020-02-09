use blit::*;
use criterion::{criterion_group, criterion_main, Criterion};
use image::*;
use rotsprite::rotsprite;

fn load_image(path: &str) -> (usize, Vec<u32>) {
    // Open the image
    let img = image::open(path).unwrap();
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
            blit::Color::from_u32(0xFF_00_FF),
        );

    (size.0 as usize, img_buf)
}

fn criterion_benchmark(c: &mut Criterion) {
    let (small_width, small_buf) = load_image("examples/king-by-buch.png");

    c.bench_function(
        &*format!("small king 45 degrees ({} width)", small_width),
        |b| {
            b.iter(|| {
                rotsprite(&small_buf, &small_buf[0], small_width, 45.0).unwrap();
            });
        },
    );

    let (large_width, large_buf) = load_image("docs/example-after.png");

    c.bench_function(
        &*format!("large king 45 degrees ({} width)", large_width),
        |b| {
            b.iter(|| {
                rotsprite(&large_buf, &large_buf[0], large_width, 45.0).unwrap();
            });
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
