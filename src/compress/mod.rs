use std::{self, mem, u8};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use bits::{BitsIOReader, BitsIOWriter, BitsReader, BitsWriter};
use config::Config;
use encode::{decode, encode};
use errors::{CompressError, DecompressError};
use header;
use lifting;
use processing::{image::Image, process_maybe_parallel_map_collect, VariableChunksIterator};
use quant;

#[cfg(test)]
mod test;

pub fn compress_aux_data(
    mut aux_data: &mut [i16],
    header: &header::Header,
    is_chroma: &[bool],
    mut buffer: &mut [u8],
) -> Result<usize, CompressError> {
    // [NOTE] current implementation can't go over 2^30
    if header.width > (1 << 30) || header.height > (1 << 30) {
        return Err(CompressError::Malformed);
    }

    let layer_size = header.width as usize * header.height as usize;
    let boost = header.get_boost();

    lift_and_quantize(&mut aux_data, layer_size, &header, &is_chroma, boost);
    compress_image_data(&mut aux_data, &header, &mut buffer, &is_chroma)
}

pub fn decompress_aux_data(
    mut data: &[u8],
    header: &header::Header,
    is_chroma: &[bool],
    downsampling: usize,
    test: bool,
    mut aux_data: &mut [i16],
) -> Result<usize, DecompressError> {
    if header.width > (1 << 30)
        || header.height > (1 << 30)
        || header.get_estimated_decompress_buffer_size() == 0
    {
        return Err(DecompressError::Malformed);
    }

    // check that output data image type [only u8 at the moment] is compatible with the header
    if header.is_signed || header.bit_depth > 8 {
        return Err(DecompressError::TypeMismatch);
    }

    let channel_size =
        header.get_downsampled_width(downsampling) * header.get_downsampled_height(downsampling);

    let payload_next_point_of_interest = decompress_image_data(
        &mut aux_data,
        &header,
        &mut data,
        downsampling,
        test,
        &is_chroma,
    )?;

    if !test {
        unlift_and_dequantize(
            &mut aux_data,
            channel_size,
            &header,
            &is_chroma,
            header.get_boost(),
            downsampling,
        );
    }

    Ok(payload_next_point_of_interest)
}

fn lift_and_quantize(
    aux_data: &mut [i16],
    layer_size: usize,
    header: &header::Header,
    is_chroma: &[bool],
    boost: i16,
) {
    let chroma_quality: i32 = 1
        .max((header.quality as i32 + header.chroma_scale as i32 / 2) / header.chroma_scale as i32);

    aux_data
        .chunks_mut(layer_size)
        .map(|chunk| chunk.chunks_mut(header.width as usize).collect::<Vec<_>>())
        .zip(is_chroma.iter())
        .for_each(|(ref mut image, &is_chroma)| {
            match header.filter {
                header::Filter::Linear => lifting::lift_linear(image),
                header::Filter::Cubic => lifting::lift_cubic(image),
            };

            let quality = if is_chroma {
                chroma_quality
            } else {
                header.quality as i32
            };

            quant::quantize(image, quality, 0, header::QUALITY_MAX as i32 * boost as i32);
        });
}

fn unlift_and_dequantize(
    aux_data: &mut [i16],
    channel_size: usize,
    header: &header::Header,
    is_chroma: &[bool],
    boost: i16,
    downsampling: usize,
) {
    let chroma_quality: i32 = 1
        .max((header.quality as i32 + header.chroma_scale as i32 / 2) / header.chroma_scale as i32);

    aux_data
        .chunks_mut(channel_size)
        .map(|chunk| {
            chunk
                .chunks_mut(header.get_downsampled_width(downsampling))
                .collect::<Vec<_>>()
        })
        .zip(is_chroma.iter())
        .for_each(|(ref mut image, &is_chroma)| {
            let quality = if is_chroma {
                chroma_quality
            } else {
                header.quality as i32
            };
            quant::dequantize(
                image,
                quality << downsampling,
                0,
                header::QUALITY_MAX as i32 * boost as i32,
            );

            match header.filter {
                header::Filter::Linear => lifting::unlift_linear(image),
                header::Filter::Cubic => lifting::unlift_cubic(image),
            };
        });
}

