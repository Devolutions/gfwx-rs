use bits::BitsWriter;
use processing::image::Image;

use super::*;

#[test]
fn test_unsigned_decode() {
    let expected: Vec<u32> = vec![
        11, 163, 15, 135, 8, 92, 116, 78, 68, 12, 82, 226, 251, 58, 148, 232, 234, 172, 27, 252,
    ];
    let input: [u8; 44] = [
        0, 0, 16, 0, 0, 38, 0, 92, 0, 16, 54, 0, 224, 1, 0, 18, 10, 0, 54, 0, 32, 6, 0, 130, 2, 0,
        57, 0, 5, 0, 5, 108, 0, 34, 0, 160, 113, 0, 84, 2, 0, 158, 0, 176,
    ];
    let mut output: Vec<u32> = vec![];
    {
        let mut slice: &[u8] = &input;
        let mut stream = bits::BitsIOReader::new(&mut slice);
        output.push(unsigned_decode(&mut stream, 0).unwrap());
        output.push(unsigned_decode(&mut stream, 0).unwrap());
        output.push(unsigned_decode(&mut stream, 0).unwrap());
        output.push(unsigned_decode(&mut stream, 0).unwrap());
        output.push(unsigned_decode(&mut stream, 1).unwrap());
        output.push(unsigned_decode(&mut stream, 1).unwrap());
        output.push(unsigned_decode(&mut stream, 1).unwrap());
        output.push(unsigned_decode(&mut stream, 1).unwrap());
        output.push(unsigned_decode(&mut stream, 2).unwrap());
        output.push(unsigned_decode(&mut stream, 2).unwrap());
        output.push(unsigned_decode(&mut stream, 2).unwrap());
        output.push(unsigned_decode(&mut stream, 2).unwrap());
        output.push(unsigned_decode(&mut stream, 3).unwrap());
        output.push(unsigned_decode(&mut stream, 3).unwrap());
        output.push(unsigned_decode(&mut stream, 3).unwrap());
        output.push(unsigned_decode(&mut stream, 3).unwrap());
        output.push(unsigned_decode(&mut stream, 4).unwrap());
        output.push(unsigned_decode(&mut stream, 4).unwrap());
        output.push(unsigned_decode(&mut stream, 4).unwrap());
        output.push(unsigned_decode(&mut stream, 4).unwrap());
    }
    assert_eq!(output, expected);
}

#[test]
fn test_interleaved_code() {
    let expected: Vec<u8> = vec![
        80, 1, 0, 0, 0, 68, 0, 0, 0, 0, 128, 15, 3, 0, 192, 123, 0, 220, 0, 80, 0, 63, 2, 0, 192,
        152, 0, 0, 9, 0, 72, 2, 13, 0, 5, 224, 128, 76, 0, 160, 74, 0, 8, 5, 1, 252, 3, 128, 46, 0,
        124, 80, 0, 128, 10, 224,
    ];
    let mut output = vec![];
    {
        let mut stream = bits::BitsIOWriter::new(&mut output);
        interleaved_code(97, &mut stream, 0).unwrap();
        interleaved_code(79, &mut stream, 0).unwrap();
        interleaved_code(30, &mut stream, 0).unwrap();
        interleaved_code(222, &mut stream, 0).unwrap();
        interleaved_code(151, &mut stream, 1).unwrap();
        interleaved_code(24, &mut stream, 1).unwrap();
        interleaved_code(236, &mut stream, 1).unwrap();
        interleaved_code(254, &mut stream, 1).unwrap();
        interleaved_code(29, &mut stream, 2).unwrap();
        interleaved_code(128, &mut stream, 2).unwrap();
        interleaved_code(21, &mut stream, 2).unwrap();
        interleaved_code(47, &mut stream, 2).unwrap();
        interleaved_code(189, &mut stream, 3).unwrap();
        interleaved_code(65, &mut stream, 3).unwrap();
        interleaved_code(59, &mut stream, 3).unwrap();
        interleaved_code(176, &mut stream, 3).unwrap();
        interleaved_code(75, &mut stream, 4).unwrap();
        interleaved_code(48, &mut stream, 4).unwrap();
        interleaved_code(156, &mut stream, 4).unwrap();
        interleaved_code(75, &mut stream, 4).unwrap();

        stream.flush_write_word().unwrap();
    }
    assert_eq!(output, expected);
}

