use super::*;
use color_transform::ChannelTransformBuilder;

#[test]
fn test_lift_and_quantize() {
    let header = header::Header {
        version: 1,
        width: 12,
        height: 8,
        layers: 1,
        channels: 3,
        bit_depth: 8,
        is_signed: false,
        quality: 124,
        chroma_scale: 8,
        block_size: header::BLOCK_DEFAULT,
        filter: header::Filter::Linear,
        quantization: header::Quantization::Scalar,
        encoder: header::Encoder::Contextual,
        intent: header::Intent::RGB,
        metadata_size: 0,
    };

    let mut aux_data = vec![
        0, 24, 48, 72, 96, 120, 144, 168, 192, 216, 240, 264, 288, 312, 336, 360, 384, 408, 432,
        456, 480, 504, 528, 552, 576, 600, 624, 648, 672, 696, 720, 744, 768, 792, 816, 840, 864,
        888, 912, 936, 960, 984, 1008, 1032, 1056, 1080, 1104, 1128, 1152, 1176, 1200, 1224, 1248,
        1272, 1296, 1320, 1344, 1368, 1392, 1416, 1440, 1464, 1488, 1512, 1536, 1560, 1584, 1608,
        1632, 1656, 1680, 1704, 1728, 1752, 1776, 1800, 1824, 1848, 1872, 1896, 1920, 1944, 1968,
        1992, 2016, 2040, 16, 40, 64, 88, 112, 136, 160, 184, 208, 232, 8, 32, 56, 80, 104, 128,
        152, 176, 200, 224, 248, 272, 296, 320, 344, 368, 392, 416, 440, 464, 488, 512, 536, 560,
        584, 608, 632, 656, 680, 704, 728, 752, 776, 800, 824, 848, 872, 896, 920, 944, 968, 992,
        1016, 1040, 1064, 1088, 1112, 1136, 1160, 1184, 1208, 1232, 1256, 1280, 1304, 1328, 1352,
        1376, 1400, 1424, 1448, 1472, 1496, 1520, 1544, 1568, 1592, 1616, 1640, 1664, 1688, 1712,
        1736, 1760, 1784, 1808, 1832, 1856, 1880, 1904, 1928, 1952, 1976, 2000, 2024, 0, 24, 48,
        72, 96, 120, 144, 168, 192, 216, 240, 16, 40, 64, 88, 112, 136, 160, 184, 208, 232, 256,
        280, 304, 328, 352, 376, 400, 424, 448, 472, 496, 520, 544, 568, 592, 616, 640, 664, 688,
        712, 736, 760, 784, 808, 832, 856, 880, 904, 928, 952, 976, 1000, 1024, 1048, 1072, 1096,
        1120, 1144, 1168, 1192, 1216, 1240, 1264, 1288, 1312, 1336, 1360, 1384, 1408, 1432, 1456,
        1480, 1504, 1528, 1552, 1576, 1600, 1624, 1648, 1672, 1696, 1720, 1744, 1768, 1792, 1816,
        1840, 1864, 1888, 1912, 1936, 1960, 1984, 2008, 2032, 8, 32, 56, 80, 104, 128, 152, 176,
        200, 224, 248,
    ];

    let expected = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 202, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 79, 0, -1, 0, -4, 0, 0, 0, 69, 0, 1,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 3, -5, 0, 2, 0, 0, 0, 4, 0, 0, 0, 4, 15, -22, 0,
        -26, 0, -26, 0, -26, 0, -26, 0, 8, 0, 0, 0, 0, 0, 0, 0, 202, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 79,
        0, -2, 0, -5, 0, 0, 0, 69, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, -3, -9, 0, 1,
        0, 0, 0, 4, 0, 0, 0, 4, -15, -30, 0, -26, 0, -26, 0, -26, 0, -26, 0, 16, 0, 0, 0, 0, 0, 0,
        0, 202, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 79, 0, -2, 0, -5, 0, 0, 0, 69, 0, 1, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 19, -3, -9, 0, 1, 0, 0, 0, 4, 0, 0, 0, 4, -15, -30, 0, -26, 0, -26, 0,
        -26, 0, -26, 0,
    ];

    lift_and_quantize(&mut aux_data, 96, &header, &[false; 3], 8);
    assert_eq!(aux_data, expected);
}

