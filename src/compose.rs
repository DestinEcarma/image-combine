use std::path::Path;

use anyhow::Context;
use image::{DynamicImage, GenericImage, ImageBuffer, Rgba};

use crate::layout::Layout;

pub fn compose_and_save(
    imgs: &[DynamicImage],
    layout: &Layout,
    bg: Rgba<u8>,
    output: &Path,
) -> anyhow::Result<()> {
    let mut canvas: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_pixel(layout.width, layout.height, bg);

    for (img, (ox, oy)) in imgs.iter().zip(layout.offsets.clone().into_iter()) {
        canvas
            .copy_from(img, ox, oy)
            .with_context(|| "compositing image")?;
    }

    canvas
        .save(output)
        .with_context(|| format!("saving '{}'", output.display()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::LayoutKind;
    use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
    use tempfile::tempdir;

    fn solid_image(w: u32, h: u32, px: Rgba<u8>) -> DynamicImage {
        DynamicImage::ImageRgba8(ImageBuffer::from_pixel(w, h, px))
    }

    #[test]
    fn compose_horizontal_saves_expected_dimensions_and_pixels() {
        let img1 = solid_image(2, 2, Rgba([255, 0, 0, 255]));
        let img2 = solid_image(2, 2, Rgba([0, 0, 255, 255]));
        let imgs = vec![img1, img2];

        let layout = LayoutKind::Horizontal.compute(&imgs, 1);

        let dir = tempdir().unwrap();
        let out = dir.path().join("out.png");

        compose_and_save(&imgs, &layout, Rgba([255, 255, 255, 255]), &out).unwrap();

        let written = image::open(&out).unwrap();

        assert_eq!(written.dimensions(), (5, 2));
        assert_eq!(written.get_pixel(0, 0), Rgba([255, 0, 0, 255]));
        assert_eq!(written.get_pixel(1, 1), Rgba([255, 0, 0, 255]));
        assert_eq!(written.get_pixel(2, 0), Rgba([255, 255, 255, 255])); // spacing bg
        assert_eq!(written.get_pixel(3, 0), Rgba([0, 0, 255, 255]));
        assert_eq!(written.get_pixel(4, 1), Rgba([0, 0, 255, 255]));
    }

    #[test]
    fn compose_vertical_centers_narrower_image_and_fills_background() {
        let img1 = solid_image(4, 2, Rgba([255, 0, 0, 255]));
        let img2 = solid_image(2, 2, Rgba([0, 255, 0, 255]));
        let imgs = vec![img1, img2];

        let layout = LayoutKind::Vertical.compute(&imgs, 1);

        let dir = tempdir().unwrap();
        let out = dir.path().join("out.png");

        let bg = Rgba([10, 20, 30, 255]);
        compose_and_save(&imgs, &layout, bg, &out).unwrap();

        let written = image::open(&out).unwrap();

        assert_eq!(written.dimensions(), (4, 5));

        assert_eq!(written.get_pixel(0, 0), Rgba([255, 0, 0, 255]));
        assert_eq!(written.get_pixel(3, 1), Rgba([255, 0, 0, 255]));

        assert_eq!(written.get_pixel(0, 2), bg); // spacing row
        assert_eq!(written.get_pixel(3, 2), bg);

        assert_eq!(written.get_pixel(0, 3), bg); // left padding for centered 2px image
        assert_eq!(written.get_pixel(1, 3), Rgba([0, 255, 0, 255]));
        assert_eq!(written.get_pixel(2, 4), Rgba([0, 255, 0, 255]));
        assert_eq!(written.get_pixel(3, 4), bg); // right padding
    }
}
