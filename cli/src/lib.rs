use color_eyre::eyre::Result;
use std::fs::OpenOptions;

use common::{
    image::{ImageFormat, jxl::JpegXl},
    surface::Surface,
};

pub fn run() -> Result<()> {
    // Set up
    let surface = Surface::new(600, 400);

    // Render
    // Skip for now

    // Save the file
    let jxl_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("./render.jxl")?;
    let jxl = JpegXl { lossless: true };
    jxl.save(&surface, jxl_file)?;

    Ok(())
}
