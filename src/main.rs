mod cli;
mod colors;
mod compose;
mod layout;
mod util;

use clap::Parser;

use crate::cli::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let paths = util::expand_inputs(&cli.images)?;
    let imgs = util::load_images(&paths)?;

    if imgs.len() < 2 {
        anyhow::bail!("At least two images is required");
    }

    let layout = cli.layout.compute(&imgs, cli.spacing);

    compose::compose_and_save(&imgs, &layout, cli.bg, &cli.output)?;

    println!(
        "✅ Wrote {:?} ({}×{}) with {} image(s)",
        cli.output,
        layout.width,
        layout.height,
        imgs.len()
    );

    Ok(())
}
