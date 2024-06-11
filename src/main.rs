use std::{fs, path::Path};

use webp::Encoder;

fn main() {
    let img = image::open("./premium-single-1.jpg").unwrap();
    let encoder: Encoder = Encoder::from_image(&img).unwrap();
    let webp = encoder.encode(70f32);

    let output_path = Path::new("assets").join("example").with_extension("webp");
    fs::write(&output_path, &*webp).unwrap();
}
