use criterion::{criterion_group, criterion_main, Criterion};
use gfwx::quant::{dequantize, quantize};

fn quantization_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "quantization",
        |b, &&size| {
            let mut image: Vec<_> = (0..size * size).map(|x| x as i16).collect();
            let mut image: Vec<_> = image.chunks_mut(size).collect();
            b.iter(move || quantize(&mut image, 75, 50, 90));
        },
        &[32, 64, 128, 256, 512, 1024],
    );
}

fn dequantization_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "dequantization",
        |b, &&size| {
            let mut image: Vec<_> = (0..size * size).map(|x| x as i16).collect();
            let mut image: Vec<_> = image.chunks_mut(size).collect();
            b.iter(move || dequantize(&mut image, 75, 50, 90));
        },
        &[32, 64, 128, 256, 512, 1024],
    );
}

criterion_group!(benches, quantization_benchmark, dequantization_benchmark);

criterion_main!(benches);
