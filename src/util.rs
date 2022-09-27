
pub fn hex_to_rgba(s: &str) -> Option<[u8; 4]> {
    let s = s.trim_start_matches('#');

    match s.len() {
        6 => {
            let r = u8::from_str_radix(&s[0..2], 16).ok()?;
            let g = u8::from_str_radix(&s[2..4], 16).ok()?;
            let b = u8::from_str_radix(&s[4..6], 16).ok()?;
            Some([r, g, b, 255])
        },
        8 => {
            let r = u8::from_str_radix(&s[0..2], 16).ok()?;
            let g = u8::from_str_radix(&s[2..4], 16).ok()?;
            let b = u8::from_str_radix(&s[4..6], 16).ok()?;
            let a = u8::from_str_radix(&s[6..8], 16).ok()?;
            Some([r, g, b, a])
        },
        _ => None,
    }
}

pub fn rgba_to_hex(color: [u8; 4]) -> String {
    format!("#{:02x}{:02x}{:02x}{:02x}", color[0], color[1], color[2], color[3])
}