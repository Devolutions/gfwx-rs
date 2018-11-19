use config::Config;
use processing::{process_maybe_parallel_for_each, OverlappingChunksIterator};

pub fn lift_linear(image: &mut [&mut [i16]]) {
    let mut step = 1;
    let hint_do_parallel =
        image.len() * image[0].len() > Config::multithreading_factors().linear_horizontal_lifting;

    while (step < image[0].len()) || (step < image.len()) {
        if step < image[0].len() {
            process_maybe_parallel_for_each(
                image.into_iter().step_by(step),
                |col| unsafe { horizontal_lift(col, step) },
                hint_do_parallel,
            );
        }

        if step < image.len() {
            unsafe { vertical_lift(image, step) }
        };

        step *= 2;
    }
}

pub fn unlift_linear(image: &mut [&mut [i16]]) {
    let mut step = 1;
    let hint_do_parallel =
        image.len() * image[0].len() > Config::multithreading_factors().linear_horizontal_lifting;

    while (2 * step < image[0].len()) || (2 * step < image.len()) {
        step *= 2;
    }

    while step > 0 {
        if step < image.len() {
            unsafe { vertical_unlift(image, step) };
        };

        if step < image[0].len() {
            process_maybe_parallel_for_each(
                image.into_iter().step_by(step),
                |mut col| unsafe { horizontal_unlift(&mut col, step) },
                hint_do_parallel,
            );
        }

        step /= 2;
    }
}

unsafe fn horizontal_lift(column: &mut [i16], step: usize) {
    let mut x = step;
    while x < column.len() - step {
        let a = *column.get_unchecked_mut(x - step);
        let b = *column.get_unchecked_mut(x + step);
        *column.get_unchecked_mut(x) -= (a + b) / 2;
        x += step * 2;
    }

    if x < column.len() {
        *column.get_unchecked_mut(x) -= *column.get_unchecked_mut(x - step);
    }

    let mut x = 2 * step;
    while x < column.len() - step {
        let a = *column.get_unchecked_mut(x - step);
        let b = *column.get_unchecked_mut(x + step);
        *column.get_unchecked_mut(x) += (a + b) / 4;
        x += step * 2;
    }

    if x < column.len() {
        *column.get_unchecked_mut(x) += *column.get_unchecked_mut(x - step) / 2;
    }
}

unsafe fn horizontal_unlift(column: &mut [i16], step: usize) {
    let mut x = 2 * step;
    while x < column.len() - step {
        let a = *column.get_unchecked_mut(x - step);
        let b = *column.get_unchecked_mut(x + step);
        *column.get_unchecked_mut(x) -= (a + b) / 4;
        x += step * 2;
    }

    if x < column.len() {
        *column.get_unchecked_mut(x) -= *column.get_unchecked_mut(x - step) / 2;
    }

    let mut x = step;
    while x < column.len() - step {
        let a = *column.get_unchecked_mut(x - step);
        let b = *column.get_unchecked_mut(x + step);
        *column.get_unchecked_mut(x) += (a + b) / 2;
        x += step * 2;
    }

    if x < column.len() {
        *column.get_unchecked_mut(x) += *column.get_unchecked_mut(x - step);
    }
}

unsafe fn vertical_lift(mut image: &mut [&mut [i16]], step: usize) {
    let hint_do_parallel =
        image.len() * image[0].len() > Config::multithreading_factors().linear_vertical_lifting;

    process_maybe_parallel_for_each(
        OverlappingChunksIterator::from_slice(&mut image, step),
        |(left, middle, right)| {
            let middle_value = middle.get_unchecked_mut(0);

            let mut x = 0;
            while x < middle_value.len() {
                let c1 = *left.get_unchecked(0).get_unchecked(x);
                let c2 = if let Some(right_value) = right.first() {
                    *right_value.get_unchecked(x)
                } else {
                    c1
                };

                *middle_value.get_unchecked_mut(x) -= (c1 + c2) / 2;

                x += step;
            }
        },
        hint_do_parallel,
    );

    process_maybe_parallel_for_each(
        OverlappingChunksIterator::from_slice(&mut image[step..], step),
        |(left, middle, right)| {
            let middle_value = middle.get_unchecked_mut(0);

            let mut x = 0;
            while x < middle_value.len() {
                let g1 = *left.get_unchecked(0).get_unchecked(x);
                let g2 = if let Some(right_value) = right.first() {
                    *right_value.get_unchecked(x)
                } else {
                    g1
                };

                *middle_value.get_unchecked_mut(x) += (g1 + g2) / 4;

                x += step;
            }
        },
        hint_do_parallel,
    );
}

unsafe fn vertical_unlift(mut image: &mut [&mut [i16]], step: usize) {
    let hint_do_parallel =
        image.len() * image[0].len() > Config::multithreading_factors().linear_vertical_lifting;

    process_maybe_parallel_for_each(
        OverlappingChunksIterator::from_slice(&mut image[step..], step),
        |(left, middle, right)| {
            let middle_value = middle.get_unchecked_mut(0);

            let mut x = 0;
            while x < middle_value.len() {
                let g1 = *left.get_unchecked(0).get_unchecked(x);
                let g2 = if let Some(right_value) = right.first() {
                    *right_value.get_unchecked(x)
                } else {
                    g1
                };

                *middle_value.get_unchecked_mut(x) -= (g1 + g2) / 4;

                x += step;
            }
        },
        hint_do_parallel,
    );

    process_maybe_parallel_for_each(
        OverlappingChunksIterator::from_slice(&mut image, step),
        |(left, middle, right)| {
            let middle_value = middle.get_unchecked_mut(0);

            let mut x = 0;
            while x < middle_value.len() {
                let c1 = *left.get_unchecked(0).get_unchecked(x);
                let c2 = if let Some(right_value) = right.first() {
                    *right_value.get_unchecked(x)
                } else {
                    c1
                };

                *middle_value.get_unchecked_mut(x) += (c1 + c2) / 2;

                x += step;
            }
        },
        hint_do_parallel,
    );
}
