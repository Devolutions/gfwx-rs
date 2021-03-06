// FromPrimitive and ToPrimitive causes clippy error, so we disable it until
// https://github.com/rust-num/num-derive/issues/20 is fixed
#![cfg_attr(feature = "cargo-clippy", allow(clippy::useless_attribute))]

use std::{io, usize};

use crate::errors::HeaderErr;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use num_traits::{FromPrimitive, ToPrimitive};

mod builder;
#[cfg(test)]
mod test;

pub use self::builder::HeaderBuilder;

pub const QUALITY_MAX: u16 = 1024;
pub const BLOCK_DEFAULT: u8 = 7;
pub const BLOCK_MAX: u8 = 30;

const MAGIC: u32 = 'G' as u32 | (('F' as u32) << 8) | (('W' as u32) << 16) | (('X' as u32) << 24);

#[derive(Clone, Copy, Debug, PartialEq, ToPrimitive, FromPrimitive)]
pub enum Filter {
    Linear = 0,
    Cubic = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, ToPrimitive, FromPrimitive)]
pub enum Quantization {
    Scalar = 0,
}

#[derive(Clone, Copy, Debug, PartialEq, ToPrimitive, FromPrimitive)]
pub enum Encoder {
    Turbo = 0,
    Fast = 1,
    Contextual = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, ToPrimitive, FromPrimitive)]
pub enum Intent {
    Generic = 0,
    RGB = 7,
    RGBA = 8,
    BGR = 10,
    BGRA = 11,
    YUV444 = 12,
}

#[derive(Debug, PartialEq)]
pub struct Header {
    pub version: u32,
    pub width: u32,
    pub height: u32,
    pub layers: u16,
    pub channels: u16,
    pub bit_depth: u8,
    pub is_signed: bool,
    pub quality: u16,
    pub chroma_scale: u8,
    pub block_size: u8,
    pub filter: Filter,
    pub quantization: Quantization,
    pub encoder: Encoder,
    pub intent: Intent,
    pub metadata_size: u32,
    pub channel_size: usize,
    pub image_size: usize,
}

impl Header {
    pub fn decode(encoded: &mut impl io::Read) -> Result<Header, HeaderErr> {
        if encoded.read_u32::<LittleEndian>()? != MAGIC {
            return Err(HeaderErr::WrongMagic);
        }

        let version = encoded.read_u32::<LittleEndian>()?;
        let width = encoded.read_u32::<LittleEndian>()?;
        let height = encoded.read_u32::<LittleEndian>()?;
        let channels = encoded
            .read_u16::<LittleEndian>()?
            .checked_add(1)
            .ok_or_else(|| HeaderErr::WrongValue(String::from("Wrong channels value")))?;
        let layers = encoded
            .read_u16::<LittleEndian>()?
            .checked_add(1)
            .ok_or_else(|| HeaderErr::WrongValue(String::from("Wrong layers value")))?;

        let tmp = encoded.read_u24::<LittleEndian>()?;
        let block_size = ((tmp & 0b11111) as u8)
            .checked_add(2)
            .ok_or_else(|| HeaderErr::WrongValue(String::from("Wrong block_size value")))?;
        let chroma_scale = (((tmp >> 5) & 0xff) as u8)
            .checked_add(1)
            .ok_or_else(|| HeaderErr::WrongValue(String::from("Wrong chroma_scale value")))?;
        let quality = (((tmp >> 13) & 0b11_1111_1111) as u16)
            .checked_add(1)
            .ok_or_else(|| HeaderErr::WrongValue(String::from("Wrong quality value")))?;
        let is_signed = (tmp >> 23) == 1;

        let bit_depth = encoded
            .read_u8()?
            .checked_add(1)
            .ok_or_else(|| HeaderErr::WrongValue(String::from("Wrong bit_depth value")))?;
        let intent = Intent::from_u8(encoded.read_u8()?)
            .ok_or_else(|| HeaderErr::WrongValue(String::from("Wrong intent value")))?;
        let encoder = Encoder::from_u8(encoded.read_u8()?)
            .ok_or_else(|| HeaderErr::WrongValue(String::from("Wrong encoder value")))?;
        let quantization = Quantization::from_u8(encoded.read_u8()?)
            .ok_or_else(|| HeaderErr::WrongValue(String::from("Wrong quantization value")))?;
        let filter = Filter::from_u8(encoded.read_u8()?)
            .ok_or_else(|| HeaderErr::WrongValue(String::from("Wrong filter value")))?;
        let metadata_size = encoded
            .read_u32::<LittleEndian>()?
            .checked_mul(4)
            .ok_or_else(|| HeaderErr::WrongValue(String::from("Wrong metadata_size value")))?;

        if is_signed || bit_depth > 8 {
            return Err(HeaderErr::WrongValue(String::from(
                "Unsupported bit_depth and is_signed values",
            )));
        }

        let builder = HeaderBuilder {
            width,
            height,
            layers,
            channels,
            quality,
            chroma_scale,
            block_size,
            filter,
            encoder,
            intent,
            metadata_size,
        };
        let mut header = builder.build()?;
        header.version = version;
        header.bit_depth = bit_depth;
        header.is_signed = is_signed;
        header.quantization = quantization;

        Ok(header)
    }

