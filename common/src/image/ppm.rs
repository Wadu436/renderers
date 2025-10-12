use std::io::{self, Write};

use crate::{
    image::ImageFormat,
    surface::{Surface, format::RGBA8},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PpmFormat {
    Ascii,
    Binary,
}

impl PpmFormat {
    fn magic_number(&self) -> &'static str {
        match self {
            Self::Ascii => "P3",
            Self::Binary => "P6",
        }
    }

    fn write_pixel<W: io::Write>(&self, mut writer: W, px: &RGBA8) -> io::Result<()> {
        match self {
            Self::Ascii => writeln!(writer, "{} {} {}", px.r, px.g, px.b),
            Self::Binary => writer.write_all(&[px.r, px.g, px.b]),
        }
    }
}

pub struct Ppm {
    pub format: PpmFormat,
}

impl ImageFormat for Ppm {
    fn save<W: Write>(&self, surface: &Surface, mut writer: W) -> io::Result<()> {
        // Header
        // P6 (magic number)
        // Width Height
        // Max value
        write!(
            writer,
            "{}\n{} {}\n255\n",
            self.format.magic_number(),
            surface.width,
            surface.height
        )?;

        // Write the pixel data.
        // Note: PPM doesn't support the Alpha channel
        for px in surface.buffer.iter() {
            self.format.write_pixel(&mut writer, px)?;
        }

        Ok(())
    }
}
