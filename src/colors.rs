use image::Rgba;

pub fn parse_rgba(s: &str) -> anyhow::Result<Rgba<u8>> {
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
        _ => anyhow::bail!("expected #RRGGBB or #RRGGBBAA"),
    };

    Ok(Rgba([r, g, b, a]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::Rgba;

    #[test]
    fn parses_rgb_without_alpha() {
        let c = parse_rgba("#ffcc00").unwrap();
        assert_eq!(c, Rgba([0xff, 0xcc, 0x00, 0xff]));
    }

    #[test]
    fn parses_rgba_with_alpha() {
        let c = parse_rgba("#11223344").unwrap();
        assert_eq!(c, Rgba([0x11, 0x22, 0x33, 0x44]));
    }

    #[test]
    fn parses_without_hash_prefix() {
        let c = parse_rgba("abcdef").unwrap();
        assert_eq!(c, Rgba([0xab, 0xcd, 0xef, 0xff]));
    }

    #[test]
    fn trims_whitespace() {
        let c = parse_rgba("  #01020304  ").unwrap();
        assert_eq!(c, Rgba([0x01, 0x02, 0x03, 0x04]));
    }

    #[test]
    fn rejects_wrong_length() {
        let err = parse_rgba("#12345").unwrap_err().to_string();
        assert!(err.contains("expected #RRGGBB or #RRGGBBAA"));
    }

    #[test]
    fn rejects_invalid_hex() {
        assert!(parse_rgba("#gg0000").is_err());
    }
}
