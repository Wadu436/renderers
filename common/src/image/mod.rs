pub mod jxl;
pub mod png;
pub mod ppm;

use std::io::{self, Write};

use crate::surface::Surface;

pub trait ImageFormat {
    fn save<W: Write>(&self, surface: &Surface, writer: W) -> io::Result<()>;
}
