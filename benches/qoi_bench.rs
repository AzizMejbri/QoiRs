use criterion::{Criterion, black_box, criterion_group, criterion_main};
use qoi::qoi::{encoder::encode_, types::Pixel};

fn bench_encode(c: &mut Criterion) {
    // Create test image
    let width = 512;
    let height = 512;
    let mut image = Vec::with_capacity((width * height) as usize);
    for y in 0..height {
        for x in 0..width {
            image.push(Pixel::new(
                (x % 256) as u8,
                (y % 256) as u8,
                ((x + y) % 256) as u8,
                255,
            ));
        }
    }

    let mut array = [Pixel::default(); 64];

    c.bench_function("encode_512x512", |b| {
        b.iter(|| {
            let _ = encode_(
                black_box(&image),
                black_box(&mut array),
                black_box(width),
                black_box(height),
            );
        })
    });
}

criterion_group!(benches, bench_encode);
criterion_main!(benches);
