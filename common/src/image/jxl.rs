use std::io;

use jpegxl_rs::{encode::EncoderFrame, encoder_builder};

use crate::{image::ImageFormat, surface::Surface};

pub struct JpegXl {
    pub lossless: bool,
}

impl ImageFormat for JpegXl {
    fn save<W: std::io::Write>(&self, surface: &Surface, mut writer: W) -> std::io::Result<()> {
        let data: Vec<u8> = surface
            .buffer
            .iter()
            .flat_map(|px| [px.r, px.g, px.b, px.a])
            .collect();

        let mut encoder = encoder_builder()
            .has_alpha(true)
            .quality(1.0)
            .build()
            .map_err(|e| io::Error::other(format!("build encoder: {e}")))?;

        let frame = EncoderFrame::new(&data).num_channels(4);
        let encoded_frame = encoder
            .encode_frame::<_, u8>(&frame, surface.width, surface.height)
            .map_err(|e| io::Error::other(format!("encode frame: {e}")))?;
        writer.write_all(&encoded_frame.data)
    }
}
