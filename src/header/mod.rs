use std::{io, usize};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use errors::HeaderDecodeErr;
use num_traits::{FromPrimitive, ToPrimitive};

#[cfg(test)]
mod test;

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
}

impl Header {
    pub fn decode(encoded: &mut impl io::Read) -> Result<Header, HeaderDecodeErr> {
        if encoded.read_u32::<LittleEndian>()? != MAGIC {
            return Err(HeaderDecodeErr::WrongMagic);
        }

        let version = encoded.read_u32::<LittleEndian>()?;
        let width = encoded.read_u32::<LittleEndian>()?;
        let height = encoded.read_u32::<LittleEndian>()?;
        let channels = encoded.read_u16::<LittleEndian>()? + 1;
        let layers = encoded.read_u16::<LittleEndian>()? + 1;

        let tmp = encoded.read_u24::<LittleEndian>()?;
        let block_size = ((tmp >> 0) & 0b11111) as u8 + 2;
        let chroma_scale = ((tmp >> 5) & 0xff) as u8 + 1;
        let quality = ((tmp >> 13) & 0b1111111111) as u16 + 1;
        let is_signed = (tmp >> 23) == 1;

        let bit_depth = encoded.read_u8()? + 1;
        let intent = Intent::from_u8(encoded.read_u8()?).ok_or(HeaderDecodeErr::WrongValue)?;
        let encoder = Encoder::from_u8(encoded.read_u8()?).ok_or(HeaderDecodeErr::WrongValue)?;
        let quantization =
            Quantization::from_u8(encoded.read_u8()?).ok_or(HeaderDecodeErr::WrongValue)?;
        let filter = Filter::from_u8(encoded.read_u8()?).ok_or(HeaderDecodeErr::WrongValue)?;
        let metadata_size = encoded.read_u32::<LittleEndian>()? * 4;

        Ok(Header {
            version,
            width,
            height,
            layers,
            channels,
            bit_depth,
            is_signed,
            quality,
            chroma_scale,
            block_size,
            filter,
            quantization,
            encoder,
            intent,
            metadata_size,
        })
    }

    pub fn encode(&self, buff: &mut impl io::Write) -> io::Result<()> {
        buff.write_u32::<LittleEndian>(MAGIC)?;
        buff.write_u32::<LittleEndian>(self.version)?;
        buff.write_u32::<LittleEndian>(self.width)?;
        buff.write_u32::<LittleEndian>(self.height)?;
        buff.write_u16::<LittleEndian>(self.channels - 1)?;
        buff.write_u16::<LittleEndian>(self.layers - 1)?;
        let tmp = (self.block_size as u32 - 2)
            | ((self.chroma_scale as u32 - 1) << 5)
            | ((self.quality as u32 - 1) << 13)
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

    pub fn get_estimated_decompress_buffer_size(&self) -> usize {
        let part1 = self.width as f64 * self.height as f64;
        let part2 = self.channels as f64 * self.layers as f64 * ((self.bit_depth + 7) / 8) as f64;

        if part1.ln() + part1.ln() > ((usize::MAX - 1) as f64).ln() {
            0
        } else {
            (part1 * part2) as usize
        }
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
}