#[test]
fn test_interleaved_decode() {
    let expected: Vec<i32> = vec![
        17, -29, -105, 64, -56, -106, -29, 38, -54, 23, -42, 68, -18, -102, -113, 90, -114, -106,
        55, -25,
    ];
    let input: [u8; 48] = [
        0, 64, 5, 0, 4, 0, 0, 240, 76, 0, 0, 24, 0, 0, 14, 0, 32, 2, 0, 30, 31, 0, 102, 0, 12, 0,
        5, 0, 24, 174, 0, 128, 0, 128, 29, 0, 48, 13, 0, 130, 4, 0, 146, 0, 0, 137, 14, 80,
    ];
    let mut output: Vec<i32> = vec![];
    {
        let mut slice: &[u8] = &input;
        let mut stream = bits::BitsIOReader::new(&mut slice);
        output.push(interleaved_decode(&mut stream, 0).unwrap());
        output.push(interleaved_decode(&mut stream, 0).unwrap());
        output.push(interleaved_decode(&mut stream, 0).unwrap());
        output.push(interleaved_decode(&mut stream, 0).unwrap());
        output.push(interleaved_decode(&mut stream, 1).unwrap());
        output.push(interleaved_decode(&mut stream, 1).unwrap());
        output.push(interleaved_decode(&mut stream, 1).unwrap());
        output.push(interleaved_decode(&mut stream, 1).unwrap());
        output.push(interleaved_decode(&mut stream, 2).unwrap());
        output.push(interleaved_decode(&mut stream, 2).unwrap());
        output.push(interleaved_decode(&mut stream, 2).unwrap());
        output.push(interleaved_decode(&mut stream, 2).unwrap());
        output.push(interleaved_decode(&mut stream, 3).unwrap());
        output.push(interleaved_decode(&mut stream, 3).unwrap());
        output.push(interleaved_decode(&mut stream, 3).unwrap());
        output.push(interleaved_decode(&mut stream, 3).unwrap());
        output.push(interleaved_decode(&mut stream, 4).unwrap());
        output.push(interleaved_decode(&mut stream, 4).unwrap());
        output.push(interleaved_decode(&mut stream, 4).unwrap());
        output.push(interleaved_decode(&mut stream, 4).unwrap());
    }
    assert_eq!(output, expected);
}

#[test]
fn test_signed_code() {
    let expected: Vec<u8> = vec![
        48, 3, 0, 0, 0, 49, 0, 0, 64, 133, 0, 0, 0, 192, 15, 0, 154, 0, 0, 202, 0, 56, 2, 0, 248,
        4, 0, 83, 0, 192, 63, 0, 40, 4, 128, 66, 249, 0, 48, 1, 174, 2, 192, 134, 0, 120, 1, 102,
        0, 72, 97, 64,
    ];
    let mut output = vec![];
    {
        let mut stream = bits::BitsIOWriter::new(&mut output);
        signed_code(181, &mut stream, 0).unwrap();
        signed_code(180, &mut stream, 0).unwrap();
        signed_code(214, &mut stream, 0).unwrap();
        signed_code(123, &mut stream, 0).unwrap();
        signed_code(106, &mut stream, 1).unwrap();
        signed_code(190, &mut stream, 1).unwrap();
        signed_code(123, &mut stream, 1).unwrap();
        signed_code(33, &mut stream, 1).unwrap();
        signed_code(127, &mut stream, 2).unwrap();
        signed_code(175, &mut stream, 2).unwrap();
        signed_code(242, &mut stream, 2).unwrap();
        signed_code(50, &mut stream, 2).unwrap();
        signed_code(81, &mut stream, 3).unwrap();
        signed_code(217, &mut stream, 3).unwrap();
        signed_code(37, &mut stream, 3).unwrap();
        signed_code(139, &mut stream, 3).unwrap();
        signed_code(41, &mut stream, 4).unwrap();
        signed_code(135, &mut stream, 4).unwrap();
        signed_code(193, &mut stream, 4).unwrap();
        signed_code(68, &mut stream, 4).unwrap();

        stream.flush_write_word().unwrap();
    }
    assert_eq!(output, expected);
}

