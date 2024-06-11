use std::{ffi::OsStr, fs, path::Path};

use anyhow::{anyhow, Error};
use webp::Encoder;

pub struct WebpConverter;

impl WebpConverter {
    pub fn process_image(input_file: &str, output_path: &Path, quality: f32) -> Result<(), Error> {
        let image_path = Path::new(input_file);
        let file_size = fs::metadata(image_path).unwrap().len();

        let img = image::open(image_path)?;

        let file_name = image_path.file_name().unwrap_or_else(|| {
            println!("Cannot get name from file using default");
            OsStr::new("default")
        });

        println!("Converting {:?}", file_name);

        let encoder = Encoder::from_image(&img).map_err(|_| anyhow!("Failed to create encoder"))?;

        let webp = encoder.encode(quality);

        let output_path = output_path.join(file_name).with_extension("webp");
        fs::write(&output_path, &*webp).unwrap();

        let new_file_size = fs::metadata(output_path).unwrap().len();
        let percentage_change =
            ((file_size as f64 - new_file_size as f64) / file_size as f64) * 100 as f64;
        println!(
            "Saved {:?} KB ({:?}%)",
            (file_size - new_file_size) / 1024,
            percentage_change as u64
        );

        Ok(())
    }
}
