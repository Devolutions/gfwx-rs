use num_traits;

use bits;
use header::Encoder;
use processing::image::ImageChunkMut;

#[cfg(test)]
mod test;

// limited length power-of-two Golomb-Rice code
pub fn unsigned_code(x: u32, stream: &mut impl bits::BitsWriter, pot: i32) {
    let y = x >> pot;
    if y >= 12 {
        stream.put_bits(0, 12); // escape to larger code
        let new_pot = if pot < 20 { pot + 4 } else { 24 };
        unsigned_code(x - (12 << (pot)), stream, new_pot);
    } else {
        // encode x / 2^pot in unary followed by x % 2^pot in binary
        stream.put_bits((1 << (pot)) | (x & !(!0u32 << (pot))), y as i32 + 1 + pot);
    }
}

pub fn unsigned_decode(stream: &mut impl bits::BitsReader, pot: i32) -> u32 {
    let x = stream.get_zeros(12);
    let p = pot.min(24); // actual pot. The max 108 below is to prevent unlimited recursion in malformed files, yet admit 2^32 - 1.
    if pot < 108 && x == 12 {
        let new_pot = (pot + 4).min(108);
        (12u32 << p).wrapping_add(unsigned_decode(stream, new_pot))
    } else if p != 0 {
        (x << p).wrapping_add(stream.get_bits(p))
    } else {
        x
    }
}

pub fn interleaved_code(x: i32, stream: &mut impl bits::BitsWriter, pot: i32) {
    let x = if x <= 0 { -2 * x } else { 2 * x - 1 } as u32;
    unsigned_code(x, stream, pot);
}

pub fn interleaved_decode(stream: &mut impl bits::BitsReader, pot: i32) -> i32 {
    let x = unsigned_decode(stream, pot) as i32;
    if (x & 1) != 0 {
        (x + 1) / 2
    } else {
        -x / 2
    }
}

pub fn signed_code(x: i32, stream: &mut impl bits::BitsWriter, pot: i32) {
    unsigned_code(x.abs() as u32, stream, pot);
    if x != 0 {
        stream.put_bits(if x > 0 { 1 } else { 0 }, 1);
    }
}

pub fn signed_decode(stream: &mut impl bits::BitsReader, pot: i32) -> i32 {
    let x = unsigned_decode(stream, pot) as i32;

    if x == 0 {
        return 0;
    }

    if stream.get_bits(1) != 0 {
        x
    } else {
        -x
    }
}

pub fn square<T: num_traits::ops::wrapping::WrappingMul>(t: T) -> T::Output {
    t.wrapping_mul(&t)
}

pub fn add_context(x: i16, w: i32, sum: &mut u32, sum2: &mut u32, count: &mut u32) {
    let x = x.abs() as u32;
    *sum += x * w as u32;
    *sum2 += square(x.min(4096)) * w as u32;
    *count += w as u32;
}

pub fn get_context(image: &ImageChunkMut<i16>, x: i32, y: i32) -> (u32, u32) {
    let skip = image.step as i32;
    let x_range = (image.x_range.0 as i32, image.x_range.1 as i32);
    let y_range = (image.y_range.0 as i32, image.y_range.1 as i32);

    let mut px = x_range.0 + (x & !(skip * 2)) + (x & skip);
    if px >= x_range.1 {
        px -= skip * 2;
    }

    let mut py = y_range.0 + (y & !(skip * 2)) + (y & skip);
    if py >= y_range.1 {
        py -= skip * 2;
    }

    let mut count = 0u32;
    let mut sum = 0u32;
    let mut sum2 = 0u32;

    add_context(
        image[(py as usize, px as usize)],
        2,
        &mut sum,
        &mut sum2,
        &mut count,
    ); // ancestor
    if (y & skip) != 0 && (x | skip) < (x_range.1 - x_range.0) {
        add_context(
            image[(
                (y_range.0 + y - skip) as usize,
                (x_range.0 + (x | skip)) as usize,
            )],
            2,
            &mut sum,
            &mut sum2,
            &mut count,
        ); // upper sibling
        if (x & skip) != 0 {
            add_context(
                image[((y_range.0 + y) as usize, (x_range.0 + x - skip) as usize)],
                2,
                &mut sum,
                &mut sum2,
                &mut count,
            ); // left sibling
        }
    }
    if y >= skip * 2 && x >= skip * 2 {
        // neighbors
        let points = [
            (y_range.0 + y - skip * 2, x_range.0 + x, 4),
            (y_range.0 + y, x_range.0 + x - skip * 2, 4),
            (y_range.0 + y - skip * 2, x_range.0 + x - skip * 2, 2),
        ];
        for p in &points {
            add_context(
                image[(p.0 as usize, p.1 as usize)],
                p.2,
                &mut sum,
                &mut sum2,
                &mut count,
            );
        }
        if x + skip * 2 < x_range.1 - x_range.0 {
            add_context(
                image[(
                    (y_range.0 + y - skip * 2) as usize,
                    (x_range.0 + x + skip * 2) as usize,
                )],
                2,
                &mut sum,
                &mut sum2,
                &mut count,
            );
        }
        if y >= skip * 4 && x >= skip * 4 {
            let points = [
                (y_range.0 + y - skip * 4, x_range.0 + x, 2),
                (y_range.0 + y, x_range.0 + x - skip * 4, 2),
                (y_range.0 + y - skip * 4, x_range.0 + x - skip * 4, 1),
            ];
            for p in &points {
                add_context(
                    image[(p.0 as usize, p.1 as usize)],
                    p.2,
                    &mut sum,
                    &mut sum2,
                    &mut count,
                );
            }
            if x + skip * 4 < x_range.1 - x_range.0 {
                add_context(
                    image[(
                        (y_range.0 + y - skip * 4) as usize,
                        (x_range.0 + x + skip * 4) as usize,
                    )],
                    1,
                    &mut sum,
                    &mut sum2,
                    &mut count,
                );
            }
        }
    }

    (
        (sum * 16 + count / 2) / count,
        (sum2 * 16 + count / 2) / count,
    )
}

