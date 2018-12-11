use super::{get_hint_do_parallel, lift, unlift};
use crate::config::Config;
use crate::processing::{process_maybe_parallel_for_each, OverlappingChunksIterator};

#[inline(always)]
pub fn lift_linear(mut image: &mut [&mut [i16]]) {
    lift(
        &mut image,
        Config::multithreading_factors().linear_horizontal_lifting,
        horizontal_lift,
        vertical_lift,
    );
}

#[inline(always)]
pub fn unlift_linear(mut image: &mut [&mut [i16]]) {
    unlift(
        &mut image,
        Config::multithreading_factors().linear_horizontal_lifting,
        horizontal_unlift,
        vertical_unlift,
    );
}

#[inline(always)]
unsafe fn horizontal_lifting_base(
    column: &mut [i16],
    step: usize,
    step_multiplier: usize,
    divider: i16,
) {
    let mut x = step * step_multiplier;
    while x < column.len() - step {
        let a = *column.get_unchecked_mut(x - step);
        let b = *column.get_unchecked_mut(x + step);
        *column.get_unchecked_mut(x) += (a + b) / (divider * 2);
        x += step * 2;
    }

    if x < column.len() {
        *column.get_unchecked_mut(x) +=
            (i32::from(*column.get_unchecked_mut(x - step)) / i32::from(divider)) as i16;
    }
}

#[inline(always)]
unsafe fn horizontal_lift(mut column: &mut [i16], step: usize) {
    horizontal_lifting_base(&mut column, step, 1, -1);
    horizontal_lifting_base(&mut column, step, 2, 2);
}

#[inline(always)]
unsafe fn horizontal_unlift(mut column: &mut [i16], step: usize) {
    horizontal_lifting_base(&mut column, step, 2, -2);
    horizontal_lifting_base(&mut column, step, 1, 1);
}

#[inline(always)]
unsafe fn vertical_lifting_base(
    left: &[&mut [i16]],
    middle: &mut [&mut [i16]],
    right: &[&mut [i16]],
    step: usize,
    divider: i16,
) {
    let middle_value = middle.get_unchecked_mut(0);

    let mut x = 0;
    while x < middle_value.len() {
        let c1 = *left.get_unchecked(0).get_unchecked(x);
        let c2 = if let Some(right_value) = right.first() {
            *right_value.get_unchecked(x)
        } else {
            c1
        };

        *middle_value.get_unchecked_mut(x) += (c1 + c2) / divider;

        x += step;
    }
}

#[inline(always)]
unsafe fn vertical_lift(mut image: &mut [&mut [i16]], step: usize) {
    let config_factor = Config::multithreading_factors().linear_vertical_lifting;
    let hint_do_parallel = get_hint_do_parallel(&image, config_factor);

    process_maybe_parallel_for_each(
        OverlappingChunksIterator::from_slice(&mut image, step),
        |(left, middle, right)| {
            vertical_lifting_base(left, middle, right, step, -2);
        },
        hint_do_parallel,
    );

    process_maybe_parallel_for_each(
        OverlappingChunksIterator::from_slice(&mut image[step..], step),
        |(left, middle, right)| {
            vertical_lifting_base(left, middle, right, step, 4);
        },
        hint_do_parallel,
    );
}

#[inline(always)]
unsafe fn vertical_unlift(mut image: &mut [&mut [i16]], step: usize) {
    let config_factor = Config::multithreading_factors().linear_vertical_lifting;
    let hint_do_parallel = get_hint_do_parallel(&image, config_factor);

    process_maybe_parallel_for_each(
        OverlappingChunksIterator::from_slice(&mut image[step..], step),
        |(left, middle, right)| {
            vertical_lifting_base(left, middle, right, step, -4);
        },
        hint_do_parallel,
    );

    process_maybe_parallel_for_each(
        OverlappingChunksIterator::from_slice(&mut image, step),
        |(left, middle, right)| {
            vertical_lifting_base(left, middle, right, step, 2);
        },
        hint_do_parallel,
    );
}