#[test]
fn test_compress_lossy_contextual_linear() {
    let header = header::Header {
        version: 1,
        width: 12,
        height: 8,
        layers: 1,
        channels: 3,
        bit_depth: 8,
        is_signed: false,
        quality: 124,
        chroma_scale: 8,
        block_size: header::BLOCK_DEFAULT,
        filter: header::Filter::Linear,
        quantization: header::Quantization::Scalar,
        encoder: header::Encoder::Contextual,
        intent: header::Intent::RGB,
        metadata_size: 0,
    };

    let image = (0..header.width * header.height * header.layers as u32 * header.channels as u32)
        .map(|i| (i % 256) as u8)
        .collect::<Vec<_>>();

    let mut buffer = vec![0u8; image.len()];

    let gfwx_size = compress(
        &image,
        &header,
        &mut buffer,
        &[],
        &ColorTransformProgram::new(),
    )
    .unwrap();

    let expected = vec![
        71, 70, 87, 88, 1, 0, 0, 0, 12, 0, 0, 0, 8, 0, 0, 0, 2, 0, 0, 0, 229, 96, 15, 7, 7, 2, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 160, 2, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 7, 0, 0, 128, 0, 0, 0, 28,
        160, 58, 0, 196, 168, 16, 0, 66, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 35, 0, 0, 128, 232, 0,
        0, 64, 212, 54, 0, 128, 0, 0, 176, 1, 212, 94, 0, 128, 0, 0, 128, 21, 3, 0, 0, 0, 3, 0, 0,
        0, 3, 0, 0, 0, 161, 80, 56, 198, 177, 170, 211, 7, 0, 0, 0, 96, 161, 80, 56, 198, 78, 233,
        244, 1, 0, 0, 0, 44, 161, 80, 56, 198, 78, 233, 244, 1, 0, 0, 0, 44, 4, 0, 0, 0, 4, 0, 0,
        0, 4, 0, 0, 0, 30, 247, 157, 138, 8, 0, 36, 0, 86, 128, 25, 144, 0, 0, 80, 1, 28, 247, 149,
        138, 0, 32, 19, 0, 2, 204, 128, 68, 0, 128, 10, 176, 28, 247, 149, 138, 0, 32, 19, 0, 2,
        204, 128, 68, 0, 128, 10, 176,
    ];

    assert_eq!(&buffer[..gfwx_size], expected.as_slice());
}

