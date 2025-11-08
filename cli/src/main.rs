use clap::Parser;
use cli::arguments::Args;
use cli::run;
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    run(args)
}