    pub fn encode(&self, buff: &mut impl io::Write) -> io::Result<()> {
        buff.write_u32::<LittleEndian>(MAGIC)?;
        buff.write_u32::<LittleEndian>(self.version)?;
        buff.write_u32::<LittleEndian>(self.width)?;
        buff.write_u32::<LittleEndian>(self.height)?;
        buff.write_u16::<LittleEndian>(self.channels - 1)?;
        buff.write_u16::<LittleEndian>(self.layers - 1)?;
        let tmp = u32::from(self.block_size - 2)
            | (u32::from(self.chroma_scale - 1) << 5)
            | (u32::from(self.quality - 1) << 13)
            | ((if self.is_signed { 1 } else { 0 }) << 23);
        buff.write_u24::<LittleEndian>(tmp)?;
        buff.write_u8(self.bit_depth - 1)?;
        buff.write_u8(self.intent.to_u8().unwrap())?;
        buff.write_u8(self.encoder.to_u8().unwrap())?;
        buff.write_u8(self.quantization.to_u8().unwrap())?;
        buff.write_u8(self.filter.to_u8().unwrap())?;
        buff.write_u32::<LittleEndian>(self.metadata_size / 4)?;

        Ok(())
    }

    pub fn get_decompress_buffer_size(&self, downsampling: usize) -> usize {
        self.get_downsampled_image_size(downsampling) * ((self.bit_depth + 7) / 8) as usize
    }

    pub fn get_boost(&self) -> i16 {
        if self.quality == QUALITY_MAX {
            1
        } else {
            8
        }
    }

    pub fn get_downsampled_width(&self, downsampling: usize) -> usize {
        (self.width as usize + (1 << downsampling) - 1) >> downsampling
    }

    pub fn get_downsampled_height(&self, downsampling: usize) -> usize {
        (self.height as usize + (1 << downsampling) - 1) >> downsampling
    }

    pub fn get_downsampled_channel_size(&self, downsampling: usize) -> usize {
        self.get_downsampled_width(downsampling) * self.get_downsampled_height(downsampling)
    }

    pub fn get_channel_size(&self) -> usize {
        self.channel_size
    }

    pub fn get_chroma_quality(&self) -> i32 {
        1.max(
            (i32::from(self.quality) + i32::from(self.chroma_scale) / 2)
                / i32::from(self.chroma_scale),
        )
    }

    pub fn get_image_size(&self) -> usize {
        self.image_size
    }

    pub fn get_downsampled_image_size(&self, downsampling: usize) -> usize {
        self.get_downsampled_channel_size(downsampling)
            * self.layers as usize
            * self.channels as usize
    }
}
