use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use image::ImageReader;
use quantise::palette::{self, Palette};
use std::hint::black_box;

fn bench_palette(c: &mut Criterion) {
    let input = ImageReader::open("in-small.jpeg")
        .expect("Failed to read image")
        .decode()
        .expect("Failed to decode image");

    let mut group = c.benchmark_group("Palette");

    for m in 2..=16 {
        group.bench_with_input(BenchmarkId::new("V1", m), &m, |b, m| {
            b.iter(|| palette::V1::palette(black_box(&input), black_box(*m)))
        });
        group.bench_with_input(BenchmarkId::new("V2", m), &m, |b, m| {
            b.iter(|| palette::V2::palette(black_box(&input), black_box(*m)))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_palette);
criterion_main!(benches);
