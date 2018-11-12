use std::{io, u8};

use bits::{BitsIOReader, BitsIOWriter, BitsReader, BitsWriter};
use encode::{signed_code, signed_decode};
use errors::DecompressError;
use header;

#[cfg(test)]
mod test;

#[derive(Clone, Debug, PartialEq)]
pub struct ChannelTransformFactor {
    pub src_channel: usize,
    pub factor: isize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChannelTransform {
    pub dest_channel: usize,
    pub channel_factors: Vec<ChannelTransformFactor>,
    pub denominator: isize,
    pub is_chroma: bool,
}

pub struct ChannelTransformBuilder {
    dest_channel: Option<usize>,
    channel_factors: Vec<ChannelTransformFactor>,
    denominator: isize,
    is_chroma: bool,
}

impl ChannelTransformBuilder {
    pub fn new() -> ChannelTransformBuilder {
        ChannelTransformBuilder {
            dest_channel: None,
            channel_factors: vec![],
            denominator: 1,
            is_chroma: false,
        }
    }

    pub fn set_dest_channel(&mut self, channel: usize) -> &mut Self {
        self.dest_channel = Some(channel);
        self
    }

    pub fn set_chroma(&mut self) -> &mut Self {
        self.is_chroma = true;
        self
    }

    pub fn add_channel_factor(&mut self, src_channel: usize, factor: isize) -> &mut Self {
        self.channel_factors.push(ChannelTransformFactor {
            src_channel,
            factor,
        });
        self
    }

    pub fn set_denominator(&mut self, denominator: isize) -> &mut Self {
        assert!(
            self.denominator > 0,
            "Denominator should be positive integer"
        );
        self.denominator = denominator;
        self
    }

