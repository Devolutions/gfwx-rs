use super::{builder::HeaderBuilder, *};

#[test]
fn test_header_encoding() {
    let width = 1920;
    let height = 1080;
    let layers = 1;
    let channels = 4;
    let header = Header {
        version: 1,
        width,
        height,
        layers,
        channels,
        bit_depth: 8,
        is_signed: false,
        quality: QUALITY_MAX,
        chroma_scale: 1,
        block_size: BLOCK_DEFAULT,
        filter: Filter::Linear,
        quantization: Quantization::Scalar,
        encoder: Encoder::Contextual,
        intent: Intent::RGBA,
        metadata_size: 0,
        channel_size: width as usize * height as usize,
        image_size: width as usize * height as usize * layers as usize * channels as usize,
    };

    let expected = vec![
        71, 70, 87, 88, 1, 0, 0, 0, 128, 7, 0, 0, 56, 4, 0, 0, 3, 0, 0, 0, 5, 224, 127, 7, 8, 2, 0,
        0, 0, 0, 0, 0,
    ];
    let mut encoded = vec![];
    header.encode(&mut encoded).unwrap();
    assert_eq!(encoded, expected);
}

#[test]
fn test_header_decoding() {
    let width = 1920;
    let height = 1080;
    let layers = 1;
    let channels = 4;
    let expected_header = Header {
        version: 1,
        width,
        height,
        layers,
        channels,
        bit_depth: 8,
        is_signed: false,
        quality: QUALITY_MAX,
        chroma_scale: 1,
        block_size: BLOCK_DEFAULT,
        filter: Filter::Linear,
        quantization: Quantization::Scalar,
        encoder: Encoder::Contextual,
        intent: Intent::RGBA,
        metadata_size: 0,
        channel_size: width as usize * height as usize,
        image_size: width as usize * height as usize * layers as usize * channels as usize,
    };

    let buff = vec![
        71, 70, 87, 88, 1, 0, 0, 0, 128, 7, 0, 0, 56, 4, 0, 0, 3, 0, 0, 0, 5, 224, 127, 7, 8, 2, 0,
        0, 0, 0, 0, 0,
    ];
    let mut encoded = io::Cursor::new(buff);
    let header = Header::decode(&mut encoded).unwrap();
    assert_eq!(header, expected_header);
}

#[test]
fn test_encoding_decoding() {
    let width = 1920;
    let height = 1080;
    let layers = 1;
    let channels = 4;
    let header = Header {
        version: 1,
        width,
        height,
        layers,
        channels,
        bit_depth: 8,
        is_signed: false,
        quality: QUALITY_MAX,
        chroma_scale: 1,
        block_size: BLOCK_DEFAULT,
        filter: Filter::Linear,
        quantization: Quantization::Scalar,
        encoder: Encoder::Contextual,
        intent: Intent::RGBA,
        metadata_size: 0,
        channel_size: width as usize * height as usize,
        image_size: width as usize * height as usize * layers as usize * channels as usize,
    };

    let mut encoded = vec![];
    header.encode(&mut encoded).unwrap();
    let mut cursor = io::Cursor::new(encoded);
    let decoded = Header::decode(&mut cursor).unwrap();
    assert_eq!(header, decoded);
}

#[test]
fn test_wrong_magic() {
    let mut encoded = io::Cursor::new(vec![0u8; 32]);
    match Header::decode(&mut encoded) {
        Err(HeaderErr::WrongMagic) => (),
        _ => panic!("wrong result of decoding with invalid magic"),
    }
}

#[test]
fn test_builder_small_width() {
    let small_width = 0;
    let builder = HeaderBuilder {
        width: small_width,
        height: 1080,
        layers: 1,
        channels: 4,
        quality: QUALITY_MAX,
        chroma_scale: 1,
        block_size: BLOCK_DEFAULT,
        filter: Filter::Linear,
        encoder: Encoder::Contextual,
        intent: Intent::RGBA,
        metadata_size: 0,
    };
    match builder.build() {
        Err(HeaderErr::WrongValue(_)) => (),
        _ => panic!(
            "HeaderBuilder must return Err for the small width: {}",
            small_width
        ),
    }
}

#[test]
fn test_builder_large_width() {
    let large_width = 1 << 30;
    let builder = HeaderBuilder {
        width: large_width,
        height: 1080,
        layers: 1,
        channels: 4,
        quality: QUALITY_MAX,
        chroma_scale: 1,
        block_size: BLOCK_DEFAULT,
        filter: Filter::Linear,
        encoder: Encoder::Contextual,
        intent: Intent::RGBA,
        metadata_size: 0,
    };
    match builder.build() {
        Err(HeaderErr::WrongValue(_)) => (),
        _ => panic!(
            "HeaderBuilder must return Err for the large width: {}",
            large_width
        ),
    }
}

