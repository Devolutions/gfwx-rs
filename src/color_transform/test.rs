use super::*;

#[test]
fn test_color_transform_program() {
    let mut color_transform_program = ColorTransformProgram::new();

    color_transform_program
        .add_channel_transform(
            ChannelTransformBuilder::new()
                .set_dest_channel(0)
                .add_channel_factor(1, -1)
                .set_chroma()
                .build(),
        )
        .add_channel_transform(
            ChannelTransformBuilder::new()
                .set_dest_channel(2)
                .add_channel_factor(1, -1)
                .set_chroma()
                .build(),
        )
        .add_channel_transform(
            ChannelTransformBuilder::new()
                .set_dest_channel(1)
                .add_channel_factor(0, 1)
                .add_channel_factor(2, 1)
                .set_denominator(4)
                .build(),
        )
        .add_channel_transform(
            ChannelTransformBuilder::new()
                .set_dest_channel(3)
                .set_chroma()
                .build(),
        )
        .add_channel_transform(
            ChannelTransformBuilder::new()
                .set_dest_channel(4)
                .set_denominator(2)
                .build(),
        );

    assert_eq!(true, color_transform_program.is_channel_has_transform(0));
    assert_eq!(true, color_transform_program.is_channel_has_transform(1));
    assert_eq!(true, color_transform_program.is_channel_has_transform(2));
    assert_eq!(false, color_transform_program.is_channel_has_transform(3));
    assert_eq!(true, color_transform_program.is_channel_has_transform(4));

    let mut channel_transform_iter = color_transform_program.iter();

    let channel_transform = channel_transform_iter.next().unwrap();
    assert_eq!(0, channel_transform.dest_channel);
    assert_eq!(true, channel_transform.is_chroma);
    assert_eq!(1, channel_transform.denominator);
    let mut channel_factors_iter = channel_transform.channel_factors.iter();
    let channel_factor = channel_factors_iter.next().unwrap();
    assert_eq!(1, channel_factor.src_channel);
    assert_eq!(-1, channel_factor.factor);
    assert!(channel_factors_iter.next().is_none());

    let channel_transform = channel_transform_iter.next().unwrap();
    assert_eq!(2, channel_transform.dest_channel);
    assert_eq!(true, channel_transform.is_chroma);
    assert_eq!(1, channel_transform.denominator);
    let mut channel_factors_iter = channel_transform.channel_factors.iter();
    let channel_factor = channel_factors_iter.next().unwrap();
    assert_eq!(1, channel_factor.src_channel);
    assert_eq!(-1, channel_factor.factor);
    assert!(channel_factors_iter.next().is_none());

    let channel_transform = channel_transform_iter.next().unwrap();
    assert_eq!(1, channel_transform.dest_channel);
    assert_eq!(false, channel_transform.is_chroma);
    assert_eq!(4, channel_transform.denominator);
    let mut channel_factors_iter = channel_transform.channel_factors.iter();
    let channel_factor = channel_factors_iter.next().unwrap();
    assert_eq!(0, channel_factor.src_channel);
    assert_eq!(1, channel_factor.factor);
    let channel_factor = channel_factors_iter.next().unwrap();
    assert_eq!(2, channel_factor.src_channel);
    assert_eq!(1, channel_factor.factor);
    assert!(channel_factors_iter.next().is_none());

    let channel_transform = channel_transform_iter.next().unwrap();
    assert_eq!(3, channel_transform.dest_channel);
    assert_eq!(true, channel_transform.is_chroma);
    assert_eq!(1, channel_transform.denominator);
    let mut channel_factors_iter = channel_transform.channel_factors.iter();
    assert!(channel_factors_iter.next().is_none());

    let channel_transform = channel_transform_iter.next().unwrap();
    assert_eq!(4, channel_transform.dest_channel);
    assert_eq!(false, channel_transform.is_chroma);
    assert_eq!(2, channel_transform.denominator);
    let mut channel_factors_iter = channel_transform.channel_factors.iter();
    assert!(channel_factors_iter.next().is_none());
}

