use image::DynamicImage;

pub struct Layout {
    pub width: u32,
    pub height: u32,
    pub offsets: Vec<(u32, u32)>,
}

impl Layout {
    pub fn new(width: u32, height: u32, offset: Vec<(u32, u32)>) -> Self {
        Layout {
            width,
            height,
            offsets: offset,
        }
    }

    pub fn vertical(imgs: &[DynamicImage], spacing: u32) -> Layout {
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

    pub fn horizontal(imgs: &[DynamicImage], spacing: u32) -> Layout {
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
