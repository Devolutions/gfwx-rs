#[macro_use]
extern crate criterion;
extern crate gfwx;

use criterion::Criterion;
use gfwx::lifting::{lift_cubic, lift_linear, unlift_cubic, unlift_linear};

fn linear_lifting_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "linear_lifting",
        |b, &&size| {
            let mut image: Vec<_> = (0..size * size).map(|x| x as i16).collect();
            let mut image: Vec<_> = image.chunks_mut(size).collect();
            b.iter(move || lift_linear(&mut image));
        },
        &[32, 64, 128, 256, 512, 1024],
    );
}

fn linear_unlifting_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "linear_unlifting",
        |b, &&size| {
            let mut image: Vec<_> = (0..size * size).map(|x| x as i16).collect();
            let mut image: Vec<_> = image.chunks_mut(size).collect();
            b.iter(move || unlift_linear(&mut image));
        },
        &[32, 64, 128, 256, 512, 1024],
    );
}

fn cubic_lifting_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "cubic_lifting",
        |b, &&size| {
            let mut image: Vec<_> = (0..size * size).map(|x| x as i16).collect();
            let mut image: Vec<_> = image.chunks_mut(size).collect();
            b.iter(move || lift_cubic(&mut image));
        },
        &[32, 64, 128, 256, 512, 1024],
    );
}

fn cubic_unlifting_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "cubic_unlifting",
        |b, &&size| {
            let mut image: Vec<_> = (0..size * size).map(|x| x as i16).collect();
            let mut image: Vec<_> = image.chunks_mut(size).collect();
            b.iter(move || unlift_cubic(&mut image));
        },
        &[32, 64, 128, 256, 512, 1024],
    );
}

criterion_group!(
    benches,
    linear_lifting_benchmark,
    linear_unlifting_benchmark,
    cubic_lifting_benchmark,
    cubic_unlifting_benchmark,
);

criterion_main!(benches);