#[test]
fn test_color_transform_passtrough() {
    let header = header::Header {
        version: 1,
        width: 3,
        height: 2,
        layers: 1,
        channels: 3,
        bit_depth: 8,
        is_signed: false,
        quality: 512,
        chroma_scale: 8,
        block_size: header::BLOCK_DEFAULT,
        filter: header::Filter::Cubic,
        quantization: header::Quantization::Scalar,
        encoder: header::Encoder::Turbo,
        intent: header::Intent::RGB,
        metadata_size: 0,
    };

    let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8];

    let expected = input.iter().map(|v| (*v as i16) * 8).collect::<Vec<_>>();

    let mut color_transform_program = ColorTransformProgram::new();
    color_transform_program
        .add_channel_transform(ChannelTransformBuilder::new().set_dest_channel(0).build())
        .add_channel_transform(ChannelTransformBuilder::new().set_dest_channel(2).build())
        .add_channel_transform(ChannelTransformBuilder::new().set_dest_channel(1).build());

    let mut actual = vec![0_i16; 3 * 2 * 3];

    color_transform_program.transform(&input, &header, &mut actual);

    assert_eq!(expected, actual);
}

#[test]
fn test_color_transform_yuv() {
    let header = header::Header {
        version: 1,
        width: 3,
        height: 2,
        layers: 1,
        channels: 3,
        bit_depth: 8,
        is_signed: false,
        quality: 512,
        chroma_scale: 8,
        block_size: header::BLOCK_DEFAULT,
        filter: header::Filter::Cubic,
        quantization: header::Quantization::Scalar,
        encoder: header::Encoder::Turbo,
        intent: header::Intent::RGB,
        metadata_size: 0,
    };

    let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8];

    let expected: Vec<i16> = vec![
        -48, -48, -48, 32, 32, 32, 36, 44, 52, 20, 28, 36, -32, -32, -32, 48, 48, 48,
    ];

    let mut color_transform_program = ColorTransformProgram::new();
    color_transform_program
        .add_channel_transform(
            ChannelTransformBuilder::new()
                .set_dest_channel(0)
                .add_channel_factor(1, -1)
                .set_chroma()
                .build(),
        )
        .add_channel_transform(
            ChannelTransformBuilder::new()
                .set_dest_channel(2)
                .add_channel_factor(1, -1)
                .set_chroma()
                .build(),
        )
        .add_channel_transform(
            ChannelTransformBuilder::new()
                .set_dest_channel(1)
                .add_channel_factor(0, 1)
                .add_channel_factor(2, 1)
                .set_denominator(4)
                .build(),
        );

    let mut actual = vec![0_i16; 3 * 2 * 3];

    color_transform_program.transform(&input, &header, &mut actual);

    assert_eq!(expected, actual);
}

#[test]
fn test_yuv420_rgba32_conversion() {
    let width = 13;
    let height = 8;
    let expected_rgb_image = vec![255; 4 * width * height];
    let yuv420_image = (0..width * height)
        .map(|_| 235)
        .chain((0..width * height / 2).map(|_| 128))
        .collect();
    let rgb_image = yuv420_to_rgba32(&yuv420_image, width, height);
    assert!(rgb_image == expected_rgb_image);
}

#[test]
fn test_rgba32_yuv420_conversion() {
    let width = 16;
    let height = 7;
    let expected_yuv420_image = vec![
        41, 41, 42, 42, 43, 43, 44, 44, 45, 45, 46, 46, 47, 47, 48, 48, 41, 42, 42, 43, 43, 44, 44,
        45, 45, 46, 46, 47, 47, 48, 48, 49, 41, 42, 42, 43, 43, 44, 44, 45, 45, 46, 46, 47, 47, 48,
        48, 49, 42, 42, 43, 43, 44, 44, 45, 45, 46, 46, 47, 47, 48, 48, 49, 49, 42, 42, 43, 43, 44,
        44, 45, 45, 46, 46, 47, 47, 48, 48, 49, 49, 42, 43, 43, 44, 44, 45, 45, 46, 46, 47, 47, 48,
        48, 49, 49, 50, 42, 43, 43, 44, 44, 45, 45, 46, 46, 47, 47, 48, 48, 49, 50, 50, 239, 239,
        238, 237, 237, 236, 236, 235, 239, 238, 238, 237, 237, 236, 235, 235, 239, 238, 237, 237,
        236, 236, 235, 234, 238, 238, 237, 237, 236, 235, 235, 234, 110, 109, 108, 108, 107, 106,
        105, 105, 111, 110, 109, 108, 108, 107, 106, 105, 111, 111, 110, 109, 109, 108, 107, 106,
        112, 112, 111, 110, 109, 109, 108, 107,
    ];
    let rgb_image = (0..width * height)
        .flat_map(|i| vec![(i / width) as u8, (i % width) as u8, 255, 255])
        .collect();
    let yuv420_image = rgba32_to_yuv420(&rgb_image, width, height);
    assert!(yuv420_image == expected_yuv420_image);
}