fn compress_image_data(
    aux_data: &mut [i16],
    header: &header::Header,
    buffer: &mut [u8],
    is_chroma: &[bool],
) -> Result<usize, CompressError> {
    let chroma_quality =
        1.max((header.quality + (header.chroma_scale / 2) as u16) / header.chroma_scale as u16);

    let mut step = 1;
    while (step * 2 < header.width) || (step * 2 < header.height) {
        step *= 2;
    }

    let mut has_dc = true;
    let mut compressed_size = 0;
    let hint_do_parallel = aux_data.len() > Config::multithreading_factors().compress;

    while step >= 1 {
        let bs = i64::from(step) << header.block_size;
        let block_count_x = (i64::from(header.width) + bs - 1) / bs;
        let block_count_y = (i64::from(header.height) + bs - 1) / bs;
        let block_count =
            (block_count_x * block_count_y * i64::from(header.layers) * i64::from(header.channels))
                as usize;

        let block_sizes_storage_size = block_count * mem::size_of::<u32>();

        // slice out already compressed data from buffer
        let (_, buffer_remainder) = buffer.split_at_mut(compressed_size);

        if buffer_remainder.len() < block_sizes_storage_size {
            return Err(CompressError::Overflow);
        }

        let (block_sizes_buffer, blocks_buffer) =
            buffer_remainder.split_at_mut(block_sizes_storage_size);

        compressed_size += block_sizes_buffer.len();

        // make iterator over temporary blocks
        let blocks_buffer_free_space = blocks_buffer.len();
        let temp_block_size = blocks_buffer_free_space / block_count;

        let aux_data_chunks = Image::from_slice(
            aux_data,
            (header.width as usize, header.height as usize),
            (header.channels * header.layers) as usize,
        )
        .into_chunks_mut(bs as usize, step as usize);

        let block_encode_results = process_maybe_parallel_map_collect(
            aux_data_chunks
                .zip(blocks_buffer.chunks_mut(temp_block_size))
                .enumerate(),
            |(block_index, (aux_data_chunk, mut output_block))| {
                let empty_block_size = output_block.len();

                let channel = aux_data_chunk.channel;
                let is_first_block_in_channel =
                    aux_data_chunk.x_range.0 == 0 && aux_data_chunk.y_range.0 == 0;

                {
                    let mut output_block_writer = BitsIOWriter::new(&mut output_block);
                    let quality = if is_chroma[channel] {
                        chroma_quality as i32
                    } else {
                        header.quality as i32
                    };
                    encode(
                        aux_data_chunk,
                        &mut output_block_writer,
                        header.encoder,
                        quality,
                        has_dc && is_first_block_in_channel,
                        is_chroma[channel],
                    );

                    output_block_writer.flush_write_word();

                    if output_block_writer.is_overflow_detected() {
                        return Err(CompressError::Overflow);
                    }
                }

                // After writes to output_block, it's size (as slice) is reduced to free space
                Ok((block_index, (empty_block_size - output_block.len()) as u32))
            },
            hint_do_parallel,
        );

        let mut sorted_block_encode_results = vec![0; block_encode_results.len()];
        for result in block_encode_results {
            match result {
                Ok((i, v)) => sorted_block_encode_results[i] = v,
                Err(e) => return Err(e),
            }
        }

        // last block tail relative to blocks_buffer slice start
        let mut tail_pos = 0;

        for (block_index, (block_size, mut block_size_buffer)) in sorted_block_encode_results
            .into_iter()
            .zip(block_sizes_buffer.chunks_mut(std::mem::size_of::<u32>()))
            .enumerate()
        {
            block_size_buffer.write_u32::<LittleEndian>(block_size / 4)?;

            if block_index != 0 {
                let block_start = block_index * temp_block_size;
                for i in 0..block_size as usize {
                    blocks_buffer[tail_pos + i] = blocks_buffer[block_start + i];
                }
            }

            compressed_size += block_size as usize;
            tail_pos += block_size as usize;
        }

        has_dc = false;
        step /= 2;
    }

    Ok(compressed_size)
}

