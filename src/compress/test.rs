use super::*;

#[test]
fn test_lift_and_quantize() {
    let width = 12;
    let height = 8;
    let layers = 1;
    let channels = 3;
    let header = header::Header {
        version: 1,
        width,
        height,
        layers,
        channels,
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
        channel_size: width as usize * height as usize,
        image_size: width as usize * height as usize * layers as usize * channels as usize,
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

    lift_and_quantize(&mut aux_data, &header, &[false; 3]);
    assert_eq!(aux_data, expected);
}

#[test]
fn test_compress_lossy_contextual_linear() {
    let width = 12;
    let height = 8;
    let layers = 1;
    let channels = 3;
    let header = header::Header {
        version: 1,
        width,
        height,
        layers,
        channels,
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
        channel_size: width as usize * height as usize,
        image_size: width as usize * height as usize * layers as usize * channels as usize,
    };

    let mut image = (0..header.get_image_size() as u32)
        .map(|i| (i % 256) as i16)
        .collect::<Vec<_>>();

    let mut buffer = vec![0u8; image.len()];

    let gfwx_size = compress_aux_data(&mut image, &header, &[false; 3], &mut buffer).unwrap();

    let expected = vec![
        1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 76, 0, 128, 0, 64, 28, 2, 16, 7, 8, 0, 1, 0, 0, 0,
        1, 0, 0, 0, 1, 0, 0, 0, 0, 32, 96, 128, 0, 224, 228, 137, 0, 128, 228, 136, 1, 0, 0, 0, 1,
        0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 162, 0, 0, 0, 162, 88, 192, 0, 236, 0, 0, 0, 20, 1, 0, 0, 0,
        1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 192, 133, 0, 0, 192, 133, 149, 213, 184, 153, 0, 0, 112, 216,
    ];

    assert_eq!(&buffer[..gfwx_size], expected.as_slice());
}

#[test]
fn test_compress_lossy_turbo_cubic() {
    let width = 12;
    let height = 8;
    let layers = 1;
    let channels = 3;
    let header = header::Header {
        version: 1,
        width,
        height,
        layers,
        channels,
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
        channel_size: width as usize * height as usize,
        image_size: width as usize * height as usize * layers as usize * channels as usize,
    };

    let mut image = (0..header.get_image_size() as u32)
        .map(|i| (i % 256) as i16)
        .collect::<Vec<_>>();

    let mut buffer = vec![0u8; 2 * image.len()];

    let gfwx_size = compress_aux_data(&mut image, &header, &[false; 3], &mut buffer).unwrap();

    let expected = vec![
        1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 12, 128, 0, 24, 16, 2, 6, 4, 8, 0, 2, 0, 0, 0, 2,
        0, 0, 0, 1, 0, 0, 0, 0, 24, 2, 128, 0, 0, 0, 33, 0, 24, 2, 128, 0, 0, 0, 33, 128, 0, 32,
        134, 1, 0, 0, 0, 1, 0, 0, 0, 3, 0, 0, 0, 192, 243, 60, 2, 192, 243, 60, 2, 104, 188, 87,
        106, 3, 192, 62, 0, 192, 62, 0, 236, 1, 0, 0, 0, 1, 0, 0, 0, 4, 0, 0, 0, 0, 0, 6, 0, 0, 0,
        6, 0, 1, 89, 11, 0, 193, 1, 14, 224, 128, 5, 44, 96, 0, 128, 0, 147,
    ];

    assert_eq!(&buffer[..gfwx_size], expected.as_slice());
}

#[test]
fn test_compress_loseless_fast_linear_block_max() {
    let width = 12;
    let height = 8;
    let layers = 1;
    let channels = 3;
    let header = header::Header {
        version: 1,
        width,
        height,
        layers,
        channels,
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
        channel_size: width as usize * height as usize,
        image_size: width as usize * height as usize * layers as usize * channels as usize,
    };

    let mut image = (0..header.get_image_size() as u32)
        .map(|i| (i % 256) as i16)
        .collect::<Vec<_>>();

    let mut buffer = vec![0u8; 2 * image.len()];

    let gfwx_size = compress_aux_data(&mut image, &header, &[false; 3], &mut buffer).unwrap();

    let expected = vec![
        1, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0, 76, 0, 128, 152, 0, 16, 2, 0, 4, 8, 0, 0, 0, 0, 38,
        2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 248, 1, 0, 192, 0, 224, 3, 0, 248, 1, 0, 192, 0, 224,
        3, 0, 0, 25, 0, 192, 56, 0, 0, 4, 3, 0, 0, 0, 3, 0, 0, 0, 7, 0, 0, 0, 0, 48, 72, 196, 152,
        1, 128, 25, 0, 192, 12, 0, 0, 48, 72, 196, 152, 1, 128, 25, 0, 192, 12, 0, 36, 0, 96, 196,
        4, 0, 12, 0, 0, 79, 0, 216, 0, 40, 18, 5, 0, 0, 106, 1, 0, 128, 254, 1, 0, 0, 80, 11, 5, 0,
        0, 0, 5, 0, 0, 0, 11, 0, 0, 0, 133, 10, 37, 209, 0, 220, 0, 96, 224, 6, 0, 55, 128, 3, 192,
        1, 0, 0, 0, 176, 133, 10, 37, 209, 0, 220, 0, 96, 224, 6, 0, 55, 128, 3, 192, 1, 0, 0, 0,
        176, 0, 20, 37, 209, 0, 0, 250, 4, 0, 128, 51, 1, 0, 192, 92, 0, 132, 2, 0, 81, 0, 2, 5, 0,
        0, 0, 129, 2, 24, 1, 66, 160, 102, 152, 77, 68, 5, 44, 192, 4, 0, 0, 176, 128,
    ];

    assert_eq!(&buffer[..gfwx_size], expected.as_slice());
}

#[test]
fn test_decompress_lossy_contextual_linear() {
    let expected = vec![
        0, 25, 50, 75, 101, 123, 145, 167, 190, 214, 239, 239, 288, 312, 337, 363, 389, 410, 432,
        454, 476, 500, 525, 525, 576, 600, 625, 651, 678, 699, 720, 741, 763, 787, 812, 812, 864,
        888, 912, 939, 967, 987, 1008, 1029, 1050, 1074, 1099, 1099, 1152, 1176, 1200, 1228, 1256,
        1276, 1296, 1316, 1337, 1361, 1386, 1386, 1437, 1451, 1490, 1514, 1538, 1560, 1583, 1606,
        1629, 1653, 1678, 1678, 1722, 1726, 1781, 1800, 1820, 1845, 1871, 1896, 1922, 1946, 1971,
        1971, 2019, 2028, 40, 55, 70, 95, 121, 146, 172, 196, 221, 221, 8, 33, 58, 83, 109, 131,
        153, 175, 198, 222, 247, 247, 296, 320, 345, 370, 396, 418, 441, 463, 486, 510, 535, 535,
        584, 608, 632, 658, 684, 706, 729, 752, 775, 799, 824, 824, 872, 895, 919, 945, 972, 994,
        1017, 1040, 1064, 1088, 1113, 1113, 1160, 1183, 1206, 1233, 1260, 1283, 1306, 1329, 1353,
        1377, 1402, 1402, 1445, 1479, 1490, 1516, 1542, 1567, 1593, 1619, 1645, 1669, 1694, 1694,
        1730, 1777, 1775, 1799, 1824, 1852, 1881, 1909, 1938, 1962, 1987, 1987, 2027, 22, 16, 45,
        74, 102, 131, 159, 188, 212, 237, 237, 16, 41, 66, 91, 117, 139, 161, 183, 206, 230, 255,
        255, 304, 328, 353, 378, 404, 426, 449, 471, 494, 518, 543, 543, 592, 616, 640, 666, 692,
        714, 737, 760, 783, 807, 832, 832, 880, 903, 927, 953, 980, 1002, 1025, 1048, 1072, 1096,
        1121, 1121, 1168, 1191, 1214, 1241, 1268, 1291, 1314, 1337, 1361, 1385, 1410, 1410, 1453,
        1487, 1498, 1524, 1550, 1575, 1601, 1627, 1653, 1677, 1702, 1702, 1738, 1785, 1783, 1807,
        1832, 1860, 1889, 1917, 1946, 1970, 1995, 1995, 2035, 30, 24, 53, 82, 110, 139, 167, 196,
        220, 245, 245,
    ];

    let compressed = vec![
        2, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 7, 0, 0, 128, 0, 0, 0, 28, 160, 58, 0, 196, 168, 16, 0,
        66, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 35, 0, 0, 128, 232, 0, 0, 64, 212, 54, 0, 128, 0,
        0, 176, 1, 212, 94, 0, 128, 0, 0, 128, 21, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 161, 80, 56,
        198, 177, 170, 211, 7, 0, 0, 0, 96, 161, 80, 56, 198, 78, 233, 244, 1, 0, 0, 0, 44, 161,
        80, 56, 198, 78, 233, 244, 1, 0, 0, 0, 44, 4, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 30, 247,
        157, 138, 8, 0, 36, 0, 86, 128, 25, 144, 0, 0, 80, 1, 28, 247, 149, 138, 0, 32, 19, 0, 2,
        204, 128, 68, 0, 128, 10, 176, 28, 247, 149, 138, 0, 32, 19, 0, 2, 204, 128, 68, 0, 128,
        10, 176,
    ];

    let width = 12;
    let height = 8;
    let layers = 1;
    let channels = 3;
    let header = header::Header {
        version: 1,
        width,
        height,
        layers,
        channels,
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
        channel_size: width as usize * height as usize,
        image_size: width as usize * height as usize * layers as usize * channels as usize,
    };

    let mut actual = vec![0i16; expected.len()];
    let next_point_of_interest =
        decompress_aux_data(&compressed, &header, &[false; 3], 0, false, &mut actual).unwrap();

    assert_eq!(0, next_point_of_interest);
    assert_eq!(expected, actual);
}

#[test]
fn test_decompress_lossy_contextual_linear_downsampled() {
    let expected = vec![
        0, 50, 101, 145, 190, 239, 576, 625, 678, 720, 763, 812, 1152, 1200, 1256, 1296, 1337,
        1386, 1796, 1404, 1383, 1434, 1485, 1534, 8, 58, 109, 153, 198, 247, 584, 632, 684, 729,
        775, 824, 1160, 1206, 1260, 1306, 1353, 1402, 1804, 1278, 1387, 1444, 1501, 1550, 16, 66,
        117, 161, 206, 255, 592, 640, 692, 737, 783, 832, 1168, 1214, 1268, 1314, 1361, 1410, 1812,
        1286, 1395, 1452, 1509, 1558,
    ];

    let width = 12;
    let height = 8;
    let layers = 1;
    let channels = 3;
    let header = header::Header {
        version: 1,
        width,
        height,
        layers,
        channels,
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
        channel_size: width as usize * height as usize,
        image_size: width as usize * height as usize * layers as usize * channels as usize,
    };

    let compressed = vec![
        2, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 7, 0, 0, 128, 0, 0, 0, 28, 160, 58, 0, 196, 168, 16, 0,
        66, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 35, 0, 0, 128, 232, 0, 0, 64, 212, 54, 0, 128, 0,
        0, 176, 1, 212, 94, 0, 128, 0, 0, 128, 21, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 161, 80, 56,
        198, 177, 170, 211, 7, 0, 0, 0, 96, 161, 80, 56, 198, 78, 233, 244, 1, 0, 0, 0, 44, 161,
        80, 56, 198, 78, 233, 244, 1, 0, 0, 0, 44, 4, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 30, 247,
        157, 138, 8, 0, 36, 0, 86, 128, 25, 144, 0, 0, 80, 1, 28, 247, 149, 138, 0, 32, 19, 0, 2,
        204, 128, 68, 0, 128, 10, 176, 28, 247, 149, 138, 0, 32, 19, 0, 2, 204, 128, 68, 0, 128,
        10, 176,
    ];

    let mut actual = vec![0i16; expected.len()];
    let next_point_of_interest =
        decompress_aux_data(&compressed, &header, &[false; 3], 1, false, &mut actual).unwrap();

    assert_eq!(0, next_point_of_interest);
    assert_eq!(expected, actual);
}

#[test]
fn test_decompress_truncated() {
    let expected = vec![
        0, 25, 50, 75, 101, 126, 151, 176, 202, 202, 202, 202, 328, 344, 361, 377, 394, 420, 446,
        472, 498, 498, 498, 498, 656, 664, 672, 680, 688, 714, 741, 767, 794, 794, 794, 794, 984,
        983, 983, 982, 981, 1008, 1035, 1062, 1090, 1090, 1090, 1090, 1313, 1303, 1294, 1284, 1275,
        1302, 1330, 1358, 1386, 1386, 1386, 1386, 1313, 1303, 1294, 1284, 1275, 1302, 1330, 1358,
        1386, 1386, 1386, 1386, 1313, 1303, 1294, 1284, 1275, 1302, 1330, 1358, 1386, 1386, 1386,
        1386, 1313, 1303, 1294, 1284, 1275, 1302, 1330, 1358, 1386, 1386, 1386, 1386, 8, 33, 58,
        83, 109, 134, 159, 184, 210, 210, 210, 210, 336, 351, 367, 383, 399, 426, 453, 480, 508,
        508, 508, 508, 664, 670, 677, 683, 690, 719, 748, 777, 806, 806, 806, 806, 992, 989, 986,
        983, 980, 1011, 1042, 1073, 1104, 1104, 1104, 1104, 1321, 1308, 1296, 1283, 1271, 1303,
        1336, 1369, 1402, 1402, 1402, 1402, 1321, 1308, 1296, 1283, 1271, 1303, 1336, 1369, 1402,
        1402, 1402, 1402, 1321, 1308, 1296, 1283, 1271, 1303, 1336, 1369, 1402, 1402, 1402, 1402,
        1321, 1308, 1296, 1283, 1271, 1303, 1336, 1369, 1402, 1402, 1402, 1402, 16, 41, 66, 91,
        117, 142, 167, 192, 218, 218, 218, 218, 344, 359, 375, 391, 407, 434, 461, 488, 516, 516,
        516, 516, 672, 678, 685, 691, 698, 727, 756, 785, 814, 814, 814, 814, 1000, 997, 994, 991,
        988, 1019, 1050, 1081, 1112, 1112, 1112, 1112, 1329, 1316, 1304, 1291, 1279, 1311, 1344,
        1377, 1410, 1410, 1410, 1410, 1329, 1316, 1304, 1291, 1279, 1311, 1344, 1377, 1410, 1410,
        1410, 1410, 1329, 1316, 1304, 1291, 1279, 1311, 1344, 1377, 1410, 1410, 1410, 1410, 1329,
        1316, 1304, 1291, 1279, 1311, 1344, 1377, 1410, 1410, 1410, 1410,
    ];

    let compressed = vec![
        2, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 7, 0, 0, 128, 0, 0, 0, 28, 160, 58, 0, 196, 168, 16, 0,
        66, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 35, 0, 0, 128, 232, 0, 0, 64, 212, 54, 0, 128, 0,
        0, 176, 1, 212, 94, 0, 128, 0, 0, 128, 21, 3, 0, 0, 0, 3, 0, 0, 0, 3_u8,
    ];

    let width = 12;
    let height = 8;
    let layers = 1;
    let channels = 3;
    let header = header::Header {
        version: 1,
        width,
        height,
        layers,
        channels,
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
        channel_size: width as usize * height as usize,
        image_size: width as usize * height as usize * layers as usize * channels as usize,
    };

    let mut actual = vec![0i16; expected.len()];
    let next_point_of_interest =
        decompress_aux_data(&compressed, &header, &[false; 3], 0, false, &mut actual).unwrap();

    assert_eq!(112, next_point_of_interest);
    assert_eq!(expected, actual);
}

#[test]
fn test_decompress_invalid_block_length() {
    let compressed = vec![
        //                      ↓ there must be 1 instead of 2
        1, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0, 254, 9, 0, 0, 254, 9, 0, 0, 254, 9, 0, 1, 0, 0, 0, 1,
        0, 0, 0, 1, 0, 0, 0, 0, 0, 160, 170, 0, 0, 160, 170, 0, 0, 160, 170, 1, 0, 0, 0, 1, 0, 0,
        0, 1, 0, 0, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0,
    ];

    let width = 8;
    let height = 4;
    let layers = 1;
    let channels = 3;
    let header = header::Header {
        version: 1,
        width,
        height,
        layers,
        channels,
        bit_depth: 8,
        is_signed: false,
        quality: header::QUALITY_MAX,
        chroma_scale: 8,
        block_size: header::BLOCK_DEFAULT,
        filter: header::Filter::Linear,
        quantization: header::Quantization::Scalar,
        encoder: header::Encoder::Turbo,
        intent: header::Intent::RGB,
        metadata_size: 0,
        channel_size: width as usize * height as usize,
        image_size: width as usize * height as usize * layers as usize * channels as usize,
    };

    let mut buffer = vec![0i16; header.get_decompress_buffer_size(0).unwrap()];
    match decompress_aux_data(&compressed, &header, &[false; 3], 0, false, &mut buffer) {
        Err(DecompressError::Underflow) => (),
        Err(e) => panic!("unexpected error: {:?}", e),
        Ok(_) => panic!("decompress must return error on invalid block lenth"),
    }
}

#[test]
fn test_decompress_invalid_block_length2() {
    let compressed = vec![
        //          ↓ there must be 1 instead of 2
        2, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0, 7, 0, 0, 128, 0, 0, 0, 28, 160, 58, 0, 196, 168, 16, 0,
        66, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 35, 0, 0, 128, 232, 0, 0, 64, 212, 54, 0, 128, 0,
        0, 176, 1, 212, 94, 0, 128, 0, 0, 128, 21, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 161, 80, 56,
        198, 177, 170, 211, 7, 0, 0, 0, 96, 161, 80, 56, 198, 78, 233, 244, 1, 0, 0, 0, 44, 161,
        80, 56, 198, 78, 233, 244, 1, 0, 0, 0, 44, 4, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 30, 247,
        157, 138, 8, 0, 36, 0, 86, 128, 25, 144, 0, 0, 80, 1, 28, 247, 149, 138, 0, 32, 19, 0, 2,
        204, 128, 68, 0, 128, 10, 176, 28, 247, 149, 138, 0, 32, 19, 0, 2, 204, 128, 68, 0, 128,
        10, 176,
    ];

    let width = 12;
    let height = 8;
    let layers = 1;
    let channels = 3;
    let header = header::Header {
        version: 1,
        width,
        height,
        layers,
        channels,
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
        channel_size: width as usize * height as usize,
        image_size: width as usize * height as usize * layers as usize * channels as usize,
    };

    let mut buffer = vec![0i16; header.get_decompress_buffer_size(0).unwrap()];
    match decompress_aux_data(&compressed, &header, &[false; 3], 0, false, &mut buffer) {
        Err(DecompressError::Underflow) => (),
        Err(e) => panic!("unexpected error: {:?}", e),
        Ok(_) => panic!("decompress must return error on invalid block lenth"),
    }
}

#[test]
fn test_decompress_invalid_block_length3() {
    let compressed = vec![
        2, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 7, 0, 0, 128, 0, 0, 0, 28, 160, 58, 0, 196, 168, 16, 0,
        66, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 35, 0, 0, 128, 232, 0, 0, 64, 212, 54, 0, 128, 0,
        //                                         ↓ there must be 3 instead of 5
        0, 176, 1, 212, 94, 0, 128, 0, 0, 128, 21, 5, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 161, 80, 56,
        198, 177, 170, 211, 7, 0, 0, 0, 96, 161, 80, 56, 198, 78, 233, 244, 1, 0, 0, 0, 44, 161,
        80, 56, 198, 78, 233, 244, 1, 0, 0, 0, 44, 4, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 30, 247,
        157, 138, 8, 0, 36, 0, 86, 128, 25, 144, 0, 0, 80, 1, 28, 247, 149, 138, 0, 32, 19, 0, 2,
        204, 128, 68, 0, 128, 10, 176, 28, 247, 149, 138, 0, 32, 19, 0, 2, 204, 128, 68, 0, 128,
        10, 176,
    ];

    let width = 12;
    let height = 8;
    let layers = 1;
    let channels = 3;
    let header = header::Header {
        version: 1,
        width,
        height,
        layers,
        channels,
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
        channel_size: width as usize * height as usize,
        image_size: width as usize * height as usize * layers as usize * channels as usize,
    };

    let mut buffer = vec![0i16; header.get_decompress_buffer_size(0).unwrap()];
    match decompress_aux_data(&compressed, &header, &[false; 3], 0, false, &mut buffer) {
        Err(DecompressError::Underflow) => (),
        Err(e) => panic!("unexpected error: {:?}", e),
        Ok(_) => panic!("decompress must return error on invalid block lenth"),
    }
}