#[test]
fn test_compress_lossy_turbo_cubic() {
    let header = header::Header {
        version: 1,
        width: 12,
        height: 8,
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

    let image = (0..header.width * header.height * header.layers as u32 * header.channels as u32)
        .map(|i| (i % 256) as u8)
        .collect::<Vec<_>>();

    let mut buffer = vec![0u8; 2 * image.len()];

    let gfwx_size = compress(
        &image,
        &header,
        &mut buffer,
        &[],
        &ColorTransformProgram::new(),
    )
    .unwrap();

    let expected = vec![
        71, 70, 87, 88, 1, 0, 0, 0, 12, 0, 0, 0, 8, 0, 0, 0, 2, 0, 0, 0, 229, 224, 63, 7, 7, 0, 0,
        1, 0, 0, 0, 0, 0, 0, 0, 160, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 13, 0, 0, 128, 0, 0, 0,
        64, 6, 0, 0, 196, 0, 0, 0, 160, 3, 0, 0, 66, 0, 0, 0, 80, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0,
        0, 23, 0, 0, 64, 0, 84, 0, 136, 128, 83, 1, 0, 23, 0, 0, 64, 0, 92, 0, 136, 128, 82, 1, 0,
        23, 0, 0, 64, 0, 92, 0, 136, 128, 82, 1, 0, 4, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 139, 255,
        231, 64, 87, 0, 0, 7, 3, 208, 6, 0, 56, 5, 0, 135, 128, 255, 231, 64, 5, 0, 112, 176, 128,
        56, 0, 112, 78, 1, 192, 225, 128, 255, 231, 64, 5, 0, 112, 176, 128, 56, 0, 112, 78, 1,
        192, 225, 10, 0, 0, 0, 10, 0, 0, 0, 10, 0, 0, 0, 247, 222, 123, 62, 45, 31, 96, 177, 0,
        214, 90, 107, 5, 0, 151, 19, 224, 9, 0, 112, 0, 23, 4, 0, 0, 96, 17, 0, 1, 0, 88, 4, 69, 0,
        0, 22, 0, 0, 0, 128, 247, 222, 123, 62, 45, 31, 96, 177, 0, 214, 90, 107, 5, 0, 23, 18,
        160, 9, 0, 112, 192, 5, 1, 0, 0, 24, 4, 0, 0, 0, 22, 1, 17, 0, 128, 69, 0, 0, 0, 96, 247,
        222, 123, 62, 45, 31, 96, 177, 0, 214, 90, 107, 5, 0, 23, 18, 160, 9, 0, 112, 192, 5, 1, 0,
        0, 24, 4, 0, 0, 0, 22, 1, 17, 0, 128, 69, 0, 0, 0, 96,
    ];

    assert_eq!(&buffer[..gfwx_size], expected.as_slice());
}

#[test]
fn test_compress_loseless_fast_linear_block_max() {
    let header = header::Header {
        version: 1,
        width: 12,
        height: 8,
        layers: 1,
        channels: 3,
        bit_depth: 8,
        is_signed: false,
        quality: header::QUALITY_MAX,
        chroma_scale: 8,
        block_size: header::BLOCK_MAX,
        filter: header::Filter::Linear,
        quantization: header::Quantization::Scalar,
        encoder: header::Encoder::Fast,
        intent: header::Intent::RGB,
        metadata_size: 0,
    };

    let image = (0..header.width * header.height * header.layers as u32 * header.channels as u32)
        .map(|i| (i % 256) as u8)
        .collect::<Vec<_>>();

    let mut buffer = vec![0u8; 2 * image.len()];

    let gfwx_size = compress(
        &image,
        &header,
        &mut buffer,
        &[],
        &ColorTransformProgram::new(),
    )
    .unwrap();

    let expected = vec![
        71, 70, 87, 88, 1, 0, 0, 0, 12, 0, 0, 0, 8, 0, 0, 0, 2, 0, 0, 0, 252, 224, 127, 7, 7, 1, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 160, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 21, 0, 128, 128, 10, 0,
        140, 128, 10, 0, 148, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 2, 0, 0, 192, 0, 24, 0, 246, 128,
        168, 0, 0, 2, 0, 0, 192, 0, 26, 0, 246, 128, 168, 0, 0, 2, 0, 0, 192, 0, 26, 0, 246, 128,
        168, 0, 0, 5, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 0, 68, 0, 196, 0, 0, 2, 96, 64, 1, 128, 10,
        10, 0, 204, 0, 0, 0, 0, 192, 0, 68, 0, 196, 0, 8, 128, 22, 1, 0, 42, 0, 0, 15, 224, 65, 0,
        68, 0, 196, 0, 8, 128, 22, 1, 0, 42, 0, 0, 15, 224, 65, 9, 0, 0, 0, 9, 0, 0, 0, 9, 0, 0, 0,
        128, 160, 16, 208, 0, 128, 224, 96, 1, 0, 193, 79, 56, 1, 0, 176, 178, 6, 0, 0, 64, 36, 0,
        0, 0, 68, 2, 0, 3, 0, 226, 7, 0, 0, 0, 241, 128, 160, 16, 208, 0, 128, 224, 96, 1, 0, 193,
        75, 64, 1, 0, 176, 89, 2, 0, 0, 32, 18, 0, 0, 0, 68, 2, 0, 3, 0, 226, 7, 0, 0, 0, 241, 128,
        160, 16, 208, 0, 128, 224, 96, 1, 0, 193, 75, 64, 1, 0, 176, 89, 2, 0, 0, 32, 18, 0, 0, 0,
        68, 2, 0, 3, 0, 226, 7, 0, 0, 0, 241,
    ];

    assert_eq!(&buffer[..gfwx_size], expected.as_slice());
}

#[test]
fn test_compress_with_color_transform() {
    let metadata = vec![0u8; 0];

    let header = header::Header {
        version: 1,
        width: 12,
        height: 8,
        layers: 1,
        channels: 3,
        bit_depth: 8,
        is_signed: false,
        quality: 124,
        chroma_scale: 8,
        block_size: header::BLOCK_DEFAULT,
        filter: header::Filter::Linear,
        quantization: header::Quantization::Scalar,
        encoder: header::Encoder::Contextual,
        intent: header::Intent::Generic,
        metadata_size: metadata.len() as u32,
    };

    let image = (0..header.width * header.height * header.layers as u32 * header.channels as u32)
        .map(|i| (i % 256) as u8)
        .collect::<Vec<_>>();

    let mut buffer = vec![0u8; image.len() * 4];

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

    let gfwx_size = compress(
        &image,
        &header,
        &mut buffer,
        &metadata,
        &color_transform_program,
    )
    .unwrap();

    let expected = vec![
        71, 70, 87, 88, 1, 0, 0, 0, 12, 0, 0, 0, 8, 0, 0, 0, 2, 0, 0, 0, 229, 96, 15, 7, 0, 2, 0,
        0, 0, 0, 0, 0, 183, 119, 85, 151, 246, 114, 119, 85, 0, 128, 50, 233, 1, 0, 0, 0, 1, 0, 0,
        0, 1, 0, 0, 0, 0, 0, 0, 194, 160, 58, 0, 196, 0, 0, 0, 198, 1, 0, 0, 0, 2, 0, 0, 0, 1, 0,
        0, 0, 0, 0, 0, 204, 208, 54, 0, 128, 0, 0, 176, 1, 0, 0, 0, 204, 1, 0, 0, 0, 3, 0, 0, 0, 1,
        0, 0, 0, 0, 0, 0, 162, 161, 80, 56, 198, 78, 228, 244, 1, 0, 0, 0, 44, 0, 0, 0, 162, 2, 0,
        0, 0, 4, 0, 0, 0, 1, 0, 0, 0, 142, 1, 182, 138, 0, 0, 0, 64, 128, 203, 213, 138, 17, 0, 68,
        2, 86, 128, 21, 48, 0, 0, 80, 1, 0, 0, 192, 133,
    ];

    assert_eq!(expected.as_slice(), &buffer[..gfwx_size],);
}

#[test]
fn test_decompress_with_color_detransform() {
    let metadata = vec![0u8; 0];

    let header = header::Header {
        version: 1,
        width: 12,
        height: 8,
        layers: 1,
        channels: 3,
        bit_depth: 8,
        is_signed: false,
        quality: 124,
        chroma_scale: 8,
        block_size: header::BLOCK_DEFAULT,
        filter: header::Filter::Linear,
        quantization: header::Quantization::Scalar,
        encoder: header::Encoder::Contextual,
        intent: header::Intent::Generic,
        metadata_size: metadata.len() as u32,
    };

    // contains color transform from test_compress_with_color_transform test
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

    decompress(&input, &header, &mut actual, 0, false).unwrap();

    assert_eq!(expected.len(), actual.len());
    assert_eq!(expected, actual);
}

#[test]
fn test_decompress_lossy_contextual_linear() {
    let expected = vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31, 29, 30, 31, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55, 56, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 65, 66,
        67, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 92,
        94, 95, 95, 96, 97, 98, 99, 100, 101, 103, 104, 101, 103, 104, 108, 109, 110, 111, 111,
        112, 114, 114, 115, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 128, 130,
        131, 131, 133, 134, 134, 136, 137, 137, 139, 140, 137, 139, 140, 144, 145, 146, 147, 147,
        148, 150, 150, 151, 153, 154, 155, 157, 157, 158, 159, 160, 161, 162, 163, 164, 164, 166,
        167, 167, 169, 170, 170, 172, 173, 173, 175, 176, 173, 175, 176, 179, 180, 181, 181, 184,
        185, 186, 186, 187, 189, 189, 190, 192, 192, 193, 195, 195, 196, 197, 199, 200, 200, 202,
        203, 203, 205, 206, 206, 208, 209, 209, 211, 212, 209, 211, 212, 215, 216, 217, 215, 222,
        223, 222, 221, 222, 225, 224, 225, 227, 228, 229, 230, 231, 232, 233, 235, 236, 237, 238,
        239, 240, 242, 243, 243, 245, 246, 246, 248, 249, 246, 248, 249, 252, 253, 254, 253, 2, 3,
        5, 2, 3, 6, 5, 6, 8, 9, 10, 11, 12, 13, 15, 16, 17, 18, 19, 20, 21, 23, 24, 24, 26, 27, 27,
        29, 30, 27, 29, 30_u8,
    ];

    let compressed = vec![
        0, 0, 0, 160, 2, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 7, 0, 0, 128, 0, 0, 0, 28, 160, 58, 0,
        196, 168, 16, 0, 66, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 35, 0, 0, 128, 232, 0, 0, 64, 212,
        54, 0, 128, 0, 0, 176, 1, 212, 94, 0, 128, 0, 0, 128, 21, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0,
        0, 161, 80, 56, 198, 177, 170, 211, 7, 0, 0, 0, 96, 161, 80, 56, 198, 78, 233, 244, 1, 0,
        0, 0, 44, 161, 80, 56, 198, 78, 233, 244, 1, 0, 0, 0, 44, 4, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0,
        0, 30, 247, 157, 138, 8, 0, 36, 0, 86, 128, 25, 144, 0, 0, 80, 1, 28, 247, 149, 138, 0, 32,
        19, 0, 2, 204, 128, 68, 0, 128, 10, 176, 28, 247, 149, 138, 0, 32, 19, 0, 2, 204, 128, 68,
        0, 128, 10, 176,
    ];

    let header = header::Header {
        version: 1,
        width: 12,
        height: 8,
        layers: 1,
        channels: 3,
        bit_depth: 8,
        is_signed: false,
        quality: 124,
        chroma_scale: 8,
        block_size: header::BLOCK_DEFAULT,
        filter: header::Filter::Linear,
        quantization: header::Quantization::Scalar,
        encoder: header::Encoder::Contextual,
        intent: header::Intent::RGB,
        metadata_size: 0,
    };

    let mut actual = vec![0_u8; expected.len()];

    let next_point_of_interest = decompress(&compressed, &header, &mut actual, 0, false).unwrap();

    assert_eq!(0, next_point_of_interest);
    assert_eq!(expected, actual);
}

