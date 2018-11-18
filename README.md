# gfwx-rs

[![crates.io](https://img.shields.io/crates/v/gfwx.svg)](https://crates.io/crates/gfwx)
[![docs](https://docs.rs/gfwx/badge.svg)](https://docs.rs/gfwx)
[![build](https://travis-ci.com/vaffeine/gfwx-rs.svg?branch=master)](https://travis-ci.com/vaffeine/gfwx-rs)

Implementation of [GFWX](http://www.gfwx.org/) image compression algorithm developed by Graham Fyffe.
Library uses [rayon](https://github.com/rayon-rs/rayon) for parallelization as a default feature.

## Getting Started

### Prerequisites

To use the library you need to have Rust installed on your machine. Library works on stable Rust branch and doesn't require nightly.

### Using

gfwx-rs is available on crates.io. The recommended way to use it is to add a line into your Cargo.toml such as:
```toml
[dependencies]
gfwx = "0.1"
```

or, if you don't want to use rayon:
```toml
[dependencies]
gfwx = { version = "0.1", default-features = false }
```

Basic usage for compression:

```rust
extern crate gfwx;

fn main() {
    let image = ...;

    let header = gfwx::Header {
        version: 1,
        width: image.width(),
        height: image.height(),
        layers: 1,
        channels: image.channels(),
        bit_depth: 8,
        is_signed: false,
        quality: gfwx::QUALITY_MAX,
        chroma_scale: 8,
        block_size: gfwx::BLOCK_DEFAULT,
        filter: gfwx::Filter::Linear,
        quantization: gfwx::Quantization::Scalar,
        encoder: gfwx::Encoder::Turbo,
        intent: gfwx::Intent::RGBA,
        metadata_size: 0,
    };
    let buffer = vec![0; 2 * image.len()]; // 2 times original size should always be enough
    header.encode(&mut buffer)?;
    let gfwx_size = gfwx::compress_simple(
        image.as_slice(),
        &header,
        &gfwx::ColorTransformProgram::new(), // no color transform
        &mut buffer,
    ).unwrap();
    buffer.truncate(gfwx_size);
}
```

Basic usage for decompression:

```rust
extern crate gfwx;

fn main() {
    let mut compressed = ...;

    let header = gfwx::Header::decode(&mut compressed).unwrap();

    let mut decompressed = vec![0; header.get_estimated_decompress_buffer_size()];
    let next_point_of_interest = gfwx::decompress_simple(
        &mut compressed,
        &header,
        0, // no downsamping
        false, // do not test, full decompress
        &mut decompressed,
    ).unwrap();

    ...
}
```

## Running the tests

### Unit tests

To run unit tests:
```bash
cargo test
```

There are also a test for the case when build should fail. You can run it with
```bash
cargo test --features test_build_fails
```

### Functional tests
To run functional tests, that use actual images, you can use `ci/func_tests.sh`:
```bash
ci/func_tests.sh ci/test_images/
```

This command will build reference application, build examples and run funtional tests
using prepeared images in `ci/test_images/` folder in the `/tmp/gfwx` directory
(so working directory stays clean).

### Benchmarks

There are also [criterion](https://github.com/japaric/criterion.rs) benchmarks which you can run with
```bash
cargo bench
```

### Examples

Examples folder contains 3 applications:
1. `compress` - compresses an input image to gfwx
2. `decompress` - decompresses a gfwx file
3. `compare` - compares two images excluding metadata. Useful for comparing the input image and the decompressed one, because they may have the same "pixels" but different metadata, which means these files will have different checksum

## Features

Library support all features of original implementation except:
- It only support u8 data, when original implementation support 8-bit and 16-bit data both signed and unsigned
- Bayer mode is not supported

However, original implementation supports only channels in interleaved format (for example, [R1, G1, B1, R2, B2, G2, ...]) and always transform channels to planar format.
This is not suitable for color spaces which already use planar channel format (for example, YUV420).

For this type of data our library provides low-level `compress_aux_data` and `decompress_aux_data` functions.
This functions do not encode header, execute and encode ColorTransformProgram and accept 16-bit image data in planar channels format with boost already applied.

This functions are a little bit more complex to use, but provide more flexibility in case you need only image data compression and decompression.
You can manually encode the header with `Header::encode()`, encode `ColorTransformProgram` with `ColorTransformProgram::encode()`
and execute it and apply boost with `ColorTransformProgram::transform()` (for planar channels) and `ColorTransformProgram::transform_and_to_planar()` (for interleaved channels).
Also, instead of using `ColorTransformProgram` you can use `interleaved_to_planar()` and `planar_to_interleaved()` that also can skip some channels during transformation (for example, skip Alpha channel in RGBA).

You can find a complete example on how to use this functions in `examples/test_app.rs` or by looking into `compress_simple` and `decompress_simple` implementation in `src/lib.rs`.
