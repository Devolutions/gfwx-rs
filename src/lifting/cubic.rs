use config::Config;
use processing::{process_maybe_parallel_for_each, DoubleOverlappingChunksIterator};

pub fn lift_cubic(image: &mut [&mut [i16]]) {
    let mut step = 1;
    let hint_do_parallel =
        image.len() * image[0].len() > Config::multithreading_factors().cubic_horizontal_lifting;

    while (step < image[0].len()) || (step < image.len()) {
        if step < image[0].len() {
            process_maybe_parallel_for_each(
                image.into_iter().step_by(step),
                |mut col| horizontal_lift(&mut col, step),
                hint_do_parallel,
            );
        }

        if step < image.len() {
            vertical_lift(image, step);
        }

        step *= 2;
    }
}

pub fn unlift_cubic(image: &mut [&mut [i16]]) {
    let mut step = 1;
    let hint_do_parallel =
        image.len() * image[0].len() > Config::multithreading_factors().cubic_horizontal_lifting;

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
    let mut c0 = column[0];
    let mut c1 = c0;
    let mut c2 = if 2 * step < column.len() {
        column[2 * step]
    } else {
        c0
    };

    if column.len() > 3 * step {
        for i in (step..column.len() - 3 * step).step_by(step * 2) {
            let c3 = column[3 * step + i];
            column[i] = column[i].wrapping_sub(cubic(c0, c1, c2, c3));
            c0 = c1;
            c1 = c2;
            c2 = c3;
            x = i + step * 2;
        }
    }
    for i in (x..column.len()).step_by(step * 2) {
        column[i] = column[i].wrapping_sub(cubic(c0, c1, c2, c2));
        c0 = c1;
        c1 = c2;
    }

    let mut g0 = column[step];
    let mut g1 = g0;
    let mut g2 = if 3 * step < column.len() {
        column[3 * step]
    } else {
        g0
    };

    x = 2 * step;
    if column.len() > 3 * step {
        for i in (2 * step..column.len() - 3 * step).step_by(step * 2) {
            let g3 = column[3 * step + i];
            column[i] = column[i].wrapping_add(cubic(g0, g1, g2, g3) / 2);
            g0 = g1;
            g1 = g2;
            g2 = g3;
            x = i + step * 2;
        }
    }
    for i in (x..column.len()).step_by(step * 2) {
        column[i] = column[i].wrapping_add(cubic(g0, g1, g2, g2) / 2);
        g0 = g1;
        g1 = g2;
    }
}

fn horizontal_unlift(column: &mut [i16], step: usize) {
    let mut g0 = column[step];
    let mut g1 = g0;
    let mut g2 = if 3 * step < column.len() {
        column[3 * step]
    } else {
        g0
    };

    let mut x = 2 * step;
    if column.len() > 3 * step {
        for i in (2 * step..column.len() - 3 * step).step_by(step * 2) {
            let g3 = column[3 * step + i];
            column[i] = column[i].wrapping_sub(cubic(g0, g1, g2, g3) / 2);
            g0 = g1;
            g1 = g2;
            g2 = g3;
            x = i + step * 2;
        }
    }
    for i in (x..column.len()).step_by(step * 2) {
        column[i] = column[i].wrapping_sub(cubic(g0, g1, g2, g2) / 2);
        g0 = g1;
        g1 = g2;
    }

    let mut c0 = column[0];
    let mut c1 = c0;
    let mut c2 = if 2 * step < column.len() {
        column[2 * step]
    } else {
        c0
    };

    x = step;
    if column.len() > 3 * step {
        for i in (step..column.len() - 3 * step).step_by(step * 2) {
            let c3 = column[3 * step + i];
            column[i] = column[i].wrapping_add(cubic(c0, c1, c2, c3));
            c0 = c1;
            c1 = c2;
            c2 = c3;
            x = i + step * 2;
        }
    }
    for i in (x..column.len()).step_by(step * 2) {
        column[i] = column[i].wrapping_add(cubic(c0, c1, c2, c2));
        c0 = c1;
        c1 = c2;
    }
}

