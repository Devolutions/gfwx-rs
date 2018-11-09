extern crate clap;
extern crate gfwx;
extern crate image;
extern crate time;

use std::{error::Error, fs, i64, io, io::prelude::*, path::Path};

use image::DynamicImage::*;
use time::PreciseTime;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = get_matches();
    let input_file = matches.value_of("INPUT").unwrap();
    let output_gfwx = matches.value_of("OUTPUT.GFWX").unwrap();
    let output_decompressed = matches.value_of("OUTPUT").unwrap();
    let quality = matches.value_of("quality").unwrap().parse().unwrap();
    let chroma_scale = matches.value_of("chroma-scale").unwrap().parse().unwrap();
    let block_size = matches.value_of("block-size").unwrap().parse().unwrap();
    let downsampling = matches.value_of("downsampling").unwrap().parse().unwrap();
    let filter = match matches.value_of("filter").unwrap() {
        "linear" => gfwx::Filter::Linear,
        "cubic" => gfwx::Filter::Cubic,
        _ => panic!("clap betrayed us"),
    };
    let encoder = match matches.value_of("encoder").unwrap() {
        "fast" => gfwx::Encoder::Fast,
        "turbo" => gfwx::Encoder::Turbo,
        "contextual" => gfwx::Encoder::Contextual,
        _ => panic!("clap betrayed us again"),
    };
    let file_path = Path::new(&input_file);
    let image = image::open(&file_path)?;
    let (width, height, image, channels, intent) = get_raw_image(image, matches.value_of("intent"));
    let color_transform = match intent {
        gfwx::Intent::YUV444 => gfwx::ColorTransformProgram::yuv444_to_yuv444(),
        _ => gfwx::ColorTransformProgram::new(),
    };

    let header = gfwx::Header {
        version: 1,
        width: width,
        height: height,
        layers: 1,
        channels: channels,
        bit_depth: 8,
        is_signed: false,
        quality,
        chroma_scale,
        block_size,
        filter,
        quantization: gfwx::Quantization::Scalar,
        encoder,
        intent,
        metadata_size: 0,
    };
    let compress_start = PreciseTime::now();
    let mut compressed = compress(&image, &header, &color_transform)?;
    println!(
        "Compression took {} microseconds",
        compress_start
            .to(PreciseTime::now())
            .num_microseconds()
            .unwrap_or(i64::MAX)
    );
    {
        let mut f = fs::File::create(output_gfwx)?;
        f.write_all(&compressed)?;
    }

    let decompress_start = PreciseTime::now();
    let decompressed = decompress(&mut compressed, downsampling)?;
    println!(
        "Decompression took {} microseconds",
        decompress_start
            .to(PreciseTime::now())
            .num_microseconds()
            .unwrap_or(i64::MAX)
    );

    let downsampled_width = (header.width as u32 + (1 << downsampling) - 1) >> downsampling;
    let downsampled_height = (header.height as u32 + (1 << downsampling) - 1) >> downsampling;
    let decompressed_image = match intent {
        gfwx::Intent::RGB => ImageRgb8(
            image::RgbImage::from_raw(downsampled_width, downsampled_height, decompressed).unwrap(),
        ),
        gfwx::Intent::RGBA => ImageRgba8(
            image::RgbaImage::from_raw(downsampled_width, downsampled_height, decompressed)
                .unwrap(),
        ),
        gfwx::Intent::BGR => ImageBgr8(
            image::ImageBuffer::<image::Bgr<u8>, Vec<u8>>::from_raw(
                downsampled_width,
                downsampled_height,
                decompressed,
            )
            .unwrap(),
        ),
        gfwx::Intent::BGRA => ImageBgra8(
            image::ImageBuffer::<image::Bgra<u8>, Vec<u8>>::from_raw(
                downsampled_width,
                downsampled_height,
                decompressed,
            )
            .unwrap(),
        ),
        gfwx::Intent::YUV444 => {
            let yuv420 = gfwx::sequential_yuv444_to_yuv420(
                &decompressed,
                downsampled_width as usize,
                downsampled_height as usize,
            );
            let rgba32 = gfwx::yuv420_to_rgba32(
                &yuv420,
                downsampled_width as usize,
                downsampled_height as usize,
            );
            ImageRgba8(
                image::RgbaImage::from_raw(downsampled_width, downsampled_height, rgba32).unwrap(),
            )
        }
        gfwx::Intent::Generic => unreachable!(),
    };

    match intent {
        gfwx::Intent::BGR => decompressed_image.to_rgb().save(&output_decompressed)?,
        gfwx::Intent::BGRA => decompressed_image.to_rgba().save(&output_decompressed)?,
        _ => decompressed_image.save(&output_decompressed)?,
    }

    Ok(())
}

fn compress(
    image: &Vec<u8>,
    header: &gfwx::Header,
    color_transform: &gfwx::ColorTransformProgram,
) -> Result<Vec<u8>, gfwx::CompressError> {
    let mut compressed = vec![0; 2 * image.len()];

    let gfwx_size = match header.intent {
        gfwx::Intent::YUV444 => gfwx::compress_sequential_channels(
            &image,
            &header,
            &mut compressed,
            &[],
            &color_transform,
        )?,
        _ => gfwx::compress(&image, &header, &mut compressed, &[], &color_transform)?,
    };

    compressed.truncate(gfwx_size);
    Ok(compressed)
}

