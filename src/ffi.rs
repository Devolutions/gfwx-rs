use std::os::raw::{c_int, c_uint, c_ushort};
use crate::HeaderBuilder;
use crate::Filter;
use crate::Encoder;
use crate::Intent;
use crate::compress_simple;
use crate::ColorTransformProgram;
use crate::Header;
use crate::decompress_simple;

#[no_mangle]
pub extern "C" fn gfwx_compress(width: c_uint, height: c_uint, channels: c_ushort, input_data: *const u8, input_data_size: usize, output_data: *mut u8, output_data_size: usize) -> c_int {

    let builder = HeaderBuilder {
        width,
        height,
        layers: 1,
        channels,
        quality: 1024,
        chroma_scale: 8,
        block_size: 7,
        filter: Filter::Linear,
        encoder: Encoder::Turbo,
        intent: Intent::RGB,
        metadata_size: 0,
    };

    let header = if let Ok(header) = builder.build() {
        header
    }
    else {
        return -1;
    };

    let mut compressed = vec![0; 2 * input_data_size];

    let image_data = unsafe { std::slice::from_raw_parts::<u8>(input_data, input_data_size) };

    let gfwx_size = if let Ok(gfwx_size) = compress_simple(
        &image_data,
        &header,
        &ColorTransformProgram::new(),
        &mut compressed) {
        gfwx_size
    }
    else {
        return -1;
    };

    if output_data_size >= gfwx_size {
        let compressed_data = unsafe { std::slice::from_raw_parts_mut::<u8>(output_data, gfwx_size) };
        compressed_data.clone_from_slice(&compressed[0..gfwx_size]);
        return gfwx_size as c_int;
    }
    else {
        return -1;
    }
}

#[no_mangle]
pub extern "C" fn gfwx_decompress(data: *const u8, data_size: usize, output_data: *mut u8, output_data_size: usize) -> c_int {
    let downsampling = 0;
    let mut compressed_data = unsafe { std::slice::from_raw_parts::<u8>(data, data_size) };

    let header = if let Ok(header) = Header::decode(&mut compressed_data) {
        header
    }
    else {
        return -1;
    };

    let mut decompressed = vec![0; header.get_decompress_buffer_size(downsampling)];

    let _result = if let Ok(result) = decompress_simple(&compressed_data, &header, downsampling, false, &mut decompressed) {
        result
    }
    else {
        return -1;
    };

    if output_data_size >= decompressed.len() {
        let decompressed_data = unsafe { std::slice::from_raw_parts_mut::<u8>(output_data, decompressed.len()) };
        decompressed_data.clone_from_slice(&decompressed[0..decompressed.len()]);
        return decompressed.len() as c_int;
    }
    else {
        return -1
    }
}
