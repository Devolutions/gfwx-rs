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
                |col| horizontal_lift(col, step),
                hint_do_parallel,
            );
        }

        if step < image.len() {
            vertical_lift(image, step);
        }

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
            vertical_unlift(image, step);
        }

        if step < image[0].len() {
            process_maybe_parallel_for_each(
                image.into_iter().step_by(step),
                |mut col| horizontal_unlift(&mut col, step),
                hint_do_parallel,
            );
        }

        step /= 2;
    }
}

fn horizontal_lift(column: &mut [i16], step: usize) {
    let mut x = step;
    for i in (step..column.len() - step).step_by(step * 2) {
        column[i] = column[i].wrapping_sub(add_and_div(column[i - step], column[i + step], 2));
        x = i + step * 2;
    }

    if x < column.len() {
        column[x] = column[x].wrapping_sub(column[x - step]);
    }

    x = 2 * step;
    for i in (2 * step..column.len() - step).step_by(step * 2) {
        column[i] = column[i].wrapping_add(add_and_div(column[i - step], column[i + step], 4));
        x = i + step * 2;
    }

    if x < column.len() {
        column[x] = column[x].wrapping_add(column[x - step] / 2);
    }
}

fn horizontal_unlift(column: &mut [i16], step: usize) {
    let mut x = 2 * step;
    for i in (2 * step..column.len() - step).step_by(step * 2) {
        column[i] = column[i].wrapping_sub(add_and_div(column[i - step], column[i + step], 4));
        x = i + step * 2;
    }

    if x < column.len() {
        column[x] = column[x].wrapping_sub(column[x - step] / 2);
    }

    x = step;
    for i in (step..column.len() - step).step_by(step * 2) {
        column[i] = column[i].wrapping_add(add_and_div(column[i - step], column[i + step], 2));
        x = i + step * 2;
    }

    if x < column.len() {
        column[x] = column[x].wrapping_add(column[x - step]);
    }
}

fn vertical_lift(mut image: &mut [&mut [i16]], step: usize) {
    let hint_do_parallel =
        image.len() * image[0].len() > Config::multithreading_factors().linear_vertical_lifting;

    process_maybe_parallel_for_each(
        OverlappingChunksIterator::from_slice(&mut image, step),
        |(left, middle, right)| {
            for x in (0..middle[0].len()).step_by(step) {
                let c1 = left[0][x];
                let c2 = if let Some(right_value) = right.first() {
                    right_value[x]
                } else {
                    c1
                };

                middle[0][x] -= add_and_div(c1, c2, 2);
            }
        },
        hint_do_parallel,
    );

    process_maybe_parallel_for_each(
        OverlappingChunksIterator::from_slice(&mut image[step..], step),
        |(left, middle, right)| {
            for x in (0..middle[0].len()).step_by(step) {
                let g1 = left[0][x];
                let g2 = if let Some(right_value) = right.first() {
                    right_value[x]
                } else {
                    g1
                };

                middle[0][x] += add_and_div(g1, g2, 4);
            }
        },
        hint_do_parallel,
    );
}

fn vertical_unlift(mut image: &mut [&mut [i16]], step: usize) {
    let hint_do_parallel =
        image.len() * image[0].len() > Config::multithreading_factors().linear_vertical_lifting;

    process_maybe_parallel_for_each(
        OverlappingChunksIterator::from_slice(&mut image[step..], step),
        |(left, middle, right)| {
            for x in (0..middle[0].len()).step_by(step) {
                let g1 = left[0][x];
                let g2 = if let Some(right_value) = right.first() {
                    right_value[x]
                } else {
                    g1
                };

                middle[0][x] -= add_and_div(g1, g2, 4);
            }
        },
        hint_do_parallel,
    );

    process_maybe_parallel_for_each(
        OverlappingChunksIterator::from_slice(&mut image, step),
        |(left, middle, right)| {
            for x in (0..middle[0].len()).step_by(step) {
                let c1 = left[0][x];
                let c2 = if let Some(right_value) = right.first() {
                    right_value[x]
                } else {
                    c1
                };

                middle[0][x] += add_and_div(c1, c2, 2);
            }
        },
        hint_do_parallel,
    );
}

fn add_and_div(a: i16, b: i16, divider: i16) -> i16 {
    ((i32::from(a) + i32::from(b)) / i32::from(divider)) as i16
}
