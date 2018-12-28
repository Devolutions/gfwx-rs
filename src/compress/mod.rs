use std::{self, mem, u8};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::bits::{BitsIOReader, BitsIOWriter, BitsWriter};
use crate::config::Config;
use crate::encode::{decode, encode};
use crate::errors::{CompressError, DecompressError};
use crate::header;
use crate::lifting;
use crate::processing::{
    image::Image, process_maybe_parallel_for_each, process_maybe_parallel_map_collect,
    VariableChunksIterator,
};
use crate::quant;

#[cfg(test)]
mod test;

pub fn compress_aux_data(
    mut aux_data: &mut [i16],
    header: &header::Header,
    is_chroma: &[bool],
    mut buffer: &mut [u8],
) -> Result<usize, CompressError> {
    lift_and_quantize(&mut aux_data, &header, &is_chroma);
    compress_image_data(&mut aux_data, &header, &mut buffer, &is_chroma)
}

pub fn decompress_aux_data(
    data: &[u8],
    header: &header::Header,
    is_chroma: &[bool],
    downsampling: usize,
    test: bool,
    mut aux_data: &mut [i16],
) -> Result<usize, DecompressError> {
    let payload_next_point_of_interest =
        decompress_image_data(&mut aux_data, &header, data, downsampling, test, &is_chroma)?;

    if !test {
        unlift_and_dequantize(&mut aux_data, &header, &is_chroma, downsampling);
    }

    Ok(payload_next_point_of_interest)
}

fn aux_data_to_2d_channel<'a>(
    aux_data: &'a mut [i16],
    header: &'a header::Header,
    is_chroma: &'a [bool],
    downsampling: usize,
) -> impl Iterator<Item = (Vec<&'a mut [i16]>, &'a bool)> {
    let channel_size = header.get_downsampled_channel_size(downsampling);

    aux_data
        .chunks_mut(channel_size)
        .map(move |chunk| {
            chunk
                .chunks_mut(header.get_downsampled_width(downsampling))
                .collect::<Vec<_>>()
        })
        .zip(is_chroma.iter())
}

fn lift_and_quantize(aux_data: &mut [i16], header: &header::Header, is_chroma: &[bool]) {
    let chroma_quality = header.get_chroma_quality();
    let boost = header.get_boost();

    process_maybe_parallel_for_each(
        aux_data_to_2d_channel(aux_data, header, is_chroma, 0),
        |(ref mut image, &is_chroma)| {
            match header.filter {
                header::Filter::Linear => lifting::lift_linear(image),
                header::Filter::Cubic => lifting::lift_cubic(image),
            };

            let quality = if is_chroma {
                chroma_quality
            } else {
                i32::from(header.quality)
            };

            quant::quantize(
                image,
                quality,
                0,
                i32::from(header::QUALITY_MAX) * i32::from(boost),
            );
        },
        true,
    );
}

fn unlift_and_dequantize(
    aux_data: &mut [i16],
    header: &header::Header,
    is_chroma: &[bool],
    downsampling: usize,
) {
    let chroma_quality = header.get_chroma_quality();
    let boost = header.get_boost();

    process_maybe_parallel_for_each(
        aux_data_to_2d_channel(aux_data, header, is_chroma, downsampling),
        |(ref mut image, &is_chroma)| {
            let quality = if is_chroma {
                chroma_quality
            } else {
                i32::from(header.quality)
            };

            quant::dequantize(
                image,
                quality << downsampling,
                0,
                i32::from(header::QUALITY_MAX) * i32::from(boost),
            );

            match header.filter {
                header::Filter::Linear => lifting::unlift_linear(image),
                header::Filter::Cubic => lifting::unlift_cubic(image),
            };
        },
        true,
    );
}