#[test]
fn test_builder_small_height() {
    let small_height = 0;
    let builder = HeaderBuilder {
        width: 1920,
        height: small_height,
        layers: 1,
        channels: 4,
        quality: QUALITY_MAX,
        chroma_scale: 1,
        block_size: BLOCK_DEFAULT,
        filter: Filter::Linear,
        encoder: Encoder::Contextual,
        intent: Intent::RGBA,
        metadata_size: 0,
    };
    match builder.build() {
        Err(HeaderErr::WrongValue(_)) => (),
        _ => panic!(
            "HeaderBuilder must return Err for the small height: {}",
            small_height
        ),
    }
}

#[test]
fn test_builder_large_height() {
    let large_height = 1 << 30;
    let builder = HeaderBuilder {
        width: 1920,
        height: large_height,
        layers: 1,
        channels: 4,
        quality: QUALITY_MAX,
        chroma_scale: 1,
        block_size: BLOCK_DEFAULT,
        filter: Filter::Linear,
        encoder: Encoder::Contextual,
        intent: Intent::RGBA,
        metadata_size: 0,
    };
    match builder.build() {
        Err(HeaderErr::WrongValue(_)) => (),
        _ => panic!(
            "HeaderBuilder must return Err for the large height: {}",
            large_height
        ),
    }
}

#[test]
fn test_builder_small_quality() {
    let small_quality = 0;
    let builder = HeaderBuilder {
        height: 1920,
        width: 1080,
        layers: 1,
        channels: 4,
        quality: small_quality,
        chroma_scale: 1,
        block_size: BLOCK_DEFAULT,
        filter: Filter::Linear,
        encoder: Encoder::Contextual,
        intent: Intent::RGBA,
        metadata_size: 0,
    };
    match builder.build() {
        Err(HeaderErr::WrongValue(_)) => (),
        _ => panic!(
            "HeaderBuilder must return Err for the small quality: {}",
            small_quality
        ),
    }
}

#[test]
fn test_builder_large_quality() {
    let large_quality = QUALITY_MAX + 1;
    let builder = HeaderBuilder {
        height: 1920,
        width: 1080,
        layers: 1,
        channels: 4,
        quality: large_quality,
        chroma_scale: 1,
        block_size: BLOCK_DEFAULT,
        filter: Filter::Linear,
        encoder: Encoder::Contextual,
        intent: Intent::RGBA,
        metadata_size: 0,
    };
    match builder.build() {
        Err(HeaderErr::WrongValue(_)) => (),
        _ => panic!(
            "HeaderBuilder must return Err for the large quality: {}",
            large_quality
        ),
    }
}

#[test]
fn test_builder_small_block_size() {
    let small_block_size = 0;
    let builder = HeaderBuilder {
        height: 1920,
        width: 1080,
        layers: 1,
        channels: 4,
        quality: QUALITY_MAX,
        chroma_scale: 1,
        block_size: small_block_size,
        filter: Filter::Linear,
        encoder: Encoder::Contextual,
        intent: Intent::RGBA,
        metadata_size: 0,
    };
    match builder.build() {
        Err(HeaderErr::WrongValue(_)) => (),
        _ => panic!(
            "HeaderBuilder must return Err for the small block size: {}",
            small_block_size
        ),
    }
}

#[test]
fn test_builder_large_block_size() {
    let large_block_size = 31;
    let builder = HeaderBuilder {
        height: 1920,
        width: 1080,
        layers: 1,
        channels: 4,
        quality: QUALITY_MAX,
        chroma_scale: 1,
        block_size: large_block_size,
        filter: Filter::Linear,
        encoder: Encoder::Contextual,
        intent: Intent::RGBA,
        metadata_size: 0,
    };
    match builder.build() {
        Err(HeaderErr::WrongValue(_)) => (),
        _ => panic!(
            "HeaderBuilder must return Err for the large block size: {}",
            large_block_size
        ),
    }
}

#[test]
fn test_builder_large_layer_size() {
    let builder = HeaderBuilder {
        height: 1 << 30 - 1,
        width: 1 << 30 - 1,
        layers: 1 << 4,
        channels: 4,
        quality: QUALITY_MAX,
        chroma_scale: 1,
        block_size: BLOCK_DEFAULT,
        filter: Filter::Linear,
        encoder: Encoder::Contextual,
        intent: Intent::RGBA,
        metadata_size: 0,
    };
    match builder.build() {
        Err(HeaderErr::WrongValue(_)) => (),
        _ => panic!("HeaderBuilder must return Err for the large layer size",),
    }
}

#[test]
fn test_builder_large_image_size() {
    let builder = HeaderBuilder {
        height: 1 << 30 - 1,
        width: 1 << 30 - 1,
        layers: 1 << 2,
        channels: 1 << 4,
        quality: QUALITY_MAX,
        chroma_scale: 1,
        block_size: BLOCK_DEFAULT,
        filter: Filter::Linear,
        encoder: Encoder::Contextual,
        intent: Intent::RGBA,
        metadata_size: 0,
    };
    match builder.build() {
        Err(HeaderErr::WrongValue(_)) => (),
        _ => panic!("HeaderBuilder must return Err for the large image size",),
    }
}