#[test]
fn test_decompress_lossy_contextual_linear_downsampled() {
    let expected = vec![
        0, 1, 2, 6, 7, 8, 12, 13, 14, 18, 19, 20, 23, 24, 25, 29, 30, 31, 72, 73, 74, 78, 79, 80,
        84, 85, 86, 90, 91, 92, 95, 96, 97, 101, 103, 104, 144, 145, 146, 150, 150, 151, 157, 157,
        158, 162, 163, 164, 167, 169, 170, 173, 175, 176, 224, 225, 226, 175, 159, 160, 172, 173,
        174, 179, 180, 181, 185, 187, 188, 191, 193, 194,
    ];

    let header = header::Header {
        version: 1,
        width: 12,
        height: 8,
        layers: 1,
        channels: 3,
        bit_depth: 8,
        is_signed: false,
        quality: 124,
        chroma_scale: 8,
        block_size: header::BLOCK_DEFAULT,
        filter: header::Filter::Linear,
        quantization: header::Quantization::Scalar,
        encoder: header::Encoder::Contextual,
        intent: header::Intent::RGB,
        metadata_size: 0,
    };

    let compressed = vec![
        0, 0, 0, 160, 2, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 7, 0, 0, 128, 0, 0, 0, 28, 160, 58, 0,
        196, 168, 16, 0, 66, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 35, 0, 0, 128, 232, 0, 0, 64, 212,
        54, 0, 128, 0, 0, 176, 1, 212, 94, 0, 128, 0, 0, 128, 21, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0,
        0, 161, 80, 56, 198, 177, 170, 211, 7, 0, 0, 0, 96, 161, 80, 56, 198, 78, 233, 244, 1, 0,
        0, 0, 44, 161, 80, 56, 198, 78, 233, 244, 1, 0, 0, 0, 44, 4, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0,
        0, 30, 247, 157, 138, 8, 0, 36, 0, 86, 128, 25, 144, 0, 0, 80, 1, 28, 247, 149, 138, 0, 32,
        19, 0, 2, 204, 128, 68, 0, 128, 10, 176, 28, 247, 149, 138, 0, 32, 19, 0, 2, 204, 128, 68,
        0, 128, 10, 176,
    ];

    let mut actual = vec![0_u8; expected.len()];

    let next_point_of_interest = decompress(&compressed, &header, &mut actual, 1, false).unwrap();

    assert_eq!(0, next_point_of_interest);
    assert_eq!(expected, actual);
}

