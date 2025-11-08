use clap::Parser;

pub mod output;
pub mod renderer;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, value_hint = clap::ValueHint::FilePath)]
    pub format: output::OutputFormat,

    #[arg(short, long, value_enum)]
    pub renderer: renderer::Renderer,
}
