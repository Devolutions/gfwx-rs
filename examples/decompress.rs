use std::{error::Error, fs, i64, io, io::prelude::*, path::Path};

use image::DynamicImage::*;
use time::PreciseTime;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = get_matches();
    let input_file = matches.value_of("INPUT").unwrap();
    let output_file = matches.value_of("OUTPUT").unwrap();
    let downsampling = matches.value_of("downsampling").unwrap().parse().unwrap();

    let mut input = fs::File::open(input_file)?;
    let header = gfwx::Header::decode(&mut input)?;

    let mut compressed = Vec::new();
    input.read_to_end(&mut compressed)?;

    let mut decompressed = vec![0; header.get_decompress_buffer_size(downsampling)];

    let decompress_start = PreciseTime::now();
    gfwx::decompress_simple(&compressed, &header, downsampling, false, &mut decompressed)?;
    let decompress_end = PreciseTime::now();

    println!(
        "Decompression took {} microseconds",
        decompress_start
            .to(decompress_end)
            .num_microseconds()
            .unwrap_or(i64::MAX)
    );

    save_image(
        decompressed,
        header.intent,
        header.get_downsampled_width(downsampling) as u32,
        header.get_downsampled_height(downsampling) as u32,
        &output_file,
    )?;

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
                .help("Sets the output file to write decompressed image")
                .required(true)
                .index(2),
        )
        .arg(
            clap::Arg::with_name("downsampling")
                .help("Sets the downsampling scale for decompression")
                .short("d")
                .long("downsampling")
                .takes_value(true)
                .default_value("0")
                .validator(|v| {
                    v.parse::<u8>().map_err(|e| e.to_string())?;
                    Ok(())
                }),
        )
        .get_matches()
}

fn save_image(
    decompressed: Vec<u8>,
    intent: gfwx::Intent,
    width: u32,
    height: u32,
    path: impl AsRef<Path>,
) -> io::Result<()> {
    let decompressed_image = match intent {
        gfwx::Intent::RGB => {
            ImageRgb8(image::RgbImage::from_raw(width, height, decompressed).unwrap())
        }
        gfwx::Intent::RGBA => {
            ImageRgba8(image::RgbaImage::from_raw(width, height, decompressed).unwrap())
        }
        gfwx::Intent::BGR => ImageBgr8(
            image::ImageBuffer::<image::Bgr<u8>, Vec<u8>>::from_raw(width, height, decompressed)
                .unwrap(),
        ),
        gfwx::Intent::BGRA => ImageBgra8(
            image::ImageBuffer::<image::Bgra<u8>, Vec<u8>>::from_raw(width, height, decompressed)
                .unwrap(),
        ),
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unsupported image intent",
            ))
        }
    };
    match intent {
        gfwx::Intent::BGR => decompressed_image.to_rgb().save(&path),
        gfwx::Intent::BGRA => decompressed_image.to_rgba().save(&path),
        _ => decompressed_image.save(&path),
    }
}
