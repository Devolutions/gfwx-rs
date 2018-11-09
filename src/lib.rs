extern crate byteorder;
extern crate num_traits;
#[macro_use]
extern crate num_derive;
#[cfg(feature = "rayon")]
extern crate rayon;

mod bits;
mod color_transform;
mod compress;
mod config;
mod encode;
mod errors;
mod header;
mod processing;

// this two modules are public for criterion benchmarks
pub mod lifting;
pub mod quant;

pub use color_transform::{
    rgba32_to_yuv420, sequential_yuv444_to_yuv420, yuv420_to_rgba32, yuv420_to_sequential_yuv444,
    ChannelTransform, ChannelTransformBuilder, ColorTransformProgram,
};
pub use compress::{
    compress, compress_sequential_channels, decompress, decompress_sequential_channels,
};
pub use errors::{CompressError, DecompressError};
pub use header::{
    Encoder, Filter, Header, Intent, Quantization, BLOCK_DEFAULT, BLOCK_MAX, QUALITY_MAX,
};
