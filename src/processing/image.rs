use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
    slice,
};

pub struct Image<'a, T>
where
    T: 'a,
{
    data: &'a mut [T],
    size: (usize, usize),
    channels: usize,
}

impl<'a, T> Image<'a, T> {
    pub fn from_slice(data: &'a mut [T], size: (usize, usize), channels: usize) -> Image<'a, T> {
        Image {
            data,
            size,
            channels,
        }
    }

    pub fn into_chunks_mut(self, chunk_size: usize, step: usize) -> ImageChunkIteratorMut<'a, T> {
        let (width, height) = self.size;

        ImageChunkIteratorMut {
            data: self.data,
            state: ImageChunkIteratorState {
                chunk_size,
                step,
                x: 0,
                y: 0,
                channel: 0,
                width,
                height,
                channels: self.channels,
            },
        }
    }
}

struct ImageChunkIteratorState {
    chunk_size: usize,
    step: usize,
    pub x: usize,
    pub y: usize,
    channel: usize,
    width: usize,
    height: usize,
    channels: usize,
}

impl ImageChunkIteratorState {
    pub fn next_chunk(&mut self) -> Option<((usize, usize), (usize, usize), usize)> {
        if self.channel >= self.channels {
            return None;
        }

        let (x, y) = (self.x, self.y);
        let channel = self.channel;

        self.x += self.chunk_size;

        if self.x >= self.width {
            self.x = 0;
            self.y += self.chunk_size;
        }

        if self.y >= self.height {
            self.y = 0;
            self.channel += 1;
        }

        Some((
            (x, (x + self.chunk_size).min(self.width)),
            (y, (y + self.chunk_size).min(self.height)),
            channel,
        ))
    }
}

pub struct ImageChunkIteratorMut<'a, T>
where
    T: 'a,
{
    data: &'a mut [T],
    state: ImageChunkIteratorState,
}

pub struct ImageChunkMut<'a, T>
where
    T: 'a,
{
    data_ptr: *mut T,
    data_len: usize,
    phantom_data: PhantomData<&'a T>,
    image_width: usize,
    channel_size: usize,

    pub x_range: (usize, usize),
    pub y_range: (usize, usize),
    pub channel: usize,
    pub step: usize,
}

impl<'a, T> ImageChunkMut<'a, T>
where
    T: 'a,
{
    fn is_writeable_zone(&self, y: usize, x: usize) -> bool {
        if y == self.y_range.0 && x == self.x_range.0 {
            return true;
        }

        if y % self.step == 0 {
            let x_step = if (y & self.step) != 0 {
                self.step
            } else {
                self.step * 2
            };
            if (x + (x_step - self.step)) % x_step == 0 {
                return true;
            }
        }

        false
    }

    fn is_owned_zone(&self, y: usize, x: usize) -> bool {
        x >= self.x_range.0 && x < self.x_range.1 && y >= self.y_range.0 && y < self.y_range.1
    }
}

unsafe impl<'a, T> Send for ImageChunkMut<'a, T> {}
unsafe impl<'a, T> Sync for ImageChunkMut<'a, T> {}

impl<'a, T> Index<(usize, usize)> for ImageChunkMut<'a, T> {
    type Output = T;

    fn index(&self, (y, x): (usize, usize)) -> &Self::Output {
        if !self.is_owned_zone(y, x) && self.is_writeable_zone(y, x) {
            panic!("Access to mutable neighbour image chunk parts is not permitted!");
        }

        let index = self.channel * self.channel_size + y * self.image_width + x;
        if index >= self.channel * self.channel_size + self.channel_size {
            panic!("Cross-channel indexing is not permitted!");
        }

        unsafe {
            let data = slice::from_raw_parts_mut(self.data_ptr, self.data_len);
            &data[index]
        }
    }
}

impl<'a, T> IndexMut<(usize, usize)> for ImageChunkMut<'a, T> {
    fn index_mut(&mut self, (y, x): (usize, usize)) -> &mut Self::Output {
        if !self.is_owned_zone(y, x) || !self.is_writeable_zone(y, x) {
            panic!("Write to immutable image chunk parts is not permitted!");
        }

        let index = self.channel * self.channel_size + y * self.image_width + x;
        if index >= self.channel * self.channel_size + self.channel_size {
            panic!("Cross-channel indexing is not permitted!");
        }

        unsafe {
            let data = slice::from_raw_parts_mut(self.data_ptr, self.data_len);
            &mut data[index]
        }
    }
}

impl<'a, T> Iterator for ImageChunkIteratorMut<'a, T> {
    type Item = ImageChunkMut<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.state
            .next_chunk()
            .map(|(x_range, y_range, channel)| ImageChunkMut {
                data_ptr: self.data.as_mut_ptr(),
                data_len: self.data.len(),
                phantom_data: PhantomData,
                image_width: self.state.width,
                channel_size: self.state.width * self.state.height,
                channel,
                x_range,
                y_range,
                step: self.state.step,
            })
    }
}
