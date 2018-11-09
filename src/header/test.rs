use super::*;

#[test]
fn test_header_encoding() {
    let header = Header {
        version: 1,
        width: 1920,
        height: 1080,
        layers: 1,
        channels: 4,
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
    };

    let expected = vec![
        71, 70, 87, 88, 1, 0, 0, 0, 128, 7, 0, 0, 56, 4, 0, 0, 3, 0, 0, 0, 5, 224, 127, 7, 8, 2, 0,
        0, 0, 0, 0, 0,
    ];
    let mut encoded = vec![];
    header.encode(&mut encoded).unwrap();
    assert!(encoded == expected);
}

#[test]
fn test_header_decoding() {
    let expected_header = Header {
        version: 1,
        width: 1920,
        height: 1080,
        layers: 1,
        channels: 4,
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
    };

    let buff = vec![
        71, 70, 87, 88, 1, 0, 0, 0, 128, 7, 0, 0, 56, 4, 0, 0, 3, 0, 0, 0, 5, 224, 127, 7, 8, 2, 0,
        0, 0, 0, 0, 0,
    ];
    let mut encoded = io::Cursor::new(buff);
    let header = Header::decode(&mut encoded).unwrap();
    assert!(header == expected_header);
}

#[test]
fn test_encoding_decoding() {
    let header = Header {
        version: 1,
        width: 1920,
        height: 1080,
        layers: 1,
        channels: 4,
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
    };

    let mut encoded = vec![];
    header.encode(&mut encoded).unwrap();
    let mut cursor = io::Cursor::new(encoded);
    let decoded = Header::decode(&mut cursor).unwrap();
    assert!(header == decoded);
}

#[test]
fn test_wrong_magic() {
    let mut encoded = io::Cursor::new(vec![0u8; 32]);
    match Header::decode(&mut encoded) {
        Err(HeaderDecodeErr::WrongMagic) => (),
        _ => panic!("wrong result of decoding with invalid magic"),
    }
}