#[test]
fn test_signed_decode() {
    let expected: Vec<i32> = vec![
        243, 97, 15, 140, 194, 186, 129, 24, 175, 147, 103, 246, 223, 78, 221, 161, 21, 125, 167,
        134,
    ];
    let input: [u8; 52] = [
        147, 0, 0, 0, 128, 21, 0, 192, 1, 0, 224, 4, 168, 2, 0, 8, 0, 40, 2, 0, 4, 1, 96, 10, 0,
        224, 31, 0, 240, 14, 0, 199, 1, 160, 17, 0, 0, 116, 0, 255, 131, 1, 96, 63, 2, 176, 3, 86,
        0, 64, 11, 240,
    ];
    let mut output: Vec<i32> = vec![];
    {
        let mut slice: &[u8] = &input;
        let mut stream = bits::BitsIOReader::new(&mut slice);
        output.push(signed_decode(&mut stream, 0).unwrap());
        output.push(signed_decode(&mut stream, 0).unwrap());
        output.push(signed_decode(&mut stream, 0).unwrap());
        output.push(signed_decode(&mut stream, 0).unwrap());
        output.push(signed_decode(&mut stream, 1).unwrap());
        output.push(signed_decode(&mut stream, 1).unwrap());
        output.push(signed_decode(&mut stream, 1).unwrap());
        output.push(signed_decode(&mut stream, 1).unwrap());
        output.push(signed_decode(&mut stream, 2).unwrap());
        output.push(signed_decode(&mut stream, 2).unwrap());
        output.push(signed_decode(&mut stream, 2).unwrap());
        output.push(signed_decode(&mut stream, 2).unwrap());
        output.push(signed_decode(&mut stream, 3).unwrap());
        output.push(signed_decode(&mut stream, 3).unwrap());
        output.push(signed_decode(&mut stream, 3).unwrap());
        output.push(signed_decode(&mut stream, 3).unwrap());
        output.push(signed_decode(&mut stream, 4).unwrap());
        output.push(signed_decode(&mut stream, 4).unwrap());
        output.push(signed_decode(&mut stream, 4).unwrap());
        output.push(signed_decode(&mut stream, 4).unwrap());
    }
    assert_eq!(output, expected);
}

#[test]
fn test_add_context() {
    let mut sum = 0u32;
    let mut sum2 = 0u32;
    let mut count = 0u32;

    add_context(238, 4, &mut sum, &mut sum2, &mut count);
    assert_eq!((sum, sum2, count), (952, 226576, 4));
    add_context(91, 1, &mut sum, &mut sum2, &mut count);
    assert_eq!((sum, sum2, count), (1043, 234857, 5));
    add_context(168, 1, &mut sum, &mut sum2, &mut count);
    assert_eq!((sum, sum2, count), (1211, 263081, 6));
    add_context(28, 1, &mut sum, &mut sum2, &mut count);
    assert_eq!((sum, sum2, count), (1239, 263865, 7));
    add_context(73, 2, &mut sum, &mut sum2, &mut count);
    assert_eq!((sum, sum2, count), (1385, 274523, 9));
    add_context(138, 1, &mut sum, &mut sum2, &mut count);
    assert_eq!((sum, sum2, count), (1523, 293567, 10));
    add_context(166, 1, &mut sum, &mut sum2, &mut count);
    assert_eq!((sum, sum2, count), (1689, 321123, 11));
    add_context(0, 4, &mut sum, &mut sum2, &mut count);
    assert_eq!((sum, sum2, count), (1689, 321123, 15));
    add_context(172, 3, &mut sum, &mut sum2, &mut count);
    assert_eq!((sum, sum2, count), (2205, 409875, 18));
    add_context(178, 4, &mut sum, &mut sum2, &mut count);
    assert_eq!((sum, sum2, count), (2917, 536611, 22));
    add_context(30, 4, &mut sum, &mut sum2, &mut count);
    assert_eq!((sum, sum2, count), (3037, 540211, 26));
}

