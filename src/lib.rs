#![warn(rust_2018_idioms)]

#[macro_use]
extern crate num_derive;

pub mod color_transform;
pub mod compress;
pub mod config;
pub mod encode;
pub mod errors;
pub mod header;
pub mod processing;

// this 3 modules are public for criterion benchmarks
pub mod bits;
pub mod lifting;
pub mod quant;

pub use crate::color_transform::{
    interleaved_to_planar, planar_to_interleaved, ChannelTransform, ChannelTransformBuilder,
    ColorTransformProgram,
};
pub use crate::compress::{compress_aux_data, decompress_aux_data};
pub use crate::errors::{CompressError, DecompressError};
pub use crate::header::{
    Encoder, Filter, Header, HeaderBuilder, Intent, Quantization, BLOCK_DEFAULT, BLOCK_MAX,
    QUALITY_MAX,
};

pub fn compress_simple(
    image: &[u8],
    header: &Header,
    color_transform: &ColorTransformProgram,
    mut buffer: &mut [u8],
) -> Result<usize, CompressError> {
    let original_len = buffer.len();
    header.encode(&mut buffer)?;
    let is_chroma = color_transform.encode(
        header.channels as usize * header.layers as usize,
        &mut buffer,
    )?;
    let service_len = original_len - buffer.len();

    let mut aux_data = vec![0i16; header.get_image_size()];
    color_transform.transform_and_to_planar(&image, &header, &mut aux_data);

    Ok(service_len + compress_aux_data(&mut aux_data, &header, &is_chroma, &mut buffer)?)
}

pub fn decompress_simple(
    mut data: &[u8],
    header: &Header,
    downsampling: usize,
    test: bool,
    mut buffer: &mut [u8],
) -> Result<usize, DecompressError> {
    let mut is_chroma = vec![false; header.layers as usize * header.channels as usize];
    let color_transform = ColorTransformProgram::decode(&mut data, &mut is_chroma)?;

    let mut aux_data = vec![0i16; header.get_downsampled_image_size(downsampling)];
    let next_point_of_interest =
        decompress_aux_data(data, &header, &is_chroma, downsampling, test, &mut aux_data)?;

    color_transform.detransform_and_to_interleaved(
        &mut aux_data,
        &header,
        header.get_downsampled_channel_size(downsampling),
        &mut buffer,
    );

    Ok(next_point_of_interest)
}
