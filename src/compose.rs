use std::path::Path;

use anyhow::{Context, Result};
use image::{DynamicImage, GenericImage, ImageBuffer, Rgba};

use crate::layout::Layout;

pub fn compose_and_save(
    imgs: &[DynamicImage],
    layout: Layout,
    bg: Rgba<u8>,
    output: &Path,
) -> Result<()> {
    let mut canvas: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_pixel(layout.width, layout.height, bg);

    for (img, (ox, oy)) in imgs.iter().zip(layout.offsets.into_iter()) {
        canvas
            .copy_from(img, ox, oy)
            .with_context(|| "compositing image")?;
    }

    canvas
        .save(output)
        .with_context(|| format!("saving '{}'", output.display()))?;

    println!(
        "✅ Wrote {output:?} ({}×{}) with {} image(s)",
        layout.width,
        layout.height,
        imgs.len()
    );

    Ok(())
}
