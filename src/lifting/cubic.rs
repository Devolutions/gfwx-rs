use config::Config;
use processing::{
    process_maybe_parallel_for_each,
    DoubleOverlappingChunks,
    DoubleOverlappingChunksIterator,
};

pub fn lift_cubic(image: &mut [&mut [i16]]) {
    let mut step = 1;
    let hint_do_parallel =
        image.len() * image[0].len() > Config::multithreading_factors().cubic_horizontal_lifting;

    while step < image.len() || step < image[0].len() {
        if step < image[0].len() {
            process_maybe_parallel_for_each(
                image.iter_mut().step_by(step),
                |mut col| unsafe { horizontal_lift(&mut col, step) },
                hint_do_parallel,
            );
        }

        if step < image.len() {
            unsafe { vertical_lift(image, step) };
        }

        step *= 2;
    }
}

pub fn unlift_cubic(image: &mut [&mut [i16]]) {
    let mut step = 1;
    let hint_do_parallel =
        image.len() * image[0].len() > Config::multithreading_factors().cubic_horizontal_lifting;

    while 2 * step < image.len() || 2 * step < image[0].len() {
        step *= 2;
    }

    while step > 0 {
        if step < image.len() {
            unsafe { vertical_unlift(image, step) };
        }

        if step < image[0].len() {
            process_maybe_parallel_for_each(
                image.iter_mut().step_by(step),
                |mut col| unsafe { horizontal_unlift(&mut col, step) },
                hint_do_parallel,
            );
        }

        step /= 2;
    }
}

unsafe fn horizontal_lift(column: &mut [i16], step: usize) {
    let mut c0 = *column.get_unchecked(0);
    let mut c1 = c0;
    let mut c2 = if 2 * step < column.len() {
        *column.get_unchecked(2 * step)
    } else {
        c0
    };

    let mut x = step;
    if column.len() > 3 * step {
        while x < column.len() - 3 * step {
            let c3 = *column.get_unchecked(3 * step + x);
            *column.get_unchecked_mut(x) -= cubic(c0, c1, c2, c3);
            c0 = c1;
            c1 = c2;
            c2 = c3;
            x += step * 2;
        }
    }
    while x < column.len() {
        *column.get_unchecked_mut(x) -= cubic(c0, c1, c2, c2);
        c0 = c1;
        c1 = c2;
        x += step * 2;
    }

    let mut g0 = *column.get_unchecked(step);
    let mut g1 = g0;
    let mut g2 = if 3 * step < column.len() {
        *column.get_unchecked(3 * step)
    } else {
        g0
    };

    let mut x = 2 * step;
    if column.len() > 3 * step {
        while x < column.len() - 3 * step {
            let g3 = *column.get_unchecked(3 * step + x);
            *column.get_unchecked_mut(x) += cubic(g0, g1, g2, g3) / 2;
            g0 = g1;
            g1 = g2;
            g2 = g3;
            x += step * 2;
        }
    }
    while x < column.len() {
        *column.get_unchecked_mut(x) += cubic(g0, g1, g2, g2) / 2;
        g0 = g1;
        g1 = g2;
        x += step * 2;
    }
}

unsafe fn horizontal_unlift(column: &mut [i16], step: usize) {
    let mut g0 = *column.get_unchecked(step);
    let mut g1 = g0;
    let mut g2 = if 3 * step < column.len() {
        *column.get_unchecked(3 * step)
    } else {
        g0
    };

    let mut x = 2 * step;
    if column.len() > 3 * step {
        while x < column.len() - 3 * step {
            let g3 = *column.get_unchecked(3 * step + x);
            *column.get_unchecked_mut(x) -= cubic(g0, g1, g2, g3) / 2;
            g0 = g1;
            g1 = g2;
            g2 = g3;
            x += step * 2;
        }
    }
    while x < column.len() {
        *column.get_unchecked_mut(x) -= cubic(g0, g1, g2, g2) / 2;
        g0 = g1;
        g1 = g2;
        x += step * 2;
    }

    let mut c0 = *column.get_unchecked(0);
    let mut c1 = c0;
    let mut c2 = if 2 * step < column.len() {
        *column.get_unchecked(2 * step)
    } else {
        c0
    };

    let mut x = step;
    if column.len() > 3 * step {
        while x < column.len() - 3 * step {
            let c3 = *column.get_unchecked(3 * step + x);
            *column.get_unchecked_mut(x) += cubic(c0, c1, c2, c3);
            c0 = c1;
            c1 = c2;
            c2 = c3;
            x += step * 2;
        }
    }
    while x < column.len() {
        *column.get_unchecked_mut(x) += cubic(c0, c1, c2, c2);
        c0 = c1;
        c1 = c2;
        x += step * 2;
    }
}