fn decompress(data: &mut Vec<u8>, downsampling: usize) -> Result<Vec<u8>, gfwx::DecompressError> {
    let mut cursor = io::Cursor::new(data);
    let header = gfwx::Header::decode(&mut cursor).unwrap();
    let header_size = cursor.position() as usize;
    let data = cursor.into_inner();

    let downsampled_width = (header.width as usize + (1 << downsampling) - 1) >> downsampling;
    let downsampled_height = (header.height as usize + (1 << downsampling) - 1) >> downsampling;
    let downsampled_len =
        downsampled_width * downsampled_height * header.layers as usize * header.channels as usize;
    let mut decompressed = vec![0; header.get_estimated_decompress_buffer_size()];
    let _next_point_of_interest = match header.intent {
        gfwx::Intent::YUV444 => gfwx::decompress_sequential_channels(
            &mut data[header_size..],
            &header,
            &mut decompressed,
            downsampling,
            false,
        )?,
        _ => gfwx::decompress(
            &mut data[header_size..],
            &header,
            &mut decompressed,
            downsampling,
            false,
        )?,
    };
    decompressed.truncate(downsampled_len);

    Ok(decompressed)
}

fn get_matches() -> clap::ArgMatches<'static> {
    clap::App::new("gfwx-rs test app")
        .version("1.0")
        .about("test app for gfwx-rs library")
        .arg(
            clap::Arg::with_name("INPUT")
                .help("Sets the input image file to use")
                .required(true)
                .index(1),
        )
        .arg(
            clap::Arg::with_name("OUTPUT.GFWX")
                .help("Sets the output file to write compressed gfwx")
                .required(true)
                .index(2),
        )
        .arg(
            clap::Arg::with_name("OUTPUT")
                .help("Sets the output file to write decompressed image")
                .required(true)
                .index(3),
        )
        .arg(
            clap::Arg::with_name("quality")
                .help("Sets the quality of compression, ranges from 1 to 1024")
                .short("q")
                .long("quality")
                .takes_value(true)
                .default_value("1024")
                .validator(|v| {
                    let v = v.parse::<u16>().map_err(|e| e.to_string())?;
                    if v > 0 && v <= 1024 {
                        Ok(())
                    } else {
                        Err("Quality must be in range (1..=1024)".to_string())
                    }
                }),
        )
        .arg(
            clap::Arg::with_name("chroma-scale")
                .help("Sets the chroma scale for compression")
                .short("c")
                .long("chroma-scale")
                .takes_value(true)
                .default_value("8")
                .validator(|v| {
                    v.parse::<u8>().map_err(|e| e.to_string())?;
                    Ok(())
                }),
        )
        .arg(
            clap::Arg::with_name("downsampling")
                .help("Sets the downsampling scale for decompression")
                .short("d") .long("downsampling")
                .takes_value(true)
                .default_value("0")
                .validator(|v| {
                    v.parse::<u8>().map_err(|e| e.to_string())?;
                    Ok(())
                }),
        )
        .arg(
            clap::Arg::with_name("block-size")
                .help("Sets the block size for compression, ranges from 1 to 30")
                .long("block-size")
                .takes_value(true)
                .default_value("7")
                .validator(|v| {
                    let v = v.parse::<u8>().map_err(|e| e.to_string())?;
                    if v > 0 && v <= 30 {
                        Ok(())
                    } else {
                        Err("Block size must be in range (1..=30)".to_string())
                    }
                }),
        )
        .arg(
            clap::Arg::with_name("filter")
                .help("Set the filter for lifting scheme")
                .short("f")
                .long("filter")
                .takes_value(true)
                .default_value("linear")
                .possible_values(&["linear", "cubic"]),
        )
        .arg(
            clap::Arg::with_name("encoder")
                .help("Set the encoder mode")
                .short("e")
                .long("encoder")
                .takes_value(true)
                .default_value("contextual")
                .possible_values(&["turbo", "fast", "contextual"]),
        )
        .arg(
            clap::Arg::with_name("intent")
                .help("Set the image intentional color space. If not specified, original image intent is used")
                .short("i")
                .long("intent")
                .takes_value(true)
                .possible_values(&["rgb", "rgba", "bgr", "bgra", "yuv420"]),
        )
        .get_matches()
}

fn get_raw_image(
    image: image::DynamicImage,
    user_intent: Option<&str>,
) -> (u32, u32, Vec<u8>, u16, gfwx::Intent) {
    match user_intent {
        Some(v) => match v {
            "rgb" => {
                let i = image.to_rgb();
                (i.width(), i.height(), i.into_raw(), 3, gfwx::Intent::RGB)
            }
            "rgba" => {
                let i = image.to_rgba();
                (i.width(), i.height(), i.into_raw(), 4, gfwx::Intent::RGBA)
            }
            "bgr" => {
                let i = image.to_bgr();
                (i.width(), i.height(), i.into_raw(), 3, gfwx::Intent::BGR)
            }
            "bgra" => {
                let i = image.to_bgra();
                (i.width(), i.height(), i.into_raw(), 4, gfwx::Intent::BGRA)
            }
            "yuv420" => {
                let rgba = image.to_rgba();
                let width = rgba.width();
                let height = rgba.height();
                let raw = rgba.into_raw();
                let yuv420 = gfwx::rgba32_to_yuv420(&raw, width as usize, height as usize);
                let yuv444 =
                    gfwx::yuv420_to_sequential_yuv444(&yuv420, width as usize, height as usize);
                (width, height, yuv444, 3, gfwx::Intent::YUV444)
            }
            _ => panic!("clap?"),
        },
        None => match image {
            ImageRgb8(i) => (i.width(), i.height(), i.into_raw(), 3, gfwx::Intent::RGB),
            ImageBgr8(i) => (i.width(), i.height(), i.into_raw(), 3, gfwx::Intent::BGR),
            ImageRgba8(i) => (i.width(), i.height(), i.into_raw(), 4, gfwx::Intent::RGBA),
            ImageBgra8(i) => (i.width(), i.height(), i.into_raw(), 4, gfwx::Intent::BGRA),
            _ => panic!("unsupported image color space"),
        },
    }
}