fn compress_image_data(
    aux_data: &mut [i16],
    header: &header::Header,
    buffer: &mut [u8],
    is_chroma: &[bool],
) -> Result<usize, CompressError> {
    let chroma_quality = header.get_chroma_quality();

    let width = header.width as usize;
    let height = header.height as usize;
    let layers = usize::from(header.layers);
    let channels = usize::from(header.channels);

    let mut step = 1;
    while step * 2 < width || step * 2 < height {
        step *= 2;
    }
    if (step << header.block_size) == 0 {
        return Err(CompressError::Overflow);
    }

    let mut has_dc = true;
    let mut compressed_size = 0;
    let hint_do_parallel = aux_data.len() > Config::multithreading_factors().compress;

    while step >= 1 {
        let bs = step << header.block_size;
        let block_count_x = (width + bs - 1) / bs;
        let block_count_y = (height + bs - 1) / bs;
        let block_count = block_count_x * block_count_y * layers * channels;

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

        let aux_data_chunks = Image::from_slice(aux_data, (width, height), channels * layers)
            .into_chunks_mut(bs, step);

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
                        chroma_quality
                    } else {
                        i32::from(header.quality)
                    };
                    encode(
                        &aux_data_chunk,
                        &mut output_block_writer,
                        header.encoder,
                        quality,
                        has_dc && is_first_block_in_channel,
                        is_chroma[channel],
                    )?;

                    output_block_writer.flush_write_word()?;
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

            let block_size = block_size as usize;
            if block_index != 0 {
                let block_start = block_index * temp_block_size;
                for i in 0..block_size {
                    blocks_buffer[tail_pos + i] = blocks_buffer[block_start + i];
                }
            }

            compressed_size += block_size;
            tail_pos += block_size;
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
    let chroma_quality = header.get_chroma_quality();

    let width = header.width as usize;
    let height = header.height as usize;
    let layers = usize::from(header.layers);
    let channels = usize::from(header.channels);

    let mut step = 1;
    while step * 2 < width || step * 2 < height {
        step *= 2;
    }
    if (step << header.block_size) == 0 {
        return Err(DecompressError::Underflow);
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

        let block_count_x = (width + bs - 1) / bs;
        let block_count_y = (height + bs - 1) / bs;
        let block_count = block_count_x * block_count_y * layers * channels;

        is_truncated = true;

        let block_sizes_storage_size = block_count * std::mem::size_of::<u32>();

        if decompressed_size > buffer.len() {
            return Err(DecompressError::Underflow);
        }
        let (_, buffer_remainder) = buffer.split_at(decompressed_size);

        if buffer_remainder.len() <= block_sizes_storage_size {
            break;
        }

        let (block_sizes_buffer, blocks_buffer) =
            buffer_remainder.split_at(block_sizes_storage_size);

        // get block sizes
        let mut blocks_sizes = vec![0_usize; block_count];

        for (mut chunk_size_buffer, block_size) in block_sizes_buffer
            .chunks(std::mem::size_of::<u32>())
            .zip(blocks_sizes.iter_mut())
        {
            *block_size = chunk_size_buffer.read_u32::<LittleEndian>().unwrap() as usize * 4;
        }

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

        let step_downsampled = step >> downsampling;
        let block_size_downsampled = step_downsampled << header.block_size;

        let aux_data_chunks = Image::from_slice(
            aux_data,
            (downsampled_width, downsampled_height),
            channels * layers,
        )
        .into_chunks_mut(block_size_downsampled, step_downsampled);

        let block_decode_results: Vec<std::io::Result<()>> = process_maybe_parallel_map_collect(
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
                        chroma_quality
                    } else {
                        i32::from(header.quality)
                    };
                    decode(
                        aux_data_chunk,
                        &mut input_block_reader,
                        header.encoder,
                        quality,
                        has_dc && is_first_block_in_channel,
                        is_chroma[channel],
                    )?;
                }

                Ok(())
            },
            hint_do_parallel,
        );

        for result in block_decode_results {
            result.map_err(|_| DecompressError::Underflow)?
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
