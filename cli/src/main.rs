use cli::run;
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    run()
}