fn vertical_lift(mut image: &mut [&mut [i16]], step: usize) {
    let hint_do_parallel =
        image.len() * image[0].len() > Config::multithreading_factors().cubic_vertical_lifting;

    process_maybe_parallel_for_each(
        DoubleOverlappingChunksIterator::from_slice(&mut image, step),
        |(prev_left, left, middle, right, next_right)| {
            for x in (0..middle[0].len()).step_by(step) {
                let c1 = left[0][x];
                let c2 = if let Some(right_value) = right.first() {
                    right_value[x]
                } else {
                    c1
                };
                let c0 = if let Some(prev_left_value) = prev_left.first() {
                    prev_left_value[x]
                } else {
                    c1
                };
                let c3 = if let Some(next_right_value) = next_right.first() {
                    next_right_value[x]
                } else {
                    c2
                };

                middle[0][x] = middle[0][x].wrapping_sub(cubic(c0, c1, c2, c3));
            }
        },
        hint_do_parallel,
    );

    process_maybe_parallel_for_each(
        DoubleOverlappingChunksIterator::from_slice(&mut image[step..], step),
        |(prev_left, left, middle, right, next_right)| {
            for x in (0..middle[0].len()).step_by(step) {
                let g1 = left[0][x];
                let g2 = if let Some(right_value) = right.first() {
                    right_value[x]
                } else {
                    g1
                };
                let g0 = if let Some(prev_left_value) = prev_left.first() {
                    prev_left_value[x]
                } else {
                    g1
                };
                let g3 = if let Some(next_right_value) = next_right.first() {
                    next_right_value[x]
                } else {
                    g2
                };

                middle[0][x] = middle[0][x].wrapping_add(cubic(g0, g1, g2, g3) / 2);
            }
        },
        hint_do_parallel,
    );
}

fn vertical_unlift(mut image: &mut [&mut [i16]], step: usize) {
    let hint_do_parallel =
        image.len() * image[0].len() > Config::multithreading_factors().cubic_vertical_lifting;

    process_maybe_parallel_for_each(
        DoubleOverlappingChunksIterator::from_slice(&mut image[step..], step),
        |(prev_left, left, middle, right, next_right)| {
            for x in (0..middle[0].len()).step_by(step) {
                let g1 = left[0][x];
                let g2 = if let Some(right_value) = right.first() {
                    right_value[x]
                } else {
                    g1
                };
                let g0 = if let Some(prev_left_value) = prev_left.first() {
                    prev_left_value[x]
                } else {
                    g1
                };
                let g3 = if let Some(next_right_value) = next_right.first() {
                    next_right_value[x]
                } else {
                    g2
                };

                middle[0][x] = middle[0][x].wrapping_sub(cubic(g0, g1, g2, g3) / 2);
            }
        },
        hint_do_parallel,
    );

    process_maybe_parallel_for_each(
        DoubleOverlappingChunksIterator::from_slice(&mut image, step),
        |(prev_left, left, middle, right, next_right)| {
            for x in (0..middle[0].len()).step_by(step) {
                let c1 = left[0][x];
                let c2 = if let Some(right_value) = right.first() {
                    right_value[x]
                } else {
                    c1
                };
                let c0 = if let Some(prev_left_value) = prev_left.first() {
                    prev_left_value[x]
                } else {
                    c1
                };
                let c3 = if let Some(next_right_value) = next_right.first() {
                    next_right_value[x]
                } else {
                    c2
                };

                middle[0][x] = middle[0][x].wrapping_add(cubic(c0, c1, c2, c3));
            }
        },
        hint_do_parallel,
    );
}

fn median(mut a: i16, mut b: i16, mut c: i16) -> i16 {
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

    return b;
}

fn round_fraction(num: i32, denom: i32) -> i16 {
    (if num < 0 {
        (num - denom / 2) / denom
    } else {
        (num + denom / 2) / denom
    }) as i16
}

pub fn cubic(c0: i16, c1: i16, c2: i16, c3: i16) -> i16 {
    let num = -(c0 as i32) + 9 * (c1 as i32 + c2 as i32) - c3 as i32;
    median(round_fraction(num, 16), c1, c2)
}
