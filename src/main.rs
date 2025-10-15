mod cli;
mod colors;
mod compose;
mod layout;
mod util;

use anyhow::{Result, bail};
use clap::Parser;

use crate::{
    cli::{Cli, LayoutCli},
    layout::Layout,
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let paths = util::expand_inputs(&cli.images)?;
    let imgs = util::load_images(&paths)?;

    if imgs.len() < 2 {
        bail!("At least two images is required");
    }

    let layout = match cli.layout {
        LayoutCli::H => Layout::horizontal(&imgs, cli.spacing),
        LayoutCli::V => Layout::vertical(&imgs, cli.spacing),
    };

    let bg = colors::parse_rgba(&cli.bg)?;

    compose::compose_and_save(&imgs, layout, bg, &cli.output)
}
