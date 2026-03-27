use clap::ValueEnum;
use image::DynamicImage;

pub struct Layout {
    pub width: u32,
    pub height: u32,
    pub offsets: Vec<(u32, u32)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum LayoutKind {
    Horizontal,
    Vertical,
}

impl LayoutKind {
    pub fn compute(&self, imgs: &[DynamicImage], spacing: u32) -> Layout {
        match self {
            Self::Horizontal => Layout::horizontal(imgs, spacing),
            Self::Vertical => Layout::vertical(imgs, spacing),
        }
    }
}

impl Layout {
    fn new(width: u32, height: u32, offset: Vec<(u32, u32)>) -> Self {
        Layout {
            width,
            height,
            offsets: offset,
        }
    }

    fn vertical(imgs: &[DynamicImage], spacing: u32) -> Layout {
        let total_h = imgs
            .iter()
            .map(|im| im.height())
            .fold(0u32, |acc, h| acc.saturating_add(h));

        let width = imgs.iter().map(|im| im.width()).max().unwrap_or(0);
        let height = total_h + spacing.saturating_mul(imgs.len().saturating_sub(1) as u32);

        let mut y = 0u32;
        let mut offsets = Vec::with_capacity(imgs.len());

        for im in imgs {
            let ox = (width.saturating_sub(im.width())) / 2;

            offsets.push((ox, y));

            y = y.saturating_add(im.height()).saturating_add(spacing);
        }

        Self::new(width, height, offsets)
    }

    fn horizontal(imgs: &[DynamicImage], spacing: u32) -> Layout {
        let total_w = imgs
            .iter()
            .map(|im| im.width())
            .fold(0u32, |acc, w| acc.saturating_add(w));

        let width = total_w + spacing.saturating_mul(imgs.len().saturating_sub(1) as u32);
        let height = imgs.iter().map(|im| im.height()).max().unwrap_or(0);

        let mut x = 0u32;
        let mut offsets = Vec::with_capacity(imgs.len());

        for im in imgs {
            let oy = (height.saturating_sub(im.height())) / 2;

            offsets.push((x, oy));

            x = x.saturating_add(im.width()).saturating_add(spacing);
        }

        Self::new(width, height, offsets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::DynamicImage;

    fn img(w: u32, h: u32) -> DynamicImage {
        DynamicImage::new_rgba8(w, h)
    }

    #[test]
    fn horizontal_layout_computes_canvas_and_offsets() {
        let imgs = vec![img(10, 20), img(30, 10), img(5, 40)];

        let layout = Layout::horizontal(&imgs, 3);

        assert_eq!(layout.width, 10 + 30 + 5 + 3 * 2);
        assert_eq!(layout.height, 40);
        assert_eq!(
            layout.offsets,
            vec![
                (0, 10),  // centered in 40px tall canvas
                (13, 15), // 10 + 3 = 13
                (46, 0),  // 13 + 30 + 3 = 46
            ]
        );
    }

    #[test]
    fn vertical_layout_computes_canvas_and_offsets() {
        let imgs = vec![img(10, 20), img(30, 10), img(20, 5)];

        let layout = Layout::vertical(&imgs, 4);

        assert_eq!(layout.width, 30);
        assert_eq!(layout.height, 20 + 10 + 5 + 4 * 2);
        assert_eq!(
            layout.offsets,
            vec![
                (10, 0), // centered in 30px wide canvas
                (0, 24), // 20 + 4 = 24
                (5, 38), // 24 + 10 + 4 = 38
            ]
        );
    }

    #[test]
    fn horizontal_empty_input_is_zero_sized() {
        let imgs = vec![];
        let layout = Layout::horizontal(&imgs, 99);

        assert_eq!(layout.width, 0);
        assert_eq!(layout.height, 0);
        assert!(layout.offsets.is_empty());
    }

    #[test]
    fn vertical_empty_input_is_zero_sized() {
        let imgs = vec![];
        let layout = Layout::vertical(&imgs, 99);

        assert_eq!(layout.width, 0);
        assert_eq!(layout.height, 0);
        assert!(layout.offsets.is_empty());
    }

    #[test]
    fn single_image_has_no_spacing_penalty_horizontal() {
        let imgs = vec![img(25, 8)];
        let layout = Layout::horizontal(&imgs, 50);

        assert_eq!(layout.width, 25);
        assert_eq!(layout.height, 8);
        assert_eq!(layout.offsets, vec![(0, 0)]);
    }

    #[test]
    fn single_image_has_no_spacing_penalty_vertical() {
        let imgs = vec![img(25, 8)];
        let layout = Layout::vertical(&imgs, 50);

        assert_eq!(layout.width, 25);
        assert_eq!(layout.height, 8);
        assert_eq!(layout.offsets, vec![(0, 0)]);
    }
}
