use std::{fs, path::Path, process};

use anyhow::{bail, Result};
use clap::{arg, command, Parser};
use rcv::webp_converter::WebpConverter;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = None)]
    input_file: Option<String>,

    #[arg(short, long, default_value = None)]
    directory: Option<String>,

    #[arg(short, long)]
    output_path: String,

    #[arg(short, long, default_value_t = 75f32)]
    quality: f32,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.input_file.is_none() && args.directory.is_none() {
        eprintln!("Error: An input file (--input) or a directory (--directory) must be used");
        process::exit(0)
    }

    let output_path = Path::new(&args.output_path);
    fs::create_dir_all(&output_path).unwrap_or_else(|_| {
        eprintln!("Failed to create output path");
        process::exit(1)
    });

    match args.directory.is_some() {
        true => {
            if let Err(_) = WebpConverter::process_directory(
                &args.directory.unwrap(),
                output_path,
                args.quality,
            ) {
                bail!("Error: Failed to open directory");
            }
        }
        false => {
            if let Err(err) =
                WebpConverter::process_image(&args.input_file.unwrap(), &output_path, args.quality)
            {
                bail!("Failed to process file - {:?}", err);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_image() {
        let input_file = "./test_assets/test_img.jpg";
        let output_path = Path::new("./assets");
        let quality = 70.0;

        let result = WebpConverter::process_image(input_file, output_path, quality);

        assert!(result.is_ok())
    }

    #[test]
    fn test_process_directory() {
        let directory = "test_assets";
        let output_path = Path::new("./assets");
        let quality = 70.0;

        let result = WebpConverter::process_directory(directory, output_path, quality);

        assert!(result.is_ok())
    }
}
