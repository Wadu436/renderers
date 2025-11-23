use std::{path::PathBuf, str::FromStr};

use clap::Parser;

pub mod output;
pub mod renderer;

#[derive(Debug, Clone, Copy)]
pub struct Vec3(pub glam::Vec3);
impl FromStr for Vec3 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(',').collect();
        if parts.len() != 3 {
            return Err("expected a,b,c");
        }

        let x = parts[0].trim().parse::<f32>().map_err(|_| "bad x")?;
        let y = parts[1].trim().parse::<f32>().map_err(|_| "bad y")?;
        let z = parts[2].trim().parse::<f32>().map_err(|_| "bad z")?;
        Ok(Vec3(glam::Vec3::new(x, y, z)))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl FromStr for Resolution {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sep = if s.contains("x") { 'x' } else { ',' };
        let parts: Vec<_> = s.split(sep).collect();
        if parts.len() != 2 {
            return Err("expected <width>x<height> or <width>,<height>");
        }

        let width = parts[0].trim().parse::<u32>().map_err(|_| "bad width")?;
        let height = parts[1].trim().parse::<u32>().map_err(|_| "bad height")?;
        Ok(Resolution { width, height })
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, value_hint = clap::ValueHint::FilePath)]
    pub format: output::OutputFormat,

    #[arg(short, long, value_enum)]
    pub renderer: renderer::Renderer,

    #[arg(short, long, default_value_t = false)]
    pub debug: bool,

    #[arg(short, long)]
    pub camera_origin: Option<Vec3>,

    #[arg(short, long)]
    pub output: Option<PathBuf>,

    #[arg(long)]
    pub resolution: Option<Resolution>,

    pub scene: PathBuf,
}
