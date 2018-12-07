use std::{error::Error, fmt, io};

#[derive(Debug)]
pub enum CompressError {
    IOErr(io::Error),
    Overflow,
    Malformed,
}

#[derive(Debug)]
pub enum DecompressError {
    IOErr(io::Error),
    Unsupported,
    Underflow,
    Malformed,
    TypeMismatch,
}

impl From<io::Error> for CompressError {
    fn from(err: io::Error) -> Self {
        CompressError::IOErr(err)
    }
}

impl fmt::Display for CompressError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompressError::IOErr(e) => write!(f, "{}", e),
            CompressError::Overflow => write!(f, "Buffer is too small"),
            CompressError::Malformed => write!(f, "Invalid arguments"),
        }
    }
}

impl Error for CompressError {}

impl From<io::Error> for DecompressError {
    fn from(err: io::Error) -> Self {
        DecompressError::IOErr(err)
    }
}

impl fmt::Display for DecompressError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecompressError::IOErr(e) => write!(f, "{}", e),
            DecompressError::Unsupported => write!(f, "Unsupported image format"),
            DecompressError::Malformed => write!(f, "Invalid arguments"),
            DecompressError::Underflow => write!(f, "Buffer underflow detected"),
            DecompressError::TypeMismatch => write!(f, "Image data doesn't match the header"),
        }
    }
}

impl Error for DecompressError {}

#[derive(Debug)]
pub enum HeaderErr {
    IOErr(io::Error),
    WrongMagic,
    WrongValue(String),
}

impl From<io::Error> for HeaderErr {
    fn from(err: io::Error) -> HeaderErr {
        HeaderErr::IOErr(err)
    }
}

impl fmt::Display for HeaderErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HeaderErr::IOErr(e) => write!(f, "{}", e),
            HeaderErr::WrongMagic => write!(f, "Header doesn't contain GFWX magic"),
            HeaderErr::WrongValue(e) => write!(f, "Invalid filed value in header: {}", e),
        }
    }
}

impl Error for HeaderErr {}