unsafe fn vertical_lift(mut image: &mut [&mut [i16]], step: usize) {
    let hint_do_parallel =
        image.len() * image[0].len() > Config::multithreading_factors().cubic_vertical_lifting;

    process_maybe_parallel_for_each(
        DoubleOverlappingChunksIterator::from_slice(&mut image, step),
        |DoubleOverlappingChunks { prev_left, left, middle, right, next_right }| {
            let mut x = 0;
            while x < middle.get_unchecked(0).len() {
                let c1 = *left.get_unchecked(0).get_unchecked(x);
                let c2 = if let Some(right_value) = right.first() {
                    *right_value.get_unchecked(x)
                } else {
                    c1
                };
                let c0 = if let Some(prev_left_value) = prev_left.first() {
                    *prev_left_value.get_unchecked(x)
                } else {
                    c1
                };
                let c3 = if let Some(next_right_value) = next_right.first() {
                    *next_right_value.get_unchecked(x)
                } else {
                    c2
                };

                *middle.get_unchecked_mut(0).get_unchecked_mut(x) -= cubic(c0, c1, c2, c3);
                x += step;
            }
        },
        hint_do_parallel,
    );

    process_maybe_parallel_for_each(
        DoubleOverlappingChunksIterator::from_slice(&mut image[step..], step),
        |DoubleOverlappingChunks { prev_left, left, middle, right, next_right }| {
            let mut x = 0;
            while x < middle.get_unchecked(0).len() {
                let g1 = *left.get_unchecked(0).get_unchecked(x);
                let g2 = if let Some(right_value) = right.first() {
                    *right_value.get_unchecked(x)
                } else {
                    g1
                };
                let g0 = if let Some(prev_left_value) = prev_left.first() {
                    *prev_left_value.get_unchecked(x)
                } else {
                    g1
                };
                let g3 = if let Some(next_right_value) = next_right.first() {
                    *next_right_value.get_unchecked(x)
                } else {
                    g2
                };

                *middle.get_unchecked_mut(0).get_unchecked_mut(x) += cubic(g0, g1, g2, g3) / 2;
                x += step;
            }
        },
        hint_do_parallel,
    );
}

unsafe fn vertical_unlift(mut image: &mut [&mut [i16]], step: usize) {
    let hint_do_parallel =
        image.len() * image[0].len() > Config::multithreading_factors().cubic_vertical_lifting;

    process_maybe_parallel_for_each(
        DoubleOverlappingChunksIterator::from_slice(&mut image[step..], step),
        |DoubleOverlappingChunks { prev_left, left, middle, right, next_right }| {
            let mut x = 0;
            while x < middle.get_unchecked(0).len() {
                let g1 = *left.get_unchecked(0).get_unchecked(x);
                let g2 = if let Some(right_value) = right.first() {
                    *right_value.get_unchecked(x)
                } else {
                    g1
                };
                let g0 = if let Some(prev_left_value) = prev_left.first() {
                    *prev_left_value.get_unchecked(x)
                } else {
                    g1
                };
                let g3 = if let Some(next_right_value) = next_right.first() {
                    *next_right_value.get_unchecked(x)
                } else {
                    g2
                };

                *middle.get_unchecked_mut(0).get_unchecked_mut(x) -= cubic(g0, g1, g2, g3) / 2;
                x += step;
            }
        },
        hint_do_parallel,
    );

    process_maybe_parallel_for_each(
        DoubleOverlappingChunksIterator::from_slice(&mut image, step),
        |DoubleOverlappingChunks { prev_left, left, middle, right, next_right }| {
            let mut x = 0;
            while x < middle.get_unchecked(0).len() {
                let c1 = *left.get_unchecked(0).get_unchecked(x);
                let c2 = if let Some(right_value) = right.first() {
                    *right_value.get_unchecked(x)
                } else {
                    c1
                };
                let c0 = if let Some(prev_left_value) = prev_left.first() {
                    *prev_left_value.get_unchecked(x)
                } else {
                    c1
                };
                let c3 = if let Some(next_right_value) = next_right.first() {
                    *next_right_value.get_unchecked(x)
                } else {
                    c2
                };

                *middle.get_unchecked_mut(0).get_unchecked_mut(x) += cubic(c0, c1, c2, c3);
                x += step;
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

    b
}

fn round_fraction(num: i32, denom: i32) -> i16 {
    (if num < 0 {
        (num - denom / 2) / denom
    } else {
        (num + denom / 2) / denom
    }) as i16
}

pub fn cubic(c0: i16, c1: i16, c2: i16, c3: i16) -> i16 {
    let num = -i32::from(c0) + 9 * (i32::from(c1) + i32::from(c2)) - i32::from(c3);
    median(round_fraction(num, 16), c1, c2)
}
