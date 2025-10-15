use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
pub struct Cli {
    /// The images to combine
    #[arg(required = true)]
    pub images: Vec<String>,

    /// The output image path (png, jpg, etc.)
    #[arg(short, long, default_value = "output.png")]
    pub output: PathBuf,

    /// The layout direction of the images
    #[arg(short, long, default_value = "h")]
    pub layout: LayoutCli,

    /// The spacing (pixels) between images
    #[arg(short, long, default_value_t = 0)]
    pub spacing: u32,

    /// The background color in hex (#RRGGBB or #RRGGBBAA)
    #[arg(short, long, default_value = "#ffffff")]
    pub bg: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum LayoutCli {
    /// Horizontal layout
    H,
    /// Vertical layout
    V,
}
