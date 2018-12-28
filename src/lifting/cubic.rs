use super::{get_hint_do_parallel, lift, unlift};
use crate::config::Config;
use crate::processing::{
    process_maybe_parallel_for_each, DoubleOverlappingChunks, DoubleOverlappingChunksIterator,
};

#[inline(always)]
pub fn lift_cubic(mut image: &mut [&mut [i16]]) {
    lift(
        &mut image,
        Config::multithreading_factors().cubic_horizontal_lifting,
        horizontal_lift,
        vertical_lift,
    );
}

#[inline(always)]
pub fn unlift_cubic(mut image: &mut [&mut [i16]]) {
    unlift(
        &mut image,
        Config::multithreading_factors().cubic_horizontal_lifting,
        horizontal_unlift,
        vertical_unlift,
    );
}

#[inline(always)]
unsafe fn horizontal_lifting_base(
    column: &mut [i16],
    step: usize,
    c0_column_start: usize,
    c2_step_multiplier: usize,
    step_start_multiplier: usize,
    divider: i32,
) {
    let mut c0 = *column.get_unchecked(c0_column_start);
    let mut c1 = c0;
    let mut c2 = if c2_step_multiplier * step < column.len() {
        *column.get_unchecked(c2_step_multiplier * step)
    } else {
        c0
    };

    let mut x = step_start_multiplier * step;
    if column.len() > 3 * step {
        while x < column.len() - 3 * step {
            let c3 = *column.get_unchecked(3 * step + x);
            *column.get_unchecked_mut(x) += (cubic(c0, c1, c2, c3) / divider) as i16;
            c0 = c1;
            c1 = c2;
            c2 = c3;
            x += step * 2;
        }
    }
    while x < column.len() {
        *column.get_unchecked_mut(x) += (cubic(c0, c1, c2, c2) / divider) as i16;
        c0 = c1;
        c1 = c2;
        x += step * 2;
    }
}

#[inline(always)]
unsafe fn vertical_lifting_base(
    chunks: &mut DoubleOverlappingChunks<'_, &mut [i16]>,
    step: usize,
    divider: i32,
) {
    let mut x = 0;
    while x < chunks.middle.get_unchecked(0).len() {
        let c1 = *chunks.left.get_unchecked(0).get_unchecked(x);
        let c2 = if let Some(right_value) = chunks.right.first() {
            *right_value.get_unchecked(x)
        } else {
            c1
        };
        let c0 = if let Some(prev_left_value) = chunks.prev_left.first() {
            *prev_left_value.get_unchecked(x)
        } else {
            c1
        };
        let c3 = if let Some(next_right_value) = chunks.next_right.first() {
            *next_right_value.get_unchecked(x)
        } else {
            c2
        };

        *chunks.middle.get_unchecked_mut(0).get_unchecked_mut(x) +=
            (cubic(c0, c1, c2, c3) / divider) as i16;
        x += step;
    }
}

#[inline(always)]
unsafe fn horizontal_lift(mut column: &mut [i16], step: usize) {
    horizontal_lifting_base(&mut column, step, 0, 2, 1, -1);
    horizontal_lifting_base(&mut column, step, step, 3, 2, 2);
}

#[inline(always)]
unsafe fn horizontal_unlift(mut column: &mut [i16], step: usize) {
    horizontal_lifting_base(&mut column, step, step, 3, 2, -2);
    horizontal_lifting_base(&mut column, step, 0, 2, 1, 1);
}

#[inline(always)]
unsafe fn vertical_lift(mut image: &mut [&mut [i16]], step: usize) {
    let config_factor = Config::multithreading_factors().cubic_vertical_lifting;
    let hint_do_parallel = get_hint_do_parallel(&image, config_factor);
    process_maybe_parallel_for_each(
        DoubleOverlappingChunksIterator::from_slice(&mut image, step),
        |mut chunks| {
            vertical_lifting_base(&mut chunks, step, -1);
        },
        hint_do_parallel,
    );
    process_maybe_parallel_for_each(
        DoubleOverlappingChunksIterator::from_slice(&mut image[step..], step),
        |mut chunks| {
            vertical_lifting_base(&mut chunks, step, 2);
        },
        hint_do_parallel,
    );
}

#[inline(always)]
unsafe fn vertical_unlift(mut image: &mut [&mut [i16]], step: usize) {
    let config_factor = Config::multithreading_factors().cubic_vertical_lifting;
    let hint_do_parallel = get_hint_do_parallel(&image, config_factor);
    process_maybe_parallel_for_each(
        DoubleOverlappingChunksIterator::from_slice(&mut image[step..], step),
        |mut chunks| {
            vertical_lifting_base(&mut chunks, step, -2);
        },
        hint_do_parallel,
    );
    process_maybe_parallel_for_each(
        DoubleOverlappingChunksIterator::from_slice(&mut image, step),
        |mut chunks| {
            vertical_lifting_base(&mut chunks, step, 1);
        },
        hint_do_parallel,
    );
}

fn median(mut a: i32, mut b: i32, mut c: i32) -> i32 {
    use std::mem;

    if a > b {
        mem::swap(&mut a, &mut b);
    }
    if b > c {
        mem::swap(&mut b, &mut c);
    }
    if a > b {
        mem::swap(&mut a, &mut b);
    }

    b
}

fn round_fraction(num: i32, denom: i32) -> i32 {
    if num < 0 {
        (num - denom / 2) / denom
    } else {
        (num + denom / 2) / denom
    }
}

pub fn cubic(c0: i16, c1: i16, c2: i16, c3: i16) -> i32 {
    let c0 = i32::from(c0);
    let c1 = i32::from(c1);
    let c2 = i32::from(c2);
    let c3 = i32::from(c3);
    let num = -c0 + 9 * (c1 + c2) - c3;
    median(round_fraction(num, 16), c1, c2)
}