fn encode_s(
    stream: &mut impl bits::BitsWriter,
    s: i32,
    sum_sq: u32,
    context: (u32, u32),
    is_chroma: bool,
) {
    if sum_sq < 2 * context.1 + (if is_chroma { 250 } else { 100 }) {
        interleaved_code(s, stream, 0);
    } else if sum_sq < 2 * context.1 + 950 {
        interleaved_code(s, stream, 1);
    } else if sum_sq < 3 * context.1 + 3000 {
        if sum_sq < 5 * context.1 + 400 {
            signed_code(s, stream, 1);
        } else {
            interleaved_code(s, stream, 2);
        }
    } else if sum_sq < 3 * context.1 + 12000 {
        if sum_sq < 5 * context.1 + 3000 {
            signed_code(s, stream, 2);
        } else {
            interleaved_code(s, stream, 3);
        }
    } else if sum_sq < 4 * context.1 + 44000 {
        if sum_sq < 6 * context.1 + 12000 {
            signed_code(s, stream, 3);
        } else {
            interleaved_code(s, stream, 4);
        }
    } else {
        signed_code(s, stream, 4);
    }
}

pub fn encode(
    image: ImageChunkMut<i16>,
    stream: &mut impl bits::BitsWriter,
    scheme: Encoder,
    q: i32,
    has_dc: bool,
    is_chroma: bool,
) {
    let step = image.step as i32;
    let x_range = (image.x_range.0 as i32, image.x_range.1 as i32);
    let y_range = (image.y_range.0 as i32, image.y_range.1 as i32);

    let sizex = (image.x_range.1 - image.x_range.0) as i32;
    let sizey = (image.y_range.1 - image.y_range.0) as i32;

    if has_dc && (sizex > 0) && (sizey > 0) {
        signed_code(
            i32::from(image[(y_range.0 as usize, x_range.0 as usize)]),
            stream,
            4,
        );
    }

    let mut context = (0, 0);
    let mut run = 0i32;
    let mut run_coder =
        if scheme == Encoder::Turbo && ((q == 0) || ((step < 2048) && (q * step < 2048))) {
            1
        } else {
            0i32
        };

    (0..sizey).step_by(step as usize).for_each(|y| {
        let x_step = if (y & step) != 0 { step } else { step * 2 };
        ((x_step - step)..sizex)
            .step_by(x_step as usize)
            .for_each(|x| {
                // [NOTE] arranged so that (x | y) & step == 1
                let mut s = i32::from(image[((y_range.0 + y) as usize, (x_range.0 + x) as usize)]);
                if run_coder != 0 && s == 0 {
                    run += 1;
                } else {
                    if scheme == Encoder::Turbo {
                        if run_coder != 0 {
                            unsigned_code(run as u32, stream, 1);
                            run = 0;
                            // s can't be zero, so shift negatives by 1
                            interleaved_code(if s < 0 { s + 1 } else { s }, stream, 1);
                        } else {
                            interleaved_code(s, stream, 1);
                        }
                        return;
                    }
                    if run_coder != 0 {
                        unsigned_code(run as u32, stream, run_coder);

                        run = 0;
                        if s < 0 {
                            s += 1;
                        }
                    }
                    if scheme == Encoder::Contextual {
                        context = get_context(&image, x, y);
                    }
                    let sum_sq = square(context.0);

                    encode_s(stream, s, sum_sq, context, is_chroma);

                    if scheme == Encoder::Fast {
                        let t = s.abs() as u32;
                        context = (
                            ((context.0 * 15 + 7) >> 4) + t,
                            ((context.1 * 15 + 7) >> 4) + square(t.min(4096)),
                        );
                        run_coder = get_run_coder_fast(context, s, run_coder);
                    } else {
                        run_coder = get_run_coder(context, s, q, run_coder, sum_sq);
                    }
                }
            });
    });
    if run != 0 {
        // flush run
        unsigned_code(run as u32, stream, run_coder);
    }
}

