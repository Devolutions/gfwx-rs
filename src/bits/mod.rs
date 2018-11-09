use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

#[cfg(test)]
mod test;

pub trait BitsWriter {
    fn put_bits(&mut self, x: u32, bits: i32);
    fn flush_write_word(&mut self);
    fn is_overflow_detected(&self) -> bool;
}

pub trait BitsReader {
    fn get_bits(&mut self, bits: i32) -> u32;
    fn get_zeros(&mut self, max_zeros: u32) -> u32;
    fn flush_read_word(&mut self);
    fn is_underflow_detected(&self) -> bool;
}

pub struct BitsIOWriter<'a, W: 'a + WriteBytesExt> {
    write_stream: &'a mut W,
    write_cache: u32,
    index_bits: i32,
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

impl<'a, W: WriteBytesExt> BitsWriter for BitsIOWriter<'a, W> {
    fn put_bits(&mut self, x: u32, bits: i32) {
        // overflow detected
        if self.index_bits < 0 {
            return;
        }

        let mut new_bits = self.index_bits + bits;

        if new_bits < 32 {
            self.write_cache = (self.write_cache << bits) | x;
        } else if bits == 32 && new_bits == 32 {
            new_bits = 0;
            if self.write_stream.write_u32::<LittleEndian>(x).is_err() {
                self.index_bits = -1;
                return;
            }
        } else {
            new_bits -= 32;

            if self
                .write_stream
                .write_u32::<LittleEndian>(
                    (self.write_cache << (bits - new_bits)) | (x >> new_bits),
                )
                .is_err()
            {
                self.index_bits = -1;
                return;
            }

            self.write_cache = x;
        }

        self.index_bits = new_bits;
    }

    fn flush_write_word(&mut self) {
        let index_bits = self.index_bits;
        self.put_bits(0, (32 - index_bits) % 32);
    }

    fn is_overflow_detected(&self) -> bool {
        self.index_bits < 0
    }
}

#[derive(Debug)]
pub struct BitsIOReader<'a, R: 'a + ReadBytesExt> {
    read_stream: &'a mut R,
    read_cache: u32,
    index_bits: i32,
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

    fn next_u32(&mut self) {
        if self.index_bits >= 0 {
            self.read_cache = match self.read_stream.read_u32::<LittleEndian>() {
                Ok(n) => {
                    self.cache_filled = true;
                    n
                }
                Err(_) => {
                    self.cache_filled = false;
                    self.index_bits = -1;
                    0xffff_ffff
                }
            };
        }
    }
}

impl<'a, R: ReadBytesExt> BitsReader for BitsIOReader<'a, R> {
    fn get_bits(&mut self, bits: i32) -> u32 {
        let mut new_bits = self.index_bits + bits;
        if !self.cache_filled {
            self.next_u32();
        }
        if self.index_bits < 0 {
            // overflow
            return self.index_bits as u32;
        }

        let mut x = self.read_cache << self.index_bits;

        if new_bits >= 32 {
            if new_bits != 32 {
                let prev_index_bits = self.index_bits;
                self.next_u32();
                if self.index_bits < 0 {
                    // overflow detected
                    return self.index_bits as u32;
                }
                x |= self.read_cache >> (32 - prev_index_bits);
            } else {
                // read whole word
                self.cache_filled = false;
            }
            new_bits -= 32;
        }
        self.index_bits = new_bits;

        x >> (32 - bits)
    }

    fn get_zeros(&mut self, max_zeros: u32) -> u32 {
        let mut new_bits = self.index_bits;
        if !self.cache_filled {
            self.next_u32();
        }
        if self.index_bits < 0 {
            // overflow
            return self.index_bits as u32;
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
                    return x;
                }
                self.next_u32();
                if self.index_bits < 0 {
                    return self.index_bits as u32;
                }
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
                return x;
            }

            new_bits += 1;
        }
    }

    fn flush_read_word(&mut self) {
        self.cache_filled = false;
        self.index_bits = 0;
    }

    fn is_underflow_detected(&self) -> bool {
        self.index_bits < 0
    }
}
