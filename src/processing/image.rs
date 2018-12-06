use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
    slice,
};

pub struct Image<'a, T> {
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

type Range = (usize, usize);

impl ImageChunkIteratorState {
    pub fn next_chunk(&mut self) -> Option<(Range, Range, usize, usize)> {
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
            channel * self.width * self.height,
        ))
    }
}

pub struct ImageChunkIteratorMut<'a, T> {
    data: &'a mut [T],
    state: ImageChunkIteratorState,
}

pub struct ImageChunkMut<'a, T> {
    data_ptr: *mut T,
    data_len: usize,
    phantom_data: PhantomData<&'a T>,
    image_width: usize,
    channel_size: usize,
    channel_start: usize,

    pub x_range: (usize, usize),
    pub y_range: (usize, usize),
    pub channel: usize,
    pub step: usize,
}

impl<T> ImageChunkMut<'_, T> {
    pub unsafe fn get_unchecked(&self, y: usize, x: usize) -> &T {
        let index = self.channel_start + y * self.image_width + x;

        debug_assert!(
            self.is_owned_zone(y, x) || !self.is_writeable_zone(y, x),
            "Access to mutable neighbour image chunk parts is not permitted!"
        );
        debug_assert!(
            index < self.channel_start + self.channel_size,
            "Cross-channel indexing is not permitted!"
        );
        debug_assert!(index < self.data_len);

        &*self.data_ptr.add(index)
    }

    pub unsafe fn get_unchecked_mut(&mut self, y: usize, x: usize) -> &mut T {
        let index = self.channel_start + y * self.image_width + x;

        debug_assert!(
            self.is_owned_zone(y, x) && self.is_writeable_zone(y, x),
            "Write to immutable image chunk parts is not permitted!",
        );
        debug_assert!(
            index < self.channel_start + self.channel_size,
            "Cross-channel indexing is not permitted!"
        );
        debug_assert!(index < self.data_len);

        &mut *self.data_ptr.add(index)
    }

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

unsafe impl<T> Send for ImageChunkMut<'_, T> {}
unsafe impl<T> Sync for ImageChunkMut<'_, T> {}

impl<T> Index<(usize, usize)> for ImageChunkMut<'_, T> {
    type Output = T;

    fn index(&self, (y, x): (usize, usize)) -> &Self::Output {
        let index = self.channel_start + y * self.image_width + x;

        assert!(
            self.is_owned_zone(y, x) || !self.is_writeable_zone(y, x),
            "Access to mutable neighbour image chunk parts is not permitted!"
        );
        assert!(
            index < self.channel_start + self.channel_size,
            "Cross-channel indexing is not permitted!"
        );

        unsafe {
            let data = slice::from_raw_parts_mut(self.data_ptr, self.data_len);
            &data[index]
        }
    }
}

impl<T> IndexMut<(usize, usize)> for ImageChunkMut<'_, T> {
    fn index_mut(&mut self, (y, x): (usize, usize)) -> &mut Self::Output {
        let index = self.channel_start + y * self.image_width + x;

        assert!(
            self.is_owned_zone(y, x) && self.is_writeable_zone(y, x),
            "Write to immutable image chunk parts is not permitted!",
        );
        assert!(
            index < self.channel_start + self.channel_size,
            "Cross-channel indexing is not permitted!"
        );

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
            .map(|(x_range, y_range, channel, channel_start)| ImageChunkMut {
                data_ptr: self.data.as_mut_ptr(),
                data_len: self.data.len(),
                phantom_data: PhantomData,
                image_width: self.state.width,
                channel_size: self.state.width * self.state.height,
                channel_start,
                channel,
                x_range,
                y_range,
                step: self.state.step,
            })
    }
}
