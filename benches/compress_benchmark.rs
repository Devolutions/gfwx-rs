#[macro_use]
extern crate criterion;
extern crate gfwx;

use criterion::Criterion;
use gfwx::compress_aux_data;

macro_rules! compress_benchmark {
    ($name:ident, $quality:expr, $filter:expr, $mode:expr) => {
        fn $name(c: &mut Criterion) {
            c.bench_function_over_inputs(
                stringify!($name),
                |b, &&size| {
                    let mut image: Vec<_> = (0..size * size).map(|x| x as i16).collect();
                    let mut compressed = vec![0; 2 * image.len()];
                    let header = gfwx::Header {
                        version: 1,
                        width: size as u32,
                        height: size as u32,
                        layers: 1,
                        channels: 1,
                        bit_depth: 8,
                        is_signed: false,
                        quality: $quality,
                        chroma_scale: 8,
                        block_size: gfwx::BLOCK_DEFAULT,
                        filter: $filter,
                        quantization: gfwx::Quantization::Scalar,
                        encoder: $mode,
                        intent: gfwx::Intent::RGB,
                        metadata_size: 0,
                    };
                    b.iter(move || {
                        compress_aux_data(&mut image, &header, &[false; 3], &mut compressed)
                            .unwrap()
                    });
                },
                &[128, 256, 512, 1024, 2048],
            );
        }
    };
}

compress_benchmark!(
    compress_lossless_linear_cont_benchmark,
    gfwx::QUALITY_MAX,
    gfwx::Filter::Linear,
    gfwx::Encoder::Contextual
);
compress_benchmark!(
    compress_losy_cubic_cont_benchmark,
    gfwx::QUALITY_MAX / 2,
    gfwx::Filter::Cubic,
    gfwx::Encoder::Contextual
);
compress_benchmark!(
    compress_lossless_linear_fast_benchmark,
    gfwx::QUALITY_MAX,
    gfwx::Filter::Linear,
    gfwx::Encoder::Fast
);
compress_benchmark!(
    compress_lossless_linear_turbo_benchmark,
    gfwx::QUALITY_MAX,
    gfwx::Filter::Linear,
    gfwx::Encoder::Turbo
);

criterion_group!(
    benches,
    compress_lossless_linear_cont_benchmark,
    compress_losy_cubic_cont_benchmark,
    compress_lossless_linear_fast_benchmark,
    compress_lossless_linear_turbo_benchmark
);

criterion_main!(benches);
