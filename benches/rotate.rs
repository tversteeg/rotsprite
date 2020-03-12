use criterion::{criterion_group, criterion_main, Criterion};
use rotsprite::{rotate, scale2x};

fn criterion_benchmark(c: &mut Criterion) {
    let (width, height, buffer) = scale2x::scale2x(&[1, 2, 3, 4, 5, 6], 3, 2);
    c.bench_function("rotate90 3x2", |b| {
        b.iter(|| {
            rotate::rotate90(&buffer, width, height);
        });
    });
    c.bench_function("rotate180 3x2", |b| {
        b.iter(|| {
            rotate::rotate180(&buffer, width, height);
        });
    });
    c.bench_function("rotate270 3x2", |b| {
        b.iter(|| {
            rotate::rotate270(&buffer, width, height);
        });
    });
    c.bench_function("rotate45 3x2", |b| {
        b.iter(|| {
            rotate::rotate(&buffer, &0, width, height, 45.0, 1);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