#[test]
fn test_rgba32_yuv420_invertibility() {
    let width = 12;
    let height = 11;
    let rgb_image = vec![255; 4 * width * height];
    let yuv420_image = rgba32_to_yuv420(&rgb_image, width, height);
    let rgb_image_again = yuv420_to_rgba32(&yuv420_image, width, height);
    assert!(rgb_image == rgb_image_again);
}

#[test]
fn test_yuv420_to_yuv444_planar() {
    let input_data = vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, // Y
        99, 98, 97, 96, 95, 94, 93, 92, // U
        89, 88, 87, 86, 85, 84, 83, 82, // V
    ];

    let expected = vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, // Y
        99, 99, 98, 98, 97, 97, 96, 96, 99, 99, 98, 98, 97, 97, 96, 96, 95, 95, 94, 94, 93, 93, 92,
        92, 95, 95, 94, 94, 93, 93, 92, 92, // U
        89, 89, 88, 88, 87, 87, 86, 86, 89, 89, 88, 88, 87, 87, 86, 86, 85, 85, 84, 84, 83, 83, 82,
        82, 85, 85, 84, 84, 83, 83, 82, 82, // V
    ];

    let actual = yuv420_to_planar_yuv444(&input_data, 8, 4);

    assert_eq!(expected, actual);
}

#[test]
fn test_yuv420_to_yuv444_planar_odd_size() {
    let input_data = vec![
        0, 1, 2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 13, 14, 16, 17, 18, 19, 20, 21, 22, // Y
        99, 98, 97, 96, 95, 94, 93, 92, // U
        89, 88, 87, 86, 85, 84, 83, 82, // V
    ];

    let expected = vec![
        0, 1, 2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 13, 14, 16, 17, 18, 19, 20, 21, 22, // Y
        99, 99, 98, 98, 97, 97, 96, 99, 99, 98, 98, 97, 97, 96, 95, 95, 94, 94, 93, 93,
        92, // U
        89, 89, 88, 88, 87, 87, 86, 89, 89, 88, 88, 87, 87, 86, 85, 85, 84, 84, 83, 83,
        82, // V
    ];

    let actual = yuv420_to_planar_yuv444(&input_data, 7, 3);

    assert_eq!(expected, actual);
}

#[test]
fn test_planar_yuv444_to_yuv420() {
    let input_data = vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, // Y
        99, 99, 98, 98, 97, 97, 96, 96, 99, 99, 98, 98, 97, 97, 96, 96, 95, 95, 94, 94, 93, 93, 92,
        92, 95, 95, 94, 94, 93, 93, 92, 92, // U
        89, 89, 88, 88, 87, 87, 86, 86, 89, 89, 88, 88, 87, 87, 86, 86, 85, 85, 84, 84, 83, 83, 82,
        82, 85, 85, 84, 84, 83, 83, 82, 82, // V
    ];

    let expected = vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, // Y
        99, 98, 97, 96, 95, 94, 93, 92, // U
        89, 88, 87, 86, 85, 84, 83, 82, // V
    ];

    let actual = planar_yuv444_to_yuv420(&input_data, 8, 4);

    assert_eq!(expected, actual);
}

#[test]
fn test_planar_yuv444_to_yuv420_odd_size() {
    let input_data = vec![
        0, 1, 2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 13, 14, 16, 17, 18, 19, 20, 21, 22, // Y
        99, 99, 98, 98, 97, 97, 96, 99, 99, 98, 98, 97, 97, 96, 95, 95, 94, 94, 93, 93,
        92, // U
        89, 89, 88, 88, 87, 87, 86, 89, 89, 88, 88, 87, 87, 86, 85, 85, 84, 84, 83, 83,
        82, // V
    ];

    let expected = vec![
        0, 1, 2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 13, 14, 16, 17, 18, 19, 20, 21, 22, // Y
        99, 98, 97, 96, 95, 94, 93, 92, // U
        89, 88, 87, 86, 85, 84, 83, 82, // V
    ];

    let actual = planar_yuv444_to_yuv420(&input_data, 7, 3);

    assert_eq!(expected, actual);
}
