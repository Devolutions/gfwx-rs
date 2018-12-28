use crate::config::Config;
use crate::processing::process_maybe_parallel_for_each;

#[cfg(test)]
mod test;

pub fn quantize(image: &mut [&mut [i16]], mut quality: i32, min_quality: i32, max_quality: i32) {
    let mut skip = 1;
    let hint_do_parallel =
        image.len() * image[0].len() > Config::multithreading_factors().quantization;

    while skip < image.len() && skip < image[0].len() {
        let q = min_quality.max(1).max(quality);

        if q >= max_quality {
            break;
        }

        process_maybe_parallel_for_each(
            image.iter_mut().enumerate().step_by(skip),
            |(y, column)| {
                let x_step = if (y & skip) != 0 { skip } else { 2 * skip };
                for x in column.iter_mut().skip(x_step - skip).step_by(x_step) {
                    *x = (i32::from(*x) * q).wrapping_div(max_quality) as i16;
                }
            },
            hint_do_parallel,
        );

        skip *= 2;
        quality = max_quality.min(2 * quality);
    }
}

pub fn dequantize(image: &mut [&mut [i16]], mut quality: i32, min_quality: i32, max_quality: i32) {
    let mut skip = 1;
    let hint_do_parallel =
        image.len() * image[0].len() > Config::multithreading_factors().quantization;

    while skip < image.len() && skip < image[0].len() {
        let q = min_quality.max(1).max(quality);

        if q >= max_quality {
            break;
        }

        process_maybe_parallel_for_each(
            image.iter_mut().enumerate().step_by(skip),
            |(y, column)| {
                let x_step = if (y & skip) != 0 { skip } else { 2 * skip };
                for x in column.iter_mut().skip(x_step - skip).step_by(x_step) {
                    *x = if *x < 0 {
                        (i32::from(*x) * max_quality - (max_quality / 2)).wrapping_div(q) as i16
                    } else if *x > 0 {
                        (i32::from(*x) * max_quality + (max_quality / 2)).wrapping_div(q) as i16
                    } else {
                        (i32::from(*x) * max_quality).wrapping_div(q) as i16
                    }
                }
            },
            hint_do_parallel,
        );

        skip *= 2;
        quality = max_quality.min(2 * quality);
    }
}