#[test]
fn test_get_context() {
    let mut image = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2,
        4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4,
    ];

    let mut chunks = Image::from_slice(&mut image, (10, 5), 1).into_chunks_mut(4, 2);
    unsafe {
        assert_eq!(get_context(&chunks.next().unwrap(), 2, 3), (53, 192));
        assert_eq!(get_context(&chunks.next().unwrap(), 0, 0), (80, 400));
        assert_eq!(get_context(&chunks.next().unwrap(), 3, 3), (16, 16));
        assert_eq!(get_context(&chunks.next().unwrap(), 0, 0), (80, 400));
        assert_eq!(get_context(&chunks.next().unwrap(), 1, 0), (0, 0));
        assert_eq!(get_context(&chunks.next().unwrap(), 2, 0), (48, 144));
    }
}

#[test]
fn test_encode_turbo_nodc_nochroma() {
    let mut image = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2,
        4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5,
        7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7,
    ];

    let mut output: Vec<u8> = vec![];
    let expected: Vec<u8> = vec![
        3, 28, 56, 156, 224, 224, 112, 220, 128, 3, 28, 112, 128, 3, 135, 63, 3, 7, 135, 123, 1,
        28, 224, 128, 192, 129, 227, 252, 192, 129, 195, 61, 254, 0, 14, 112, 128, 131, 195, 113,
        128, 3, 135, 123, 142, 243, 7, 112, 3, 14, 28, 28,
    ];

    {
        let mut chunks = Image::from_slice(&mut image, (10, 8), 1).into_chunks_mut(10, 1);

        let mut stream = bits::BitsIOWriter::new(&mut output);
        encode(
            &chunks.next().unwrap(),
            &mut stream,
            Encoder::Turbo,
            0,
            false,
            false,
        ).unwrap();
        stream.flush_write_word().unwrap();
    }

    assert_eq!(output, expected);
}

#[test]
fn test_encode_turbo_dc_chroma() {
    let mut image = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2,
        4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5,
        7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7,
    ];

    let mut output: Vec<u8> = vec![];
    let expected: Vec<u8> = vec![
        112, 224, 112, 142, 131, 195, 113, 15, 14, 112, 192, 129, 14, 28, 254, 0, 28, 28, 238, 1,
        112, 128, 3, 14, 7, 142, 243, 7, 7, 14, 247, 0, 3, 56, 192, 1, 14, 14, 199, 249, 14, 28,
        238, 1, 206, 31, 192, 1, 56, 112, 112, 56, 0, 0, 0, 12,
    ];

    {
        let mut chunks = Image::from_slice(&mut image, (10, 8), 1).into_chunks_mut(10, 1);

        let mut stream = bits::BitsIOWriter::new(&mut output);
        encode(
            &chunks.next().unwrap(),
            &mut stream,
            Encoder::Turbo,
            0,
            true,
            true,
        ).unwrap();
        stream.flush_write_word().unwrap();
    }

    assert_eq!(output, expected);
}

