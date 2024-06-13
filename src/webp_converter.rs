use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    thread,
};

use anyhow::{anyhow, Error};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
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

        // println!("Converting {:?}", file_name);

        let encoder = Encoder::from_image(&img).map_err(|_| anyhow!("Failed to create encoder"))?;

        let webp = encoder.encode(quality);

        let output_path = output_path.join(file_name).with_extension("webp");
        fs::write(&output_path, &*webp).unwrap();

        let new_file_size = fs::metadata(output_path).unwrap().len();
        let percentage_change =
            ((file_size as f64 - new_file_size as f64) / file_size as f64) * 100_f64;
        // println!(
        //     "Saved {:?} KB ({:?}%)",
        //     (file_size - new_file_size) / 1024,
        //     percentage_change as u64
        // );

        Ok(())
    }

    pub fn process_directory(
        dir: &str,
        output_path: &Path,
        quality: f32,
        count: &mut u64,
    ) -> Result<(), Error> {
        let files = Self::get_files_in_dir(dir)?;

        let m = ProgressBar::new(files.len() as u64);
        let sty = ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} Converting..",
        )
        .unwrap()
        .progress_chars("##-");
        m.set_style(sty);

        files.par_iter().for_each(|file| {
            let file_name = file.to_str().unwrap();

            if WebpConverter::process_image(file_name, output_path, quality).is_err() {
                eprintln!(
                    "Error processing file: {:?} - Skipping...",
                    file.file_name().unwrap()
                );
            }
            m.inc(1);
        });

        *count = files.len() as u64;

        Ok(())
    }

    fn get_files_in_dir(dir: &str) -> Result<Vec<PathBuf>, Error> {
        let mut files = Vec::new();
        println!("Looking for images..");
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && Self::is_image_file(&path) {
                files.push(path);
            }
        }
        println!("Found {} images", files.len());

        Ok(files)
    }

    fn is_image_file(file: &Path) -> bool {
        image::open(file).is_ok()
    }
}
