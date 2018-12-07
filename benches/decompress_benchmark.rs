use criterion::{criterion_group, criterion_main, Criterion, ParameterizedBenchmark, Throughput};
use gfwx::{compress_aux_data, decompress_aux_data};

macro_rules! decompress_benchmark {
    ($name:ident, $filter:expr, $mode:expr) => {
        fn $name(c: &mut Criterion) {
            let channels = 3;
            c.bench(
                stringify!($name),
                ParameterizedBenchmark::new(
                    stringify!($name),
                    move |b, &&size| {
                        let builder = gfwx::HeaderBuilder {
                            width: size as u32,
                            height: size as u32,
                            layers: 1,
                            channels,
                            quality: 124,
                            chroma_scale: 8,
                            block_size: gfwx::BLOCK_DEFAULT,
                            filter: $filter,
                            encoder: $mode,
                            intent: gfwx::Intent::RGB,
                            metadata_size: 0,
                        };
                        let header = builder.build().unwrap();

                        let mut aux_data: Vec<_> = (0..header.get_channel_size()
                            * header.channels as u32)
                            .map(|x| (x % 256) as i16)
                            .collect();
                        let mut compressed = vec![0; 2 * aux_data.len()];
                        compress_aux_data(&mut aux_data, &header, &[false; 3], &mut compressed)
                            .unwrap();
                        let mut decompressed = vec![0; aux_data.len()];
                        b.iter(move || {
                            decompress_aux_data(
                                &compressed,
                                &header,
                                &[false; 3],
                                0,
                                false,
                                &mut decompressed,
                            )
                            .unwrap()
                        });
                    },
                    &[128, 256, 512, 1024],
                )
                .throughput(move |&elems| {
                    Throughput::Bytes((elems * elems * channels as i32) as u32)
                }),
            );
        }
    };
}

decompress_benchmark!(
    decompress_linear_contextual,
    gfwx::Filter::Linear,
    gfwx::Encoder::Contextual
);
decompress_benchmark!(
    decompress_cubic_contextual,
    gfwx::Filter::Cubic,
    gfwx::Encoder::Contextual
);
decompress_benchmark!(
    decompress_cubic_fast,
    gfwx::Filter::Cubic,
    gfwx::Encoder::Fast
);
decompress_benchmark!(
    decompress_cubic_turbo,
    gfwx::Filter::Cubic,
    gfwx::Encoder::Turbo
);

criterion_group!(
    benches,
    decompress_linear_contextual,
    decompress_cubic_contextual,
    decompress_cubic_fast,
    decompress_cubic_turbo,
);

criterion_main!(benches);
