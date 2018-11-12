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
    let gfwx_size = gfwx::compress_interleaved(
        image.as_slice(),
        &header,
        &mut buffer,
        &[], // no metadata
        &gfwx::ColorTransformProgram::new(), // no color transform
    ).unwrap();
    buffer.truncate(gfwx_size);
}
```

Basic usage for decompression:

```rust
extern crate gfwx;

fn main() {
    let mut compressed = ...;

    let mut cursor = io::Cursor::new(compressed);
    let header = gfwx::Header::decode(&mut cursor).unwrap();
    let header_size = cursor.position() as usize;
    let compressed = cursor.into_inner();

    let mut decompressed = vec![0; header.get_estimated_decompress_buffer_size()];
    let next_point_of_interest = gfwx::decompress_interleaved(
        &mut compressed[header_size..],
        &header,
        &mut decompressed,
        0, // no downsamping
        false, // do not test, full decompress
    ).unwrap();

    ...
}
```

You can find a complete example in `examples/test_app.rs`.

## Running the tests

### Unit tests

To run unit tests:
```
cargo test
```

There are also a test for the case when build should fail. You can run it with
```
cargo test --features test_build_fails
```

### Benchmarks

There are also [criterion](https://github.com/japaric/criterion.rs) benchmarks which you can run with
```
cargo bench
```

### Examples

Examples folder contains 4 components:
1. `test_app` - compresses an input image to gfwx, writes it to the file, and decompresses it back to the input format with given options
2. `compare` - compares two images excluding metadata. Useful for comparing the input image and the decompressed one, because they may have the same "pixels" but different metadata, which means these files will have different checksum
3. `reference_app` - folder with source code of test app created with original GFWX implementation. Usefull for comparing GFWX produced by library and the reference implementation.
4. `test_helper.py` - automatically checks compression and decompression for all the .png images in the specified folder. This script uses other binaries that must be located in the same folder.

To build reference application, you need CMake and OpenCV to be installed on your system. Then:
```bash
cd examples/reference_app/
mkdir build
cd build
cmake ..
make
```

## Features

Library support all features of original implementation except:
- It only support u8 data, when original implementation support 8-bit and 16-bit data both signed and unsigned
- Bayer mode is not supported

However, original implementation supports only channels in interleaved format (for example, [R1, G1, B1, R2, B2, G2, ...]) and always transform channels to planar format.
This is not suitable for color spaces which already use planar channel format (for example, YUV420). For this type of data our library provides `compress_planar` and `decompress_planar` functions which doesn't change the format of the channels.

### YUV420 support

This library also provides functions to convert from RGBA32 to YUV420 and back. But unfortunately, GFWX doesn't support channels of different size, which is the case of YUV420.
As a workaround, library provides `yuv420_to_planar_yuv444` and `planar_yuv444_to_yuv420` functions, that transform YUV420 to YUV444 but with planar channels format.
We found out that usage of YUV444 as an internal format (instead of RGB, for example) increases compression ratio and speed, even considering time required for transformation.
`test_app` performs transformation from image intent to YUV444 (through YUV420 for demo purposes) if '--intent yuv420' option was passed:

```rust
let yuv420 = gfwx::rgba32_to_yuv420(&rgba32, width, height);
let yuv444 = gfwx::yuv420_to_planar_yuv444(&yuv420, width, height);
gfwx::compress_planar(
    &yuv444,
    &header,
    &mut compressed,
    &[],
    &gfwx::ColorTransformProgram::yuv444_to_yuv444()
)?;

...

gfwx::decompress_planar(
    &mut compressed[header_size..],
    &header,
    &mut decompressed,
    0,
    false,
)?
let yuv420 = gfwx::planar_yuv444_to_yuv420(&decompressed, width, height);
let rgba32 = gfwx::yuv420_to_rgba32(&yuv420, width, height);
```