#[test]
fn test_encode_fast_dc_nochroma() {
    let mut image = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2,
        4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5,
        7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7,
    ];

    let mut output: Vec<u8> = vec![];
    let expected: Vec<u8> = vec![
        4, 64, 64, 140, 24, 51, 39, 0, 97, 192, 192, 96, 59, 143, 115, 101, 22, 142, 226, 148, 119,
        30, 167, 94, 47, 151, 163, 56, 220, 121, 148, 171, 171, 47, 151, 227, 0, 128, 226, 148,
    ];

    {
        let mut chunks = Image::from_slice(&mut image, (10, 8), 1).into_chunks_mut(10, 1);

        let mut stream = bits::BitsIOWriter::new(&mut output);
        encode(
            &chunks.next().unwrap(),
            &mut stream,
            Encoder::Fast,
            0,
            true,
            false,
        ).unwrap();
        stream.flush_write_word().unwrap();
    }

    assert_eq!(output, expected);
}

#[test]
fn test_encode_contextual_nodc_nochroma() {
    let mut image = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2,
        4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5,
        7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7,
    ];

    let mut output: Vec<u8> = vec![];
    let expected: Vec<u8> = vec![
        171, 195, 183, 238, 117, 122, 109, 165, 75, 87, 248, 218, 89, 98, 242, 90, 182, 198, 241,
        173, 163, 158, 152, 188, 239, 140, 227, 43, 202, 168, 39, 78, 0, 0, 128, 118,
    ];

    {
        let mut chunks = Image::from_slice(&mut image, (10, 8), 1).into_chunks_mut(10, 1);

        let mut stream = bits::BitsIOWriter::new(&mut output);
        encode(
            &chunks.next().unwrap(),
            &mut stream,
            Encoder::Contextual,
            0,
            false,
            false,
        ).unwrap();
        stream.flush_write_word().unwrap();
    }

    assert_eq!(output, expected);
}

#[test]
fn test_encode_contextual_dc_chroma() {
    let mut image = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2,
        4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5,
        7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7,
    ];

    let mut output: Vec<u8> = vec![];
    let expected: Vec<u8> = vec![
        14, 223, 186, 143, 233, 181, 149, 174, 93, 225, 107, 215, 137, 201, 107, 45, 26, 199, 183,
        102, 122, 98, 242, 218, 51, 142, 175, 140, 163, 158, 56, 189, 0, 0, 218, 41,
    ];

    {
        let mut chunks = Image::from_slice(&mut image, (10, 8), 1).into_chunks_mut(10, 1);

        let mut stream = bits::BitsIOWriter::new(&mut output);
        encode(
            &chunks.next().unwrap(),
            &mut stream,
            Encoder::Contextual,
            0,
            true,
            true,
        ).unwrap();
        stream.flush_write_word().unwrap();
    }

    assert_eq!(output, expected);
}

#[test]
fn test_encode_shifted_chunk() {
    let mut image = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2,
        4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5,
        7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7,
    ];

    let mut output: Vec<u8> = vec![];
    let expected: Vec<u8> = vec![195, 57, 206, 223, 192, 224, 112, 156];

    {
        let mut chunks = Image::from_slice(&mut image, (10, 8), 1).into_chunks_mut(4, 1);
        // skip 4 chunks
        chunks.next().unwrap();
        chunks.next().unwrap();
        chunks.next().unwrap();
        chunks.next().unwrap();

        let mut stream = bits::BitsIOWriter::new(&mut output);
        encode(
            &chunks.next().unwrap(),
            &mut stream,
            Encoder::Turbo,
            0,
            false,
            false,
        ).unwrap();
        stream.flush_write_word().unwrap();
    }

    assert_eq!(output, expected);
}

