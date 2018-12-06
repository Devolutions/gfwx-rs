use std::{io, u8};

use num_traits::cast::NumCast;

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
    dest_channel: usize,
    channel_factors: Vec<ChannelTransformFactor>,
    denominator: isize,
    is_chroma: bool,
}

impl ChannelTransformBuilder {
    pub fn with_dest_channel(dest_channel: usize) -> ChannelTransformBuilder {
        ChannelTransformBuilder {
            dest_channel,
            channel_factors: vec![],
            denominator: 1,
            is_chroma: false,
        }
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
            dest_channel: self.dest_channel,
            channel_factors: self.channel_factors.clone(),
            denominator: self.denominator,
            is_chroma: self.is_chroma,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
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
                ChannelTransformBuilder::with_dest_channel(1)
                    .set_chroma()
                    .build(),
            )
            .add_channel_transform(
                ChannelTransformBuilder::with_dest_channel(2)
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
                ChannelTransformBuilder::with_dest_channel(0)
                    .add_channel_factor(1, -1)
                    .set_chroma()
                    .build(),
            )
            .add_channel_transform(
                ChannelTransformBuilder::with_dest_channel(2)
                    .add_channel_factor(1, -1)
                    .set_chroma()
                    .build(),
            )
            .add_channel_transform(
                ChannelTransformBuilder::with_dest_channel(1)
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
                ChannelTransformBuilder::with_dest_channel(2)
                    .add_channel_factor(1, -1)
                    .set_chroma()
                    .build(),
            )
            .add_channel_transform(
                ChannelTransformBuilder::with_dest_channel(0)
                    .add_channel_factor(1, -2)
                    .add_channel_factor(2, -1)
                    .set_denominator(2)
                    .set_chroma()
                    .build(),
            )
            .add_channel_transform(
                ChannelTransformBuilder::with_dest_channel(1)
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
                ChannelTransformBuilder::with_dest_channel(0)
                    .add_channel_factor(1, -1)
                    .set_chroma()
                    .build(),
            )
            .add_channel_transform(
                ChannelTransformBuilder::with_dest_channel(2)
                    .add_channel_factor(1, -2)
                    .add_channel_factor(0, -1)
                    .set_denominator(2)
                    .set_chroma()
                    .build(),
            )
            .add_channel_transform(
                ChannelTransformBuilder::with_dest_channel(1)
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
            let dest_channel = signed_decode(&mut stream, 2)?;

            if dest_channel < 0 {
                break;
            }

            if dest_channel as usize >= is_chroma.len() {
                return Err(DecompressError::Malformed);
            }

            let mut channel_transform_builder =
                ChannelTransformBuilder::with_dest_channel(dest_channel as usize);
            loop {
                let src_channel = signed_decode(&mut stream, 2)?;

                if src_channel < 0 {
                    break;
                }

                let factor = signed_decode(&mut stream, 2)?;
                channel_transform_builder.add_channel_factor(src_channel as usize, factor as isize);
            }
            let denominator = signed_decode(&mut stream, 2)?;
            channel_transform_builder.set_denominator(denominator as isize);

            let channel_is_chroma = signed_decode(&mut stream, 2)?;

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
            .any(|c| c.denominator > 1 || !c.channel_factors.is_empty())
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &ChannelTransform> {
        self.channel_transforms.iter()
    }

    pub fn encode(&self, channels: usize, mut buffer: &mut impl io::Write) -> io::Result<Vec<bool>> {
        let mut stream = BitsIOWriter::new(&mut buffer);
        let mut is_chroma = vec![false; channels];

        for channel_transform in &self.channel_transforms {
            signed_code(channel_transform.dest_channel as i32, &mut stream, 2)?;

            for channel_factor in &channel_transform.channel_factors {
                signed_code(channel_factor.src_channel as i32, &mut stream, 2)?;
                signed_code(channel_factor.factor as i32, &mut stream, 2)?;
            }
            signed_code(-1, &mut stream, 2)?;

            signed_code(channel_transform.denominator as i32, &mut stream, 2)?;
            signed_code(channel_transform.is_chroma as i32, &mut stream, 2)?;

            is_chroma[channel_transform.dest_channel] = channel_transform.is_chroma;
        }

        // end of decode program
        signed_code(-1, &mut stream, 2)?;
        stream.flush_write_word()?;

        Ok(is_chroma)
    }

    pub fn transform_and_to_planar<T: Into<i16> + Copy>(
        &self,
        image: &[T],
        header: &header::Header,
        aux: &mut [i16],
    ) {
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
                        aux[dest_base + i] += image[layer + i * channels].into() * boosted_factor;
                    }
                }
            }

            for i in 0..channel_size {
                aux[dest_base + i] /= channel_transform.denominator as i16;
                let layer = (channel_transform.dest_channel / channels) * channel_size * channels
                    + channel_transform.dest_channel % channels;
                aux[dest_base + i] += image[layer + i * header.channels as usize].into() * boost;
            }

            is_channel_transformed[channel_transform.dest_channel] = true;
        }

        for (channel, is_transformed) in is_channel_transformed.iter().enumerate() {
            if !is_transformed {
                let dest_base = channel * channel_size;
                let layer = (channel / channels) * channel_size * channels + channel % channels;
                for i in 0..channel_size {
                    aux[dest_base + i] = image[layer + i * channels].into() * boost;
                }
            }
        }
    }

    pub fn transform<T: Into<i16> + Copy>(
        &self,
        image: &[T],
        header: &header::Header,
        aux: &mut [i16],
    ) {
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
                            .into()
                            * boosted_factor;
                    }
                }
            }

            for i in 0..channel_size {
                aux[dest_base + i] /= channel_transform.denominator as i16;
                aux[dest_base + i] += image[dest_base + i].into() * boost;
            }

            is_channel_transformed[channel_transform.dest_channel] = true;
        }

        for (channel, is_transformed) in is_channel_transformed.iter().enumerate() {
            if !is_transformed {
                let dest_base = channel * channel_size;
                for i in 0..channel_size {
                    aux[dest_base + i] = image[dest_base + i].into() * boost;
                }
            }
        }
    }

    pub fn detransform_and_to_interleaved<T: NumCast>(
        &self,
        aux: &mut [i16],
        header: &header::Header,
        channel_size: usize,
        image: &mut [T],
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
                image[layer + i * channels] = T::from(
                    (aux[c * channel_size + i] / boost)
                        .min(u8::MAX.into())
                        .max(u8::MIN.into()),
                )
                .unwrap();
            }
        }
    }

    pub fn detransform<T: NumCast>(
        &self,
        aux: &mut [i16],
        header: &header::Header,
        channel_size: usize,
        image: &mut [T],
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
            *dest = T::from((*src / boost).min(u8::MAX.into()).max(u8::MIN.into())).unwrap();
        }
    }
}

