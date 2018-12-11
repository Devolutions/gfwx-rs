mod cubic;
mod linear;

#[cfg(test)]
mod test;

pub use self::cubic::{lift_cubic, unlift_cubic};
pub use self::linear::{lift_linear, unlift_linear};

use crate::processing::process_maybe_parallel_for_each;

fn lift(
    image: &mut [&mut [i16]],
    config_factor: usize,
    horizontal_lift: unsafe fn(&mut [i16], usize),
    vertical_lift: unsafe fn(&mut [&mut [i16]], usize),
) {
    let mut step = 1;
    let hint_do_parallel = get_hint_do_parallel(&image, config_factor);

    while step < image.len() || step < image[0].len() {
        if step < image[0].len() {
            process_maybe_parallel_for_each(
                image.iter_mut().step_by(step),
                |col| unsafe { horizontal_lift(col, step) },
                hint_do_parallel,
            );
        }

        if step < image.len() {
            unsafe { vertical_lift(image, step) };
        }

        step *= 2;
    }
}

fn unlift(
    image: &mut [&mut [i16]],
    config_factor: usize,
    horizontal_unlift: unsafe fn(&mut [i16], usize),
    vertical_unlift: unsafe fn(&mut [&mut [i16]], usize),
) {
    let hint_do_parallel = get_hint_do_parallel(&image, config_factor);

    let mut step = 1;
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

fn get_hint_do_parallel(image: &[&mut [i16]], config_factor: usize) -> bool {
    image.len() * image[0].len() > config_factor
}
