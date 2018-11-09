use config::Config;
use processing::process_maybe_parallel_for_each;

#[cfg(test)]
mod test;

pub fn quantize(image: &mut [&mut [i16]], mut quality: i32, min_quality: i32, max_quality: i32) {
    let mut skip = 1;
    let hint_do_parallel =
        image.len() * image[0].len() > Config::multithreading_factors().quantization;

    while skip < image[0].len() && skip < image.len() {
        let q = min_quality.max(1).max(quality);

        if q >= max_quality {
            break;
        }

        process_maybe_parallel_for_each(
            image.into_iter().enumerate().step_by(skip),
            |(y, column)| {
                let x_step = if (y & skip) != 0 { skip } else { 2 * skip };
                column
                    .into_iter()
                    .skip(x_step - skip)
                    .step_by(x_step)
                    .for_each(|x| *x = ((*x as i32) * q / max_quality) as i16);
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

    while skip < image[0].len() && skip < image.len() {
        let q = min_quality.max(1).max(quality);

        if q >= max_quality {
            break;
        }

        process_maybe_parallel_for_each(
            image.into_iter().enumerate().step_by(skip),
            |(y, column)| {
                let x_step = if (y & skip) != 0 { skip } else { 2 * skip };
                column
                    .into_iter()
                    .skip(x_step - skip)
                    .step_by(x_step)
                    .for_each(|x| {
                        *x = if *x < 0 {
                            (((*x as i32) * max_quality - (max_quality / 2)) / q) as i16
                        } else if *x > 0 {
                            (((*x as i32) * max_quality + (max_quality / 2)) / q) as i16
                        } else {
                            ((*x as i32) * max_quality / q) as i16
                        }
                    });
            },
            hint_do_parallel,
        );

        skip *= 2;
        quality = max_quality.min(2 * quality);
    }
}
