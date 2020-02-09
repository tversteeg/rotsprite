use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rotsprite::scale2x;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("scale2x 3x2 different values", |b| {
        b.iter(|| {
            let buffer = black_box([1, 2, 3, 4, 5, 6]);
            scale2x::scale2x(&buffer, 3, 2);
        });
    });
    c.bench_function("scale2x 3x2 same values", |b| {
        b.iter(|| {
            let buffer = black_box([1, 1, 1, 1, 1, 1]);
            scale2x::scale2x(&buffer, 3, 2);
        });
    });
    c.bench_function("scale2x 6x2 same values", |b| {
        b.iter(|| {
            let buffer = black_box([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
            scale2x::scale2x(&buffer, 6, 2);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