pub fn decompress_image_data(
    aux_data: &mut [i16],
    header: &header::Header,
    buffer: &[u8],
    downsampling: usize,
    test: bool,
    is_chroma: &[bool],
) -> Result<usize, DecompressError> {
    let chroma_quality = 1
        .max((header.quality as i32 + header.chroma_scale as i32 / 2) / header.chroma_scale as i32);

    let mut step = 1;
    while step * 2 < header.width || step * 2 < header.height {
        step *= 2;
    }

    // guess next point of interest
    let mut next_point_of_interest = buffer.len() + 1024;
    let mut is_truncated = false;
    let mut has_dc = true;
    let mut decompressed_size = 0;
    let hint_do_parallel = aux_data.len() > Config::multithreading_factors().compress;

    let downsampled_width = header.get_downsampled_width(downsampling);
    let downsampled_height = header.get_downsampled_height(downsampling);

    while (step >> downsampling) >= 1 {
        let bs = step << header.block_size;

        let block_count_x = (header.width + bs - 1) / bs;
        let block_count_y = (header.height + bs - 1) / bs;
        let block_count =
            (block_count_x * block_count_y * header.layers as u32 * header.channels as u32)
                as usize;

        is_truncated = true;

        let block_sizes_storage_size = block_count * std::mem::size_of::<u32>();

        let (_, buffer_remainder) = buffer.split_at(decompressed_size);

        if buffer_remainder.len() <= block_sizes_storage_size {
            break;
        }

        let (block_sizes_buffer, blocks_buffer) =
            buffer_remainder.split_at(block_sizes_storage_size);

        // get block sizes
        let mut blocks_sizes = vec![0_usize; block_count];

        block_sizes_buffer
            .chunks(std::mem::size_of::<u32>())
            .zip(blocks_sizes.iter_mut())
            .for_each(|(mut chunk_size_buffer, block_size)| {
                *block_size = chunk_size_buffer.read_u32::<LittleEndian>().unwrap() as usize * 4;
            });

        let blocks_size_sum: usize = blocks_sizes.iter().sum();

        next_point_of_interest = decompressed_size
            + block_sizes_storage_size
            + blocks_size_sum
            + if step >> downsampling > 1 {
                // 4 times more block on next iteration
                block_sizes_storage_size * 4
            } else {
                0
            };

        if blocks_buffer.len() >= blocks_size_sum {
            is_truncated = false;
        }

        let step_downsampled = step as usize >> downsampling;
        let block_size_downsampled = (step_downsampled as usize) << header.block_size;

        let aux_data_chunks = Image::from_slice(
            aux_data,
            (downsampled_width, downsampled_height),
            (header.channels * header.layers) as usize,
        )
        .into_chunks_mut(block_size_downsampled, step_downsampled);

        let block_decode_results = process_maybe_parallel_map_collect(
            aux_data_chunks
                .zip(VariableChunksIterator::new(&blocks_buffer, &blocks_sizes))
                .enumerate(),
            |(block_index, (aux_data_chunk, mut input_block))| {
                // truncated chunk does not require any actions.
                if test || input_block.len() < blocks_sizes[block_index] {
                    return Ok(());
                }

                let channel = aux_data_chunk.channel;
                let is_first_block_in_channel =
                    aux_data_chunk.x_range.0 == 0 && aux_data_chunk.y_range.0 == 0;

                {
                    let mut input_block_reader = BitsIOReader::new(&mut input_block);
                    let quality = if is_chroma[channel] {
                        chroma_quality as i32
                    } else {
                        header.quality as i32
                    };
                    decode(
                        aux_data_chunk,
                        &mut input_block_reader,
                        header.encoder,
                        quality,
                        has_dc && is_first_block_in_channel,
                        is_chroma[channel],
                    );

                    if input_block_reader.is_underflow_detected() {
                        return Err(DecompressError::Underflow);
                    }
                }

                Ok(())
            },
            hint_do_parallel,
        );

        for result in block_decode_results {
            result?
        }

        has_dc = false;
        step /= 2;
        decompressed_size += block_sizes_storage_size + blocks_size_sum;
    }

    if is_truncated {
        Ok(next_point_of_interest)
    } else {
        Ok(0)
    }
}
