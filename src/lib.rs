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
    interleaved_to_planar, planar_to_interleaved, ChannelTransform, ChannelTransformBuilder,
    ColorTransformProgram,
};
pub use compress::{compress_aux_data, decompress_aux_data};
pub use errors::{CompressError, DecompressError};
pub use header::{
    Encoder, Filter, Header, Intent, Quantization, BLOCK_DEFAULT, BLOCK_MAX, QUALITY_MAX,
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
    );
    let service_len = original_len - buffer.len();

    let layer_size = header.width as usize * header.height as usize;
    let mut aux_data = vec![0i16; header.layers as usize * header.channels as usize * layer_size];
    color_transform.transform_and_to_planar(&image, &header, &mut aux_data);

    Ok(service_len + compress_aux_data(&mut aux_data, &header, &is_chroma, &mut buffer)?)
}

pub fn decompress_simple(
    mut data: &[u8],
    header: &Header,
    downsampling: usize,
    mut buffer: &mut [u8],
) -> Result<usize, DecompressError> {
    let mut is_chroma = vec![false; header.layers as usize * header.channels as usize];
    let color_transform = ColorTransformProgram::decode(&mut data, &mut is_chroma)?;

    let channel_size =
        header.get_downsampled_width(downsampling) * header.get_downsampled_height(downsampling);

    let layer_size = header.width as usize * header.height as usize;
    let mut aux_data = vec![0i16; header.layers as usize * header.channels as usize * layer_size];
    let next_point_of_interest = decompress_aux_data(
        data,
        &header,
        &is_chroma,
        downsampling,
        false,
        &mut aux_data,
    )?;

    color_transform.detransform_and_to_interleaved(
        &mut aux_data,
        &header,
        channel_size,
        &mut buffer,
    );

    Ok(next_point_of_interest)
}