#[test]
fn test_encode_contextual_shifted_chunk() {
    let mut image = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2,
        4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5,
        7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7,
    ];

    let mut output: Vec<u8> = vec![];
    let expected: Vec<u8> = vec![212, 198, 106, 159, 0, 0, 0, 101];

    {
        let mut chunks = Image::from_slice(&mut image, (10, 8), 1).into_chunks_mut(4, 1);
        // skip 4 chunks
        chunks.next().unwrap();
        chunks.next().unwrap();
        chunks.next().unwrap();
        chunks.next().unwrap();

        let mut stream = bits::BitsIOWriter::new(&mut output);
        encode(
            &chunks.next().unwrap(),
            &mut stream,
            Encoder::Contextual,
            0,
            false,
            false,
        ).unwrap();
        stream.flush_write_word().unwrap();
    }

    assert_eq!(output, expected);
}

#[test]
fn test_decode_turbo_nodc_nochroma() {
    let expected = vec![
        0, 2, 0, 4, 0, 6, 0, 8, 0, 0, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 0, 4, 0, 6, 0, 8, 0, 0, 0, 2,
        4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 0, 6, 0, 8, 0, 0, 0, 2, 0, 4, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5,
        0, 8, 0, 0, 0, 2, 0, 4, 0, 6, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7,
    ];

    let input: Vec<u8> = vec![
        3, 28, 56, 156, 224, 224, 112, 220, 128, 3, 28, 112, 128, 3, 135, 63, 3, 7, 135, 123, 1,
        28, 224, 128, 192, 129, 227, 252, 192, 129, 195, 61, 254, 0, 14, 112, 128, 131, 195, 113,
        128, 3, 135, 123, 142, 243, 7, 112, 3, 14, 28, 28,
    ];

    let mut output = vec![0; expected.len()];

    {
        // will generate 10x8 chunk, with step equal 1
        let mut chunks = Image::from_slice(&mut output, (10, 8), 1).into_chunks_mut(10, 1);

        let mut slice: &[u8] = &input;
        let mut stream = bits::BitsIOReader::new(&mut slice);
        decode(
            chunks.next().unwrap(),
            &mut stream,
            Encoder::Turbo,
            0,
            false,
            false,
            ).unwrap();
    }

    assert_eq!(output, expected);
}

#[test]
fn test_decode_turbo_dc_chroma() {
    let expected = vec![
        1, 2, 0, 4, 0, 6, 0, 8, 0, 0, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 0, 4, 0, 6, 0, 8, 0, 0, 0, 2,
        4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 0, 6, 0, 8, 0, 0, 0, 2, 0, 4, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5,
        0, 8, 0, 0, 0, 2, 0, 4, 0, 6, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7,
    ];

    let input: Vec<u8> = vec![
        112, 224, 112, 142, 131, 195, 113, 15, 14, 112, 192, 129, 14, 28, 254, 0, 28, 28, 238, 1,
        112, 128, 3, 14, 7, 142, 243, 7, 7, 14, 247, 0, 3, 56, 192, 1, 14, 14, 199, 249, 14, 28,
        238, 1, 206, 31, 192, 1, 56, 112, 112, 56, 0, 0, 0, 12,
    ];

    let mut output = vec![0; expected.len()];

    {
        let mut chunks = Image::from_slice(&mut output, (10, 8), 1).into_chunks_mut(10, 1);

        let mut slice: &[u8] = &input;
        let mut stream = bits::BitsIOReader::new(&mut slice);
        decode(
            chunks.next().unwrap(),
            &mut stream,
            Encoder::Turbo,
            0,
            true,
            true,
        ).unwrap();
    }

    assert_eq!(output, expected);
}