pub fn interleaved_to_planar<T: Into<i16> + Copy>(
    input: &[T],
    channels: usize,
    boost: i16,
    output: &mut [i16],
    skip_channels: &[usize],
) {
    let channel_size = input.len() / channels;

    let mut skipped = 0;
    for c in 0..channels {
        if skip_channels.contains(&c) {
            skipped += 1;
            continue;
        }
        let dest_base = (c - skipped) * channel_size;
        let layer = (c / channels) * channel_size * channels + c % channels;
        for i in 0..channel_size {
            output[dest_base + i] = input[layer + i * channels].into() * boost;
        }
    }
}

pub fn planar_to_interleaved<T: NumCast>(
    input: &[i16],
    channels: usize,
    boost: i16,
    output: &mut [T],
    skip_channels: &[usize],
) {
    let channel_size = output.len() / channels;

    let mut skipped = 0;
    for c in 0..channels {
        if skip_channels.contains(&c) {
            skipped += 1;
            continue;
        }
        let layer = (c / channels) * channel_size * channels + c % channels;
        for i in 0..channel_size {
            output[layer + i * channels] = T::from(
                (input[(c - skipped) * channel_size + i] / boost)
                    .min(u8::MAX.into())
                    .max(u8::MIN.into()),
            )
            .unwrap();
        }
    }
}