fn get_s(
    stream: &mut impl bits::BitsReader,
    sum_sq: u32,
    context: (u32, u32),
    is_chroma: bool,
) -> i32 {
    if sum_sq < 2 * context.1 + (if is_chroma { 250 } else { 100 }) {
        interleaved_decode(stream, 0)
    } else if sum_sq < 2 * context.1 + 950 {
        interleaved_decode(stream, 1)
    } else if sum_sq < 3 * context.1 + 3000 {
        if sum_sq < 5 * context.1 + 400 {
            signed_decode(stream, 1)
        } else {
            interleaved_decode(stream, 2)
        }
    } else if sum_sq < 3 * context.1 + 12000 {
        if sum_sq < 5 * context.1 + 3000 {
            signed_decode(stream, 2)
        } else {
            interleaved_decode(stream, 3)
        }
    } else if sum_sq < 4 * context.1 + 44000 {
        if sum_sq < 6 * context.1 + 12000 {
            signed_decode(stream, 3)
        } else {
            interleaved_decode(stream, 4)
        }
    } else {
        signed_decode(stream, 4)
    }
}

fn get_run_coder_fast(context: (u32, u32), s: i32, run_coder: i32) -> i32 {
    // use decaying first and second moment
    if (s == 0) == (run_coder == 0) {
        if context.0 < 1 {
            4
        } else if context.0 < 2 {
            3
        } else if context.0 < 4 {
            2
        } else if context.0 < 8 {
            1
        } else {
            0
        }
    } else {
        run_coder
    }
}

fn get_run_coder(context: (u32, u32), s: i32, q: i32, run_coder: i32, sum_sq: u32) -> i32 {
    if (s == 0) == (run_coder == 0) {
        if q == 1024 {
            if context.0 < 2 {
                1
            } else {
                0
            }
        } else if context.0 < 4 && context.1 < 2 {
            4
        } else if context.0 < 8 && context.1 < 4 {
            3
        } else if 2 * sum_sq < 3 * context.1 + 48 {
            2
        } else if 2 * sum_sq < 5 * context.1 + 32 {
            1
        } else {
            0
        }
    } else {
        run_coder
    }
}

pub fn decode(
    mut image: ImageChunkMut<i16>,
    stream: &mut impl bits::BitsReader,
    scheme: Encoder,
    q: i32,
    has_dc: bool,
    is_chroma: bool,
) {
    let step = image.step as i32;
    let x_range = (image.x_range.0 as i32, image.x_range.1 as i32);
    let y_range = (image.y_range.0 as i32, image.y_range.1 as i32);

    let sizex = (image.x_range.1 - image.x_range.0) as i32;
    let sizey = (image.y_range.1 - image.y_range.0) as i32;

    if has_dc && (sizex > 0) && (sizey > 0) {
        image[(y_range.0 as usize, x_range.0 as usize)] = signed_decode(stream, 4) as i16;
    }
    let mut context = (0u32, 0u32);
    let mut run = -1i32;
    let mut run_coder =
        if scheme == Encoder::Turbo && ((q == 0) || ((step < 2048) && (q * step < 2048))) {
            1
        } else {
            0i32
        };

    for y in (0..sizey).step_by(step as usize) {
        let x_step = if (y & step) != 0 { step } else { step * 2 };
        for x in ((x_step - step)..sizex).step_by(x_step as usize) {
            // [NOTE] arranged so that (x | y) & step == 1
            let mut s = 0;
            if run_coder != 0 && run == -1 {
                run = unsigned_decode(stream, run_coder) as i32;
            }
            if run <= 0 {
                if scheme == Encoder::Turbo {
                    s = interleaved_decode(stream, 1);
                } else {
                    if scheme == Encoder::Contextual {
                        context = get_context(&mut image, x as i32, y as i32);
                    }
                    let sum_sq = square(context.0);
                    s = get_s(&mut *stream, sum_sq, context, is_chroma);

                    if scheme == Encoder::Fast {
                        let t = s.abs() as u32;
                        context = (
                            ((context.0 * 15 + 7) >> 4) + t,
                            ((context.1 * 15 + 7) >> 4) + square(t.min(4096)),
                        );
                        run_coder = get_run_coder_fast(context, s, run_coder);
                    } else {
                        run_coder = get_run_coder(context, s, q, run_coder, sum_sq);
                    }
                }
                if run == 0 && s <= 0 {
                    s -= 1; // s can't be zero, so shift negatives by 1
                }
                run = -1;
            } else {
                run -= 1; // consume a zero
            }
            image[((y_range.0 + y) as usize, (x_range.0 + x) as usize)] = s as i16;
        }
    }
}