#[test]
fn test_decode_fast_dc_nochroma() {
    let expected = vec![
        1, 2, 0, 4, 0, 6, 0, 8, 0, 0, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 0, 4, 0, 6, 0, 8, 0, 0, 0, 2,
        4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 0, 6, 0, 8, 0, 0, 0, 2, 0, 4, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5,
        0, 8, 0, 0, 0, 2, 0, 4, 0, 6, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7,
    ];

    let input: Vec<u8> = vec![
        4, 64, 64, 140, 24, 51, 39, 0, 97, 192, 192, 96, 59, 143, 115, 101, 22, 142, 226, 148, 119,
        30, 167, 94, 47, 151, 163, 56, 220, 121, 148, 171, 171, 47, 151, 227, 0, 128, 226, 148,
    ];

    let mut output = vec![0; expected.len()];

    {
        let mut chunks = Image::from_slice(&mut output, (10, 8), 1).into_chunks_mut(10, 1);

        let mut slice: &[u8] = &input;
        let mut stream = bits::BitsIOReader::new(&mut slice);

        decode(
            chunks.next().unwrap(),
            &mut stream,
            Encoder::Fast,
            0,
            true,
            false,
        ).unwrap();
    }

    assert_eq!(output, expected);
}

#[test]
fn test_decode_contextual_nodc_nochroma() {
    let expected = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let input: Vec<u8> = vec![
        171, 195, 183, 238, 117, 122, 109, 165, 75, 87, 248, 218, 89, 98, 242, 90, 182, 198, 241,
        173, 163, 158, 152, 188, 239, 140, 227, 43, 202, 168, 39, 78, 0, 0, 128, 118,
    ];

    let mut output = vec![0; expected.len()];

    {
        let mut chunks = Image::from_slice(&mut output, (10, 8), 1).into_chunks_mut(10, 1);

        let mut slice: &[u8] = &input;
        let mut stream = bits::BitsIOReader::new(&mut slice);
        decode(
            chunks.next().unwrap(),
            &mut stream,
            Encoder::Contextual,
            0,
            false,
            false,
        ).unwrap();
    }

    assert_eq!(output, expected);
}

#[test]
fn test_decode_contextual_dc_chroma() {
    let expected = vec![
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let input: Vec<u8> = vec![
        14, 223, 186, 143, 233, 181, 149, 174, 93, 225, 107, 215, 137, 201, 107, 45, 26, 199, 183,
        102, 122, 98, 242, 218, 51, 142, 175, 140, 163, 158, 56, 189, 0, 0, 218, 41,
    ];

    let mut output = vec![0; expected.len()];

    {
        let mut chunks = Image::from_slice(&mut output, (10, 8), 1).into_chunks_mut(10, 1);

        let mut slice: &[u8] = &input;
        let mut stream = bits::BitsIOReader::new(&mut slice);
        decode(
            chunks.next().unwrap(),
            &mut stream,
            Encoder::Contextual,
            0,
            true,
            true,
        ).unwrap();
    }

    assert_eq!(output, expected);
}

#[test]
fn test_decode_shifted_chunk() {
    let expected = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 4, 0, 0, 0, 0, 0, 0, 6, 8, 0, 2, 0, 0,
        0, 0, 0, 0, 0, 3, 0, 4, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 0, 0,
    ];

    let input: Vec<u8> = vec![
        3, 28, 56, 156, 224, 224, 112, 220, 128, 3, 28, 112, 128, 3, 135, 63, 3, 7, 135, 123, 1,
        28, 224, 128, 192, 129, 227, 252, 192, 129, 195, 61, 254, 0, 14, 112, 128, 131, 195, 113,
        128, 3, 135, 123, 142, 243, 7, 112, 3, 14, 28, 28,
    ];

    let mut output = vec![0; expected.len()];

    {
        let mut chunks = Image::from_slice(&mut output, (10, 8), 1).into_chunks_mut(4, 1);
        // skip 4 chunks
        chunks.next().unwrap();
        chunks.next().unwrap();
        chunks.next().unwrap();
        chunks.next().unwrap();

        let mut slice: &[u8] = &input;
        let mut stream = bits::BitsIOReader::new(&mut slice);
        decode(
            chunks.next().unwrap(),
            &mut stream,
            Encoder::Turbo,
            0,
            false,
            false,
        ).unwrap();
    }

    assert_eq!(output, expected);
}