    pub fn build(&self) -> ChannelTransform {
        ChannelTransform {
            dest_channel: *self
                .dest_channel
                .as_ref()
                .expect("Destination channel should be set!"),
            channel_factors: self.channel_factors.clone(),
            denominator: self.denominator,
            is_chroma: self.is_chroma,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ColorTransformProgram {
    channel_transforms: Vec<ChannelTransform>,
}

impl ColorTransformProgram {
    pub fn new() -> Self {
        ColorTransformProgram {
            channel_transforms: vec![],
        }
    }

    /// Stores input as yuv444. Does not perform any color transforms, just flags second and
    /// third channels as chroma channels. Decompressed output also will be in yuv44.
    pub fn yuv444_to_yuv444() -> Self {
        let mut program = Self::new();

        program
            .add_channel_transform(
                ChannelTransformBuilder::new()
                    .set_dest_channel(1)
                    .set_chroma()
                    .build(),
            )
            .add_channel_transform(
                ChannelTransformBuilder::new()
                    .set_dest_channel(2)
                    .set_chroma()
                    .build(),
            );
        program
    }

    /// Stores rgb data as approximated YUV444 (real order is UYV).
    /// performs the following: R -= G (chroma); B -= G (chroma); G += (R + B) / 4 (luma)
    pub fn rgb_to_yuv() -> Self {
        let mut program = Self::new();

        program
            .add_channel_transform(
                ChannelTransformBuilder::new()
                    .set_dest_channel(0)
                    .add_channel_factor(1, -1)
                    .set_chroma()
                    .build(),
            )
            .add_channel_transform(
                ChannelTransformBuilder::new()
                    .set_dest_channel(2)
                    .add_channel_factor(1, -1)
                    .set_chroma()
                    .build(),
            )
            .add_channel_transform(
                ChannelTransformBuilder::new()
                    .set_dest_channel(1)
                    .add_channel_factor(0, 1)
                    .add_channel_factor(2, 1)
                    .set_denominator(4)
                    .build(),
            );
        program
    }

    /// Stores bgr data as approximated a710.
    /// performs the following:
    /// R -= G (chroma); B -= (G * 2 + R) / 2 (chroma); G += (B * 2 + R * 3) / 8 (luma)
    pub fn bgr_to_a710() -> Self {
        let mut program = Self::new();

        program
            .add_channel_transform(
                ChannelTransformBuilder::new()
                    .set_dest_channel(2)
                    .add_channel_factor(1, -1)
                    .set_chroma()
                    .build(),
            )
            .add_channel_transform(
                ChannelTransformBuilder::new()
                    .set_dest_channel(0)
                    .add_channel_factor(1, -2)
                    .add_channel_factor(2, -1)
                    .set_denominator(2)
                    .set_chroma()
                    .build(),
            )
            .add_channel_transform(
                ChannelTransformBuilder::new()
                    .set_dest_channel(1)
                    .add_channel_factor(0, 2)
                    .add_channel_factor(2, 3)
                    .set_denominator(8)
                    .build(),
            );
        program
    }

    /// Stores rgb data as approximated a710.
    /// performs the following:
    /// R -= G (chroma); B -= (G * 2 + R) / 2 (chroma); G += (B * 2 + R * 3) / 8 (luma)
    pub fn rgb_to_a710() -> Self {
        let mut program = Self::new();

        program
            .add_channel_transform(
                ChannelTransformBuilder::new()
                    .set_dest_channel(0)
                    .add_channel_factor(1, -1)
                    .set_chroma()
                    .build(),
            )
            .add_channel_transform(
                ChannelTransformBuilder::new()
                    .set_dest_channel(2)
                    .add_channel_factor(1, -2)
                    .add_channel_factor(0, -1)
                    .set_denominator(2)
                    .set_chroma()
                    .build(),
            )
            .add_channel_transform(
                ChannelTransformBuilder::new()
                    .set_dest_channel(1)
                    .add_channel_factor(2, 2)
                    .add_channel_factor(0, 3)
                    .set_denominator(8)
                    .build(),
            );
        program
    }

    pub fn decode(
        mut buffer: &mut impl io::Read,
        is_chroma: &mut [bool],
    ) -> Result<Self, DecompressError> {
        let mut stream = BitsIOReader::new(&mut buffer);
        let mut color_transform_program = ColorTransformProgram::new();
        loop {
            let dest_channel = signed_decode(&mut stream, 2);

            if dest_channel < 0 {
                break;
            }

            if dest_channel as usize >= is_chroma.len() {
                return Err(DecompressError::Malformed);
            }

            let mut channel_transform_builder = ChannelTransformBuilder::new();
            channel_transform_builder.set_dest_channel(dest_channel as usize);
            loop {
                let src_channel = signed_decode(&mut stream, 2);

                if src_channel < 0 {
                    break;
                }

                let factor = signed_decode(&mut stream, 2);
                channel_transform_builder.add_channel_factor(src_channel as usize, factor as isize);
            }
            let denominator = signed_decode(&mut stream, 2);
            channel_transform_builder.set_denominator(denominator as isize);

            let channel_is_chroma = signed_decode(&mut stream, 2);

            if channel_is_chroma != 0 {
                channel_transform_builder.set_chroma();
                is_chroma[dest_channel as usize] = true;
            }

            color_transform_program.add_channel_transform(channel_transform_builder.build());
        }
        stream.flush_read_word();
        Ok(color_transform_program)
    }

    pub fn add_channel_transform(&mut self, channel_transform: ChannelTransform) -> &mut Self {
        self.channel_transforms.push(channel_transform);
        self
    }

    pub fn is_channel_has_transform(&self, channel: usize) -> bool {
        self.channel_transforms
            .iter()
            .filter(|t| t.dest_channel == channel)
            .any(|c| c.denominator > 1 || c.channel_factors.len() != 0)
    }

    pub fn iter<'a>(&'a self) -> impl DoubleEndedIterator<Item = &'a ChannelTransform> {
        self.channel_transforms.iter()
    }

    pub fn encode(&self, channels: usize, mut buffer: &mut impl io::Write) -> Vec<bool> {
        let mut stream = BitsIOWriter::new(&mut buffer);
        let mut is_chroma = vec![false; channels];

        for channel_transform in &self.channel_transforms {
            signed_code(channel_transform.dest_channel as i32, &mut stream, 2);

            for channel_factor in &channel_transform.channel_factors {
                signed_code(channel_factor.src_channel as i32, &mut stream, 2);
                signed_code(channel_factor.factor as i32, &mut stream, 2);
            }
            signed_code(-1, &mut stream, 2);

            signed_code(channel_transform.denominator as i32, &mut stream, 2);
            signed_code(channel_transform.is_chroma as i32, &mut stream, 2);

            is_chroma[channel_transform.dest_channel] = channel_transform.is_chroma;
        }

        // end of decode program
        signed_code(-1, &mut stream, 2);
        stream.flush_write_word();

        is_chroma
    }

    pub fn transform_and_to_planar(&self, image: &[u8], header: &header::Header, aux: &mut [i16]) {
        assert!(aux.len() >= image.len());

        let boost = header.get_boost() as i16;
        let channels = header.channels as usize;
        let channel_size = header.width as usize * header.height as usize;
        let mut is_channel_transformed =
            vec![false; header.channels as usize * header.layers as usize];

        for channel_transform in &self.channel_transforms {
            let dest_base = channel_transform.dest_channel * channel_size;

            for channel_factor in &channel_transform.channel_factors {
                if is_channel_transformed[channel_factor.src_channel] {
                    for i in 0..channel_size {
                        aux[dest_base + i] += aux[channel_factor.src_channel * channel_size + i]
                            * channel_factor.factor as i16;
                    }
                } else {
                    let layer = (channel_factor.src_channel / channels) * channel_size * channels
                        + channel_factor.src_channel % channels;
                    let boosted_factor = channel_factor.factor as i16 * boost;
                    for i in 0..channel_size {
                        aux[dest_base + i] += image[layer + i * channels] as i16 * boosted_factor;
                    }
                }
            }

            for i in 0..channel_size {
                aux[dest_base + i] /= channel_transform.denominator as i16;
                let layer = (channel_transform.dest_channel / channels) * channel_size * channels
                    + channel_transform.dest_channel % channels;
                aux[dest_base + i] += image[layer + i * header.channels as usize] as i16 * boost;
            }

            is_channel_transformed[channel_transform.dest_channel] = true;
        }

        for (channel, is_transformed) in is_channel_transformed.iter().enumerate() {
            if !is_transformed {
                let dest_base = channel * channel_size;
                let layer = (channel / channels) * channel_size * channels + channel % channels;
                for i in 0..channel_size {
                    aux[dest_base + i] = image[layer + i * channels] as i16 * boost;
                }
            }
        }
    }

    pub fn transform(&self, image: &[u8], header: &header::Header, aux: &mut [i16]) {
        assert!(aux.len() >= image.len());

        let boost = header.get_boost() as i16;

        let channel_size = header.width as usize * header.height as usize;

        let mut is_channel_transformed =
            vec![false; header.channels as usize * header.layers as usize];

        for channel_transform in &self.channel_transforms {
            let dest_base = channel_transform.dest_channel * channel_size;

            for channel_factor in &channel_transform.channel_factors {
                if is_channel_transformed[channel_factor.src_channel] {
                    for i in 0..channel_size {
                        aux[dest_base + i] += aux[channel_factor.src_channel * channel_size + i]
                            * channel_factor.factor as i16;
                    }
                } else {
                    let boosted_factor = channel_factor.factor as i16 * boost;
                    for i in 0..channel_size {
                        aux[dest_base + i] += image[channel_factor.src_channel * channel_size + i]
                            as i16
                            * boosted_factor;
                    }
                }
            }

            for i in 0..channel_size {
                aux[dest_base + i] /= channel_transform.denominator as i16;
                aux[dest_base + i] += image[dest_base + i] as i16 * boost;
            }

            is_channel_transformed[channel_transform.dest_channel] = true;
        }

        for (channel, is_transformed) in is_channel_transformed.iter().enumerate() {
            if !is_transformed {
                let dest_base = channel * channel_size;
                for i in 0..channel_size {
                    aux[dest_base + i] = image[dest_base + i] as i16 * boost;
                }
            }
        }
    }

    pub fn detransform_and_to_parallel(
        &self,
        aux: &mut [i16],
        header: &header::Header,
        channel_size: usize,
        image: &mut [u8],
    ) {
        assert!(image.len() >= aux.len());

        // split off leftover
        let (image, _) = image.split_at_mut(aux.len());

        let channels = header.channels as usize;
        let boost = header.get_boost() as i16;

        for channel_transform in self.channel_transforms.iter().rev() {
            let mut transform_temp = vec![0_i16; channel_size];
            let dest_base = channel_transform.dest_channel * channel_size;

            for channel_factor in &channel_transform.channel_factors {
                for i in 0..channel_size {
                    transform_temp[i] += aux[channel_factor.src_channel * channel_size + i]
                        * channel_factor.factor as i16;
                }
            }

            for i in 0..channel_size {
                transform_temp[i] /= channel_transform.denominator as i16;
                aux[dest_base + i] -= transform_temp[i];
            }
        }

        for c in 0..channels * header.layers as usize {
            let layer = (c / channels) * channel_size * channels + c % channels;
            for i in 0..channel_size {
                image[layer + i * channels] = (aux[c * channel_size + i] / boost)
                    .min(u8::MAX as i16)
                    .max(u8::MIN as i16) as u8;
            }
        }
    }

    pub fn detransform(
        &self,
        aux: &mut [i16],
        header: &header::Header,
        channel_size: usize,
        image: &mut [u8],
    ) {
        assert!(image.len() >= aux.len());

        // split off leftover
        let (image, _) = image.split_at_mut(aux.len());

        let boost = header.get_boost() as i16;

        for channel_transform in self.channel_transforms.iter().rev() {
            let mut transform_temp = vec![0_i16; channel_size];
            let dest_base = channel_transform.dest_channel * channel_size;

            for channel_factor in &channel_transform.channel_factors {
                for i in 0..channel_size {
                    transform_temp[i] += aux[channel_factor.src_channel * channel_size + i]
                        * channel_factor.factor as i16;
                }
            }

            for i in 0..channel_size {
                transform_temp[i] /= channel_transform.denominator as i16;
                aux[dest_base + i] -= transform_temp[i];
            }
        }

        for (dest, src) in image.iter_mut().zip(aux.iter()) {
            *dest = (*src / boost).min(u8::MAX as i16).max(u8::MIN as i16) as u8;
        }
    }
}

pub fn yuv420_to_planar_yuv444(image: &Vec<u8>, width: usize, height: usize) -> Vec<u8> {
    let channel_size = width * height;

    let uv_width = ceil_nearest_even(width) / 2;
    let uv_height = ceil_nearest_even(height) / 2;
    let uv_size = uv_width * uv_height;

    let src_u_base = channel_size;
    let src_v_base = channel_size + uv_size;

    let dest_u_base = channel_size;
    let dest_v_base = channel_size * 2;

    let mut result = vec![0_u8; channel_size * 3];

    for i in 0..channel_size {
        let uv_index = (i / (width * 2)) * uv_width + (i % width) / 2;

        result[i] = image[i]; // Y
        result[dest_u_base + i] = image[src_u_base + uv_index]; // U
        result[dest_v_base + i] = image[src_v_base + uv_index]; // V
    }
    result
}

pub fn planar_yuv444_to_yuv420(image: &Vec<u8>, width: usize, height: usize) -> Vec<u8> {
    let channel_size = width * height;

    let uv_width = ceil_nearest_even(width) / 2;
    let uv_height = ceil_nearest_even(height) / 2;
    let uv_size = uv_width * uv_height;

    let dest_u_base = channel_size;
    let dest_v_base = channel_size + uv_size;

    let src_u_base = channel_size;
    let src_v_base = channel_size * 2;

    let mut result = vec![0_u8; channel_size + uv_size * 2];

    for i in 0..channel_size {
        let uv_index = (i / (width * 2)) * uv_width + (i % width) / 2;

        result[i] = image[i]; // Y
        result[dest_u_base + uv_index] = image[src_u_base + i]; // U
        result[dest_v_base + uv_index] = image[src_v_base + i]; // V
    }
    result
}

pub fn rgba32_to_yuv420(image: &Vec<u8>, width: usize, height: usize) -> Vec<u8> {
    let total = width * height;
    let mut res = Vec::with_capacity(3 * total / 2);
    let mut us = Vec::with_capacity(total);
    let mut vs = Vec::with_capacity(total);
    for j in 0..height {
        for i in 0..width {
            let r = image[4 * j * width + 4 * i] as i32;
            let g = image[4 * j * width + 4 * i + 1] as i32;
            let b = image[4 * j * width + 4 * i + 2] as i32;
            res.push((((66 * r + 129 * g + 25 * b + 128) >> 8) + 16) as u8);
            us.push(((-38 * r - 74 * g + 112 * b + 128) >> 8) + 128);
            vs.push(((112 * r - 94 * g - 18 * b + 128) >> 8) + 128);
        }
    }

    reduce_chroma(&us, width, height, &mut res);
    reduce_chroma(&vs, width, height, &mut res);

    res
}

pub fn yuv420_to_rgba32(image: &Vec<u8>, width: usize, height: usize) -> Vec<u8> {
    let total = width * height;
    let mut res = Vec::with_capacity(4 * total);
    for j in 0..height {
        for i in 0..width {
            let y = image[j * width + i];
            let u = image[(j / 2) * (width / 2) + i / 2 + total];
            let v = image[(j / 2) * (width / 2) + i / 2 + total + total / 4];
            let c = y as i32 - 16;
            let d = u as i32 - 128;
            let e = v as i32 - 128;
            res.push(clamp_to_u8((298 * c + 409 * e + 128) >> 8));
            res.push(clamp_to_u8((298 * c - 100 * d - 208 * e + 128) >> 8));
            res.push(clamp_to_u8((298 * c + 516 * d + 128) >> 8));
            res.push(255);
        }
    }
    res
}

fn reduce_chroma(comonent: &Vec<i32>, width: usize, height: usize, result: &mut Vec<u8>) {
    for j in 0..(height + 1) / 2 {
        for i in 0..(width + 1) / 2 {
            let v1 = comonent[2 * j * width + 2 * i];
            let v2 = if 2 * i + 1 < width {
                comonent[2 * j * width + 2 * i + 1]
            } else {
                v1
            };
            let index3 = 2 * j * width + 2 * i + width;
            let v3 = if index3 < comonent.len() {
                comonent[index3]
            } else {
                v1
            };
            let index4 = 2 * j * width + 2 * i + width + 1;
            let v4 = if index4 >= comonent.len() {
                v2
            } else if 2 * i + 1 >= width {
                v3
            } else {
                comonent[index4]
            };
            result.push(((v1 + v2 + v3 + v4) / 4) as u8);
        }
    }
}

fn clamp_to_u8(val: i32) -> u8 {
    val.min(255).max(0) as u8
}

fn ceil_nearest_even(val: usize) -> usize {
    val + val % 2
}