#[test]
fn test_decompress_truncated() {
    let expected = vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 22, 23, 24, 25,
        26, 27, 25, 26, 27, 25, 26, 27, 25, 26, 27, 41, 42, 43, 43, 43, 44, 45, 45, 46, 47, 47, 48,
        49, 49, 50, 52, 53, 54, 55, 56, 57, 59, 60, 61, 62, 63, 64, 62, 63, 64, 62, 63, 64, 62, 63,
        64, 82, 83, 84, 83, 83, 84, 84, 84, 85, 85, 85, 86, 86, 86, 87, 89, 89, 90, 92, 93, 94, 95,
        97, 98, 99, 100, 101, 99, 100, 101, 99, 100, 101, 99, 100, 101, 123, 124, 125, 122, 123,
        124, 122, 123, 124, 122, 122, 123, 122, 122, 123, 126, 126, 127, 129, 130, 131, 132, 134,
        135, 136, 138, 139, 136, 138, 139, 136, 138, 139, 136, 138, 139, 164, 165, 166, 162, 163,
        164, 161, 162, 163, 160, 160, 161, 159, 158, 159, 162, 162, 163, 166, 167, 168, 169, 171,
        172, 173, 175, 176, 173, 175, 176, 173, 175, 176, 173, 175, 176, 164, 165, 166, 162, 163,
        164, 161, 162, 163, 160, 160, 161, 159, 158, 159, 162, 162, 163, 166, 167, 168, 169, 171,
        172, 173, 175, 176, 173, 175, 176, 173, 175, 176, 173, 175, 176, 164, 165, 166, 162, 163,
        164, 161, 162, 163, 160, 160, 161, 159, 158, 159, 162, 162, 163, 166, 167, 168, 169, 171,
        172, 173, 175, 176, 173, 175, 176, 173, 175, 176, 173, 175, 176, 164, 165, 166, 162, 163,
        164, 161, 162, 163, 160, 160, 161, 159, 158, 159, 162, 162, 163, 166, 167, 168, 169, 171,
        172, 173, 175, 176, 173, 175, 176, 173, 175, 176, 173, 175, 176,
    ];

    let compressed = vec![
        0, 0, 0, 160, 2, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 7, 0, 0, 128, 0, 0, 0, 28, 160, 58, 0,
        196, 168, 16, 0, 66, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 35, 0, 0, 128, 232, 0, 0, 64, 212,
        54, 0, 128, 0, 0, 176, 1, 212, 94, 0, 128, 0, 0, 128, 21, 3, 0, 0, 0, 3, 0, 0, 0, 3_u8,
    ];

    let header = header::Header {
        version: 1,
        width: 12,
        height: 8,
        layers: 1,
        channels: 3,
        bit_depth: 8,
        is_signed: false,
        quality: 124,
        chroma_scale: 8,
        block_size: 7,
        filter: header::Filter::Linear,
        quantization: header::Quantization::Scalar,
        encoder: header::Encoder::Contextual,
        intent: header::Intent::RGB,
        metadata_size: 0,
    };

    let mut actual = vec![0_u8; expected.len()];

    let next_point_of_interest = decompress(&compressed, &header, &mut actual, 0, false).unwrap();

    assert_eq!(148, next_point_of_interest);
    assert_eq!(expected, actual);
}
