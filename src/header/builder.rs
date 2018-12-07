use super::{Encoder, Filter, Header, HeaderErr, Intent, Quantization};
use std::{fmt::Display, marker::PhantomData};

pub struct HeaderBuilder {
    pub width: u32,
    pub height: u32,
    pub layers: u16,
    pub channels: u16,
    pub quality: u16,
    pub chroma_scale: u8,
    pub block_size: u8,
    pub filter: Filter,
    pub encoder: Encoder,
    pub intent: Intent,
    pub metadata_size: u32,
}

impl HeaderBuilder {
    pub fn build(self) -> Result<Header, HeaderErr> {
        Ok(Header {
            version: 1,
            width: check_range(self.width, 0, 1 << 30, "Width")?,
            height: check_range(self.height, 0, 1 << 30, "Height")?,
            layers: self.layers,
            channels: self.channels,
            bit_depth: 8,
            is_signed: false,
            quality: check_range(self.quality, 0, 1025, "Quality")?,
            chroma_scale: self.chroma_scale,
            block_size: check_range(self.block_size, 0, 31, "Block size")?,
            filter: self.filter,
            quantization: Quantization::Scalar,
            encoder: self.encoder,
            intent: self.intent,
            metadata_size: self.metadata_size,
            ph: PhantomData,
        })
    }
}

fn check_range<T>(value: T, min: T, max: T, name: &str) -> Result<T, HeaderErr>
where
    T: PartialOrd + Display + Copy,
{
    if min < value && value < max {
        Ok(value)
    } else {
        Err(HeaderErr::WrongValue(format!(
            "{} must be in range ({}..{})",
            name, min, max
        )))
    }
}
