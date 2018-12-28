use gfwx::*;

#[test]
fn test_compress_simple() {
    let builder = HeaderBuilder {
        width: 12,
        height: 8,
        layers: 1,
        channels: 3,
        quality: 124,
        chroma_scale: 8,
        block_size: BLOCK_DEFAULT,
        filter: Filter::Linear,
        encoder: Encoder::Contextual,
        intent: Intent::Generic,
        metadata_size: 0,
    };
    let header = builder.build().unwrap();

    let image = (0..header.get_image_size())
        .map(|i| (i % 256) as u8)
        .collect::<Vec<_>>();

    let mut buffer = vec![0u8; image.len() * 4];

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

    let gfwx_size =
        compress_simple(&image, &header, &color_transform_program, &mut buffer).unwrap();

    let expected = vec![
        71, 70, 87, 88, 1, 0, 0, 0, 12, 0, 0, 0, 8, 0, 0, 0, 2, 0, 0, 0, 229, 96, 15, 7, 0, 2, 0,
        0, 0, 0, 0, 0, 183, 119, 85, 151, 246, 114, 119, 85, 0, 128, 50, 233, 1, 0, 0, 0, 1, 0, 0,
        0, 1, 0, 0, 0, 0, 0, 0, 194, 160, 58, 0, 196, 0, 0, 0, 198, 1, 0, 0, 0, 2, 0, 0, 0, 1, 0,
        0, 0, 0, 0, 0, 204, 208, 54, 0, 128, 0, 0, 176, 1, 0, 0, 0, 204, 1, 0, 0, 0, 3, 0, 0, 0, 1,
        0, 0, 0, 0, 0, 0, 162, 161, 80, 56, 198, 78, 228, 244, 1, 0, 0, 0, 44, 0, 0, 0, 162, 2, 0,
        0, 0, 4, 0, 0, 0, 1, 0, 0, 0, 142, 1, 182, 138, 0, 0, 0, 64, 128, 203, 213, 138, 17, 0, 68,
        2, 86, 128, 21, 48, 0, 0, 80, 1, 0, 0, 192, 133,
    ];

    assert_eq!(expected.as_slice(), &buffer[..gfwx_size]);
}

#[test]
fn test_decompress_simple() {
    let builder = HeaderBuilder {
        width: 12,
        height: 8,
        layers: 1,
        channels: 3,
        quality: 124,
        chroma_scale: 8,
        block_size: BLOCK_DEFAULT,
        filter: Filter::Linear,
        encoder: Encoder::Contextual,
        intent: Intent::Generic,
        metadata_size: 0,
    };
    let header = builder.build().unwrap();

    // contains color transform from test_compress_simple test
    let input = vec![
        183, 119, 85, 151, 246, 114, 119, 85, 0, 128, 50, 233, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0,
        0, 0, 0, 194, 160, 58, 0, 196, 0, 0, 0, 198, 1, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
        204, 208, 54, 0, 128, 0, 0, 176, 1, 0, 0, 0, 204, 1, 0, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0,
        0, 162, 161, 80, 56, 198, 78, 228, 244, 1, 0, 0, 0, 44, 0, 0, 0, 162, 2, 0, 0, 0, 4, 0, 0,
        0, 1, 0, 0, 0, 142, 1, 182, 138, 0, 0, 0, 64, 128, 203, 213, 138, 17, 0, 68, 2, 86, 128,
        21, 48, 0, 0, 80, 1, 0, 0, 192, 133,
    ];

    let expected = vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31, 29, 30, 31, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        49, 50, 51, 51, 52, 53, 54, 55, 56, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 65, 66,
        67, 72, 73, 74, 74, 75, 76, 77, 78, 79, 81, 82, 83, 85, 86, 87, 87, 88, 89, 90, 91, 92, 92,
        93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 101, 102, 103, 108, 109, 110, 110, 111,
        112, 113, 114, 115, 117, 118, 119, 121, 122, 123, 124, 125, 126, 126, 127, 128, 128, 129,
        130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 137, 138, 139, 144, 145, 146, 146, 147,
        148, 149, 150, 151, 153, 154, 155, 158, 159, 160, 160, 161, 162, 162, 163, 164, 164, 165,
        166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 173, 174, 175, 179, 180, 181, 187, 183,
        184, 173, 189, 190, 183, 191, 192, 193, 194, 195, 195, 196, 197, 198, 199, 200, 200, 201,
        202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 209, 210, 211, 215, 216, 217, 227, 219,
        220, 197, 228, 229, 212, 228, 229, 227, 228, 229, 230, 231, 232, 234, 235, 236, 237, 238,
        239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 246, 247, 248, 252, 253, 254, 255, 0, 0,
        0, 2, 3, 2, 6, 7, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
        27, 28, 29, 27, 28, 29,
    ];

    let mut actual = vec![0u8; expected.len()];

    decompress_simple(&input, &header, 0, false, &mut actual).unwrap();

    assert_eq!(expected.len(), actual.len());
    assert_eq!(expected, actual);
}
