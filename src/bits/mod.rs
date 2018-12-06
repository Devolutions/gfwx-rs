use std::io;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

#[cfg(test)]
mod test;

pub trait BitsWriter {
    fn put_bits(&mut self, x: u32, bits: u32) -> io::Result<()>;
    fn flush_write_word(&mut self) -> io::Result<()>;
}

pub trait BitsReader {
    fn get_bits(&mut self, bits: u32) -> io::Result<u32>;
    fn get_zeros(&mut self, max_zeros: u32) -> io::Result<u32>;
    fn flush_read_word(&mut self);
}

pub struct BitsIOWriter<'a, W: WriteBytesExt> {
    write_stream: &'a mut W,
    write_cache: u32,
    index_bits: u32,
}

impl<'a, W: WriteBytesExt> BitsIOWriter<'a, W> {
    pub fn new(stream: &'a mut W) -> Self {
        BitsIOWriter {
            write_stream: stream,
            write_cache: 0,
            index_bits: 0,
        }
    }
}

impl<W: WriteBytesExt> BitsWriter for BitsIOWriter<'_, W> {
    fn put_bits(&mut self, x: u32, bits: u32) -> io::Result<()> {
        let mut new_bits = self.index_bits + bits;

        if new_bits < 32 {
            self.write_cache = (self.write_cache << bits) | x;
        } else {
            new_bits -= 32;

            self.write_stream.write_u32::<LittleEndian>(
                (self.write_cache << (bits - new_bits)) | (x >> new_bits),
            )?;

            self.write_cache = x;
        }

        self.index_bits = new_bits;

        Ok(())
    }

    fn flush_write_word(&mut self) -> io::Result<()> {
        let index_bits = self.index_bits;
        self.put_bits(0, (32 - index_bits) % 32)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct BitsIOReader<'a, R: ReadBytesExt> {
    read_stream: &'a mut R,
    read_cache: u32,
    index_bits: u32,
    cache_filled: bool,
}

impl<'a, R: ReadBytesExt> BitsIOReader<'a, R> {
    pub fn new(stream: &'a mut R) -> Self {
        BitsIOReader {
            read_cache: 0xff_ff_ff_ff,
            read_stream: stream,
            index_bits: 0,
            cache_filled: false,
        }
    }

    fn next_u32(&mut self) -> io::Result<u32> {
        match self.read_stream.read_u32::<LittleEndian>() {
            Ok(n) => {
                self.cache_filled = true;
                Ok(n)
            }
            Err(e) => {
                self.cache_filled = false;
                Err(e)
            }
        }
    }
}

impl<'a, R: ReadBytesExt> BitsReader for BitsIOReader<'a, R> {
    fn get_bits(&mut self, bits: u32) -> io::Result<u32> {
        let mut new_bits = self.index_bits + bits;
        if !self.cache_filled {
            self.read_cache = self.next_u32()?;
        }

        let mut x = self.read_cache << self.index_bits;

        if new_bits >= 32 {
            if new_bits != 32 {
                let prev_index_bits = self.index_bits;
                self.read_cache = self.next_u32()?;
                x |= self.read_cache >> (32 - prev_index_bits);
            } else {
                // read whole word
                self.cache_filled = false;
            }
            new_bits -= 32;
        }
        self.index_bits = new_bits;

        Ok(x >> (32 - bits))
    }

    fn get_zeros(&mut self, max_zeros: u32) -> io::Result<u32> {
        let mut new_bits = self.index_bits;
        if !self.cache_filled {
            self.read_cache = self.next_u32()?;
        }

        let mut b = self.read_cache;
        let mut x = 0;
        loop {
            if new_bits == 31 {
                let lsb_set = (b & 1u32) != 0;
                if !lsb_set {
                    x += 1;
                }
                if lsb_set || x == max_zeros {
                    self.index_bits = 0;
                    self.cache_filled = false;
                    return Ok(x);
                }

                self.read_cache = self.next_u32()?;
                b = self.read_cache;
                new_bits = 0;
                continue;
            }

            let msb_set = ((b << new_bits) & (1u32 << 31)) != 0;
            if !msb_set {
                x += 1;
            }
            if msb_set || x == max_zeros {
                self.index_bits = new_bits + 1;
                return Ok(x);
            }

            new_bits += 1;
        }
    }

    fn flush_read_word(&mut self) {
        self.cache_filled = false;
        self.index_bits = 0;
    }
}
