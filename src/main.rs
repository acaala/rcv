use std::{ffi::OsStr, fs, path::Path, process};

use anyhow::{Error, Result};
use clap::{arg, command, Parser};
use webp::Encoder;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input_file: String,

    #[arg(short, long)]
    output_path: String,

    #[arg(short, long, default_value_t = 75f32)]
    quality: f32,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = process_image(&args.input_file, &args.output_path, args.quality) {
        eprintln!("Error processing image: {:?}", e);
        process::exit(1)
    }
}

fn process_image(input_file: &str, output_path: &str, quality: f32) -> Result<(), Error> {
    let img = image::open(input_file);

    let img = match img {
        Ok(i) => i,
        Err(_e) => {
            eprintln!("Error: No such image");
            process::exit(1)
        }
    };

    let encoder = Encoder::from_image(&img);

    let encoder = match encoder {
        Ok(e) => e,
        Err(err) => {
            eprintln!("Error encoding image: {:?}", err);
            process::exit(1)
        }
    };

    let webp = encoder.encode(quality);

    let file_name = Path::new(input_file).file_name().unwrap_or_else(|| {
        println!("Cannot get name from file using default");
        OsStr::new("default")
    });

    let output_dir = Path::new(&output_path);
    fs::create_dir_all(&output_dir)?;

    let output_path = Path::new(output_dir).join(file_name).with_extension("webp");
    fs::write(&output_path, &*webp).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_image() {
        let input_file = "test_img.jpg";
        let output_path = "./assets";
        let quality = 70.0;

        let result = process_image(input_file, output_path, quality);

        assert!(result.is_ok())
    }
}
