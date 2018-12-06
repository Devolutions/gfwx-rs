use std::mem;

pub struct DoubleOverlappingChunks<'a, T> {
    pub prev_left: &'a [T],
    pub left: &'a [T],
    pub middle: &'a mut [T],
    pub right: &'a [T],
    pub next_right: &'a [T],
}

pub struct DoubleOverlappingChunksIterator<'a, T> {
    prev_left: &'a [T],
    left: &'a [T],
    middle: &'a mut [T],
    right: &'a [T],
    remainder: &'a mut [T],
}

impl<'a, T> DoubleOverlappingChunksIterator<'a, T> {
    pub fn from_slice(slice: &'a mut [T], step: usize) -> DoubleOverlappingChunksIterator<'a, T> {
        if slice.len() <= step {
            // valid, but no elements will be generated by iterator
            return DoubleOverlappingChunksIterator {
                prev_left: &[],
                left: &[],
                middle: &mut [],
                right: &[],
                remainder: &mut [],
            };
        }

        let (left, remainder) = slice.split_at_mut(step);

        if remainder.len() <= step {
            return DoubleOverlappingChunksIterator {
                prev_left: &[],
                left,
                middle: remainder,
                right: &[],
                remainder: &mut [],
            };
        }

        let (middle, remainder) = remainder.split_at_mut(step);

        if remainder.len() <= step {
            return DoubleOverlappingChunksIterator {
                prev_left: &[],
                left,
                middle,
                right: remainder,
                remainder: &mut [],
            };
        }

        let (right, remainder) = remainder.split_at_mut(step);

        DoubleOverlappingChunksIterator {
            prev_left: &[],
            left,
            middle,
            right,
            remainder,
        }
    }
}

impl<'a, T> Iterator for DoubleOverlappingChunksIterator<'a, T> {
    type Item = DoubleOverlappingChunks<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let step = self.left.len();

        if self.middle.is_empty() {
            return None;
        }

        let mut middle: &'a mut [T] = &mut [];
        mem::swap(&mut self.middle, &mut middle);

        if self.remainder.is_empty() {
            return Some(DoubleOverlappingChunks {
                prev_left: self.prev_left,
                left: self.left,
                middle,
                right: self.right,
                next_right: &[],
            });
        }

        let mut remainder: &'a mut [T] = &mut [];
        mem::swap(&mut self.remainder, &mut remainder);

        let (next_middle, remainder) = if remainder.len() <= step {
            let empty_slice: &'a mut [T] = &mut [];
            (remainder, empty_slice)
        } else {
            remainder.split_at_mut(step)
        };

        let (next_right, remainder) = if remainder.len() <= step {
            let empty_slice: &'a mut [T] = &mut [];
            (remainder, empty_slice)
        } else {
            remainder.split_at_mut(step)
        };

        let result = Some(DoubleOverlappingChunks {
            prev_left: self.prev_left,
            left: self.left,
            middle,
            right: self.right,
            next_right: next_right as &'a [T],
        });

        self.prev_left = self.left;
        self.left = self.right;
        self.right = next_right;
        self.middle = next_middle;
        self.remainder = remainder;

        result
    }
}
