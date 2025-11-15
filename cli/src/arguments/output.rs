use clap::ValueEnum;

#[derive(ValueEnum, Debug, Clone)]
pub enum OutputFormat {
    JpegXl,
    Ppm,
    None,
}
