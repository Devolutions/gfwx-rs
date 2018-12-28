use std::{error::Error, fs, i64, io::prelude::*, path::Path};

use image::DynamicImage::*;
use time::PreciseTime;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = get_matches();
    let input_file = matches.value_of("INPUT").unwrap();
    let output_gfwx = matches.value_of("OUTPUT").unwrap();
    let quality = matches.value_of("quality").unwrap().parse().unwrap();
    let block_size = matches.value_of("block-size").unwrap().parse().unwrap();
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
    let (width, height, image, channels, intent) =
        into_raw_image(image, matches.value_of("intent"));

    let builder = gfwx::HeaderBuilder {
        width,
        height,
        layers: 1,
        channels,
        quality,
        chroma_scale: 8,
        block_size,
        filter,
        encoder,
        intent,
        metadata_size: 0,
    };
    let header = builder.build().unwrap();

    let mut compressed = vec![0; 2 * image.len()];

    let compress_start = PreciseTime::now();
    let gfwx_size = gfwx::compress_simple(
        &image,
        &header,
        &gfwx::ColorTransformProgram::new(),
        &mut compressed,
    )?;
    let compress_end = PreciseTime::now();

    println!(
        "Compression took {} microseconds",
        compress_start
            .to(compress_end)
            .num_microseconds()
            .unwrap_or(i64::MAX)
    );

    let mut f = fs::File::create(output_gfwx)?;
    f.write_all(&compressed[0..gfwx_size])?;

    Ok(())
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
            clap::Arg::with_name("OUTPUT")
                .help("Sets the output file to write compressed gfwx")
                .required(true)
                .index(2),
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
                .default_value("turbo")
                .possible_values(&["turbo", "fast", "contextual"]),
        )
        .arg(
            clap::Arg::with_name("intent")
                .help("Set the image intentional color space. If not specified, original image intent is used")
                .short("i")
                .long("intent")
                .takes_value(true)
                .possible_values(&["rgb", "rgba", "bgr", "bgra"]),
        )
        .get_matches()
}

fn into_raw_image(
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
