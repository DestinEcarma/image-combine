use anyhow::{Result, bail};
use image::Rgba;

pub fn parse_rgba(s: &str) -> Result<Rgba<u8>> {
    let t = s.trim();
    let hex = t.strip_prefix('#').unwrap_or(t);

    let (r, g, b, a) = match hex.len() {
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16)?;
            let g = u8::from_str_radix(&hex[2..4], 16)?;
            let b = u8::from_str_radix(&hex[4..6], 16)?;
            (r, g, b, 255)
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16)?;
            let g = u8::from_str_radix(&hex[2..4], 16)?;
            let b = u8::from_str_radix(&hex[4..6], 16)?;
            let a = u8::from_str_radix(&hex[6..8], 16)?;
            (r, g, b, a)
        }
        _ => bail!("expected #RRGGBB or #RRGGBBAA"),
    };

    Ok(Rgba([r, g, b, a]))
}
