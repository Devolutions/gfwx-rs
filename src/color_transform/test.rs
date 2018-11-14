use super::*;

#[test]
fn test_color_transform_program_builder() {
    let mut color_transform_program = ColorTransformProgram::new();

    color_transform_program
        .add_channel_transform(
            ChannelTransformBuilder::with_dest_channel(0)
                .add_channel_factor(1, -1)
                .set_chroma()
                .build(),
        )
        .add_channel_transform(
            ChannelTransformBuilder::with_dest_channel(2)
                .add_channel_factor(1, -1)
                .set_chroma()
                .build(),
        )
        .add_channel_transform(
            ChannelTransformBuilder::with_dest_channel(1)
                .add_channel_factor(0, 1)
                .add_channel_factor(2, 1)
                .set_denominator(4)
                .build(),
        )
        .add_channel_transform(
            ChannelTransformBuilder::with_dest_channel(3)
                .set_chroma()
                .build(),
        )
        .add_channel_transform(
            ChannelTransformBuilder::with_dest_channel(4)
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

    let input: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8];

    let expected = input.iter().map(|v| (*v as i16) * 8).collect::<Vec<_>>();

    let mut color_transform_program = ColorTransformProgram::new();
    color_transform_program
        .add_channel_transform(ChannelTransformBuilder::with_dest_channel(0).build())
        .add_channel_transform(ChannelTransformBuilder::with_dest_channel(2).build())
        .add_channel_transform(ChannelTransformBuilder::with_dest_channel(1).build());

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

    let input: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8];

    let expected: Vec<i16> = vec![
        -48, -48, -48, 32, 32, 32, 36, 44, 52, 20, 28, 36, -32, -32, -32, 48, 48, 48,
    ];

    let mut color_transform_program = ColorTransformProgram::new();
    color_transform_program
        .add_channel_transform(
            ChannelTransformBuilder::with_dest_channel(0)
                .add_channel_factor(1, -1)
                .set_chroma()
                .build(),
        )
        .add_channel_transform(
            ChannelTransformBuilder::with_dest_channel(2)
                .add_channel_factor(1, -1)
                .set_chroma()
                .build(),
        )
        .add_channel_transform(
            ChannelTransformBuilder::with_dest_channel(1)
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
fn test_color_transform_encode() {
    let expected = vec![183, 119, 85, 151, 246, 114, 119, 85, 0, 128, 50, 233];

    let mut color_transform_program = ColorTransformProgram::new();
    color_transform_program
        .add_channel_transform(
            ChannelTransformBuilder::with_dest_channel(0)
                .add_channel_factor(1, -1)
                .set_chroma()
                .build(),
        )
        .add_channel_transform(
            ChannelTransformBuilder::with_dest_channel(2)
                .add_channel_factor(1, -1)
                .set_chroma()
                .build(),
        )
        .add_channel_transform(
            ChannelTransformBuilder::with_dest_channel(1)
                .add_channel_factor(0, 1)
                .add_channel_factor(2, 1)
                .set_denominator(4)
                .build(),
        );
    let mut buffer = vec![0u8; expected.len()];

    let is_chroma = {
        let mut slice = buffer.as_mut_slice();
        color_transform_program.encode(3, &mut slice)
    };

    assert_eq!(buffer, expected);
    assert_eq!(is_chroma, vec![true, false, true]);
}

#[test]
fn test_color_transform_decode() {
    let buffer = vec![183, 119, 85, 151, 246, 114, 119, 85, 0, 128, 50, 233];

    let expected_color_transform_program = {
        let mut color_transform_program = ColorTransformProgram::new();
        color_transform_program
            .add_channel_transform(
                ChannelTransformBuilder::with_dest_channel(0)
                    .add_channel_factor(1, -1)
                    .set_chroma()
                    .build(),
            )
            .add_channel_transform(
                ChannelTransformBuilder::with_dest_channel(2)
                    .add_channel_factor(1, -1)
                    .set_chroma()
                    .build(),
            )
            .add_channel_transform(
                ChannelTransformBuilder::with_dest_channel(1)
                    .add_channel_factor(0, 1)
                    .add_channel_factor(2, 1)
                    .set_denominator(4)
                    .build(),
            );
        color_transform_program
    };

    let mut slice = buffer.as_slice();
    let mut is_chroma = vec![false; 3];
    let color_transform_program =
        ColorTransformProgram::decode(&mut slice, &mut is_chroma).unwrap();

    assert_eq!(color_transform_program, expected_color_transform_program);
    assert_eq!(is_chroma, vec![true, false, true]);
}