#[test]
fn test_decode_contextual_shifted_chunk() {
    let expected = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let input: Vec<u8> = vec![
        14, 223, 186, 143, 233, 181, 149, 174, 93, 225, 107, 215, 137, 201, 107, 45, 26, 199, 183,
        102, 122, 98, 242, 218, 51, 142, 175, 140, 163, 158, 56, 189, 0, 0, 218, 41,
    ];

    let mut output = vec![0; expected.len()];

    {
        let mut chunks = Image::from_slice(&mut output, (10, 8), 1).into_chunks_mut(4, 1);
        // skip 4 chunks
        chunks.next().unwrap();
        chunks.next().unwrap();
        chunks.next().unwrap();
        chunks.next().unwrap();

        let mut slice: &[u8] = &input;
        let mut stream = bits::BitsIOReader::new(&mut slice);
        decode(
            chunks.next().unwrap(),
            &mut stream,
            Encoder::Contextual,
            0,
            true,
            true,
        ).unwrap();
    }

    assert_eq!(output, expected);
}

#[test]
fn test_decode_multichunk() {
    let expected = vec![
        1, 0, 0, 0, 5, 0, 0, 0, 10, 6, 0, 0, 0, 0, 0, 0, 0, 0, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 1, 0, 0, 25, 1, 0, 1, -11, 4, 0, 0, 0, 0, 0, 1, 0, 0, 0,
        -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0, 0, -2, 0, 0, 0, 1, 2,
    ];

    let input: Vec<u8> = vec![
        14, 223, 186, 143, 233, 181, 149, 174, 93, 225, 107, 215, 137, 201, 107, 45, 26, 199, 183,
        102, 122, 98, 242, 218, 51, 142, 175, 140, 163, 158, 56, 189, 0, 0, 218, 41,
    ];
    let mut input_chunks = input.chunks(4);

    let mut output = vec![0; expected.len()];

    Image::from_slice(&mut output, (10, 8), 1)
        .into_chunks_mut(4, 1)
        .zip(&mut input_chunks)
        .for_each(|(output_chunk, mut input_chunk)| {
            let mut stream = bits::BitsIOReader::new(&mut input_chunk);
            decode(
                output_chunk,
                &mut stream,
                Encoder::Contextual,
                0,
                true,
                true,
            ).unwrap();
        });

    assert_eq!(output, expected);
}

#[test]
#[cfg(feature = "rayon")]
fn test_decode_multichunk_parallel() {
    use rayon::prelude::*;

    let expected = vec![
        1, 0, 0, 0, 5, 0, 0, 0, 10, 6, 0, 0, 0, 0, 0, 0, 0, 0, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 1, 0, 0, 25, 1, 0, 1, -11, 4, 0, 0, 0, 0, 0, 1, 0, 0, 0,
        -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0, 0, -2, 0, 0, 0, 1, 2,
    ];

    let input: Vec<u8> = vec![
        14, 223, 186, 143, 233, 181, 149, 174, 93, 225, 107, 215, 137, 201, 107, 45, 26, 199, 183,
        102, 122, 98, 242, 218, 51, 142, 175, 140, 163, 158, 56, 189, 0, 0, 218, 41,
    ];
    let mut input_chunks = input.chunks(4);

    let mut output = vec![0; expected.len()];

    Image::from_slice(&mut output, (10, 8), 1)
        .into_chunks_mut(4, 1)
        .zip(&mut input_chunks)
        .par_bridge()
        .for_each(|(output_chunk, mut input_chunk)| {
            let mut stream = bits::BitsIOReader::new(&mut input_chunk);
            decode(
                output_chunk,
                &mut stream,
                Encoder::Contextual,
                0,
                true,
                true,
            ).unwrap();
        });

    assert_eq!(output, expected);
}
