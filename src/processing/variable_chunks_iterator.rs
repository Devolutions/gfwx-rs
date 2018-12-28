use std::mem;

pub struct VariableChunksIterator<'a, 'b, T> {
    slice: &'a [T],
    chunk_sizes: &'b [usize],
}

impl<'a, 'b, T> VariableChunksIterator<'a, 'b, T> {
    pub fn new(slice: &'a [T], chunk_sizes: &'b [usize]) -> VariableChunksIterator<'a, 'b, T> {
        VariableChunksIterator { slice, chunk_sizes }
    }

    fn get_next_chunk_size(&mut self) -> Option<usize> {
        if self.chunk_sizes.is_empty() {
            return None;
        }

        let mut chunk_sizes: &[usize] = &[];
        mem::swap(&mut self.chunk_sizes, &mut chunk_sizes);
        let (next_chunk_size, chunk_sizes) = chunk_sizes.split_first().unwrap();
        self.chunk_sizes = chunk_sizes;
        Some(*next_chunk_size)
    }

    fn get_next_chunk(&mut self, size: usize) -> Option<&'a [T]> {
        if self.slice.is_empty() {
            None
        } else {
            let mut slice: &[T] = &[];
            mem::swap(&mut self.slice, &mut slice);

            // for last chunk - return remainder
            if slice.len() <= size {
                return Some(slice);
            }

            let (chunk, remainder) = slice.split_at(size);
            self.slice = remainder;

            Some(chunk)
        }
    }
}

impl<'a, 'b, T> Iterator for VariableChunksIterator<'a, 'b, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        let next_chunk_size = if let Some(value) = self.get_next_chunk_size() {
            value
        } else {
            return None;
        };

        self.get_next_chunk(next_chunk_size)
    }
}
