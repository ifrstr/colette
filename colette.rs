//! # Colette
//!
//! Convert between truecolor, xterm(1) 256 color and 16 color.
//! Single-header library.
//! Arranged from [tmux].
//!
//! [tmux]: https://github.com/tmux/tmux

/// Convert an RGB triplet to the xterm(1) 256 color palette.
///
/// xterm provides a 6x6x6 color cube (16 - 231) and 24 greys (232 - 255). We
/// map our RGB color to the closest in the cube, also work out the closest
/// grey, and use the nearest of the two.
///
/// Note that the xterm has much lower resolution for darker colors (they are
/// not evenly spread out), so our 6 levels are not evenly spread: 0x0, 0x5f
/// (95), 0x87 (135), 0xaf (175), 0xd7 (215) and 0xff (255). Greys are more
/// evenly spread (8, 18, 28 ... 238).
pub fn color_rgbto256(r: u8, g: u8, b: u8) -> i32 {
    let q2c = [0x00, 0x5f, 0x87, 0xaf, 0xd7, 0xff];

    /* Map RGB to 6x6x6 cube. */
    let qr = color_to_6cube(r);
    let cr = q2c[qr as usize];
    let qg = color_to_6cube(g);
    let cg = q2c[qg as usize];
    let qb = color_to_6cube(b);
    let cb = q2c[qb as usize];

    /* If we have hit the color exactly, return early. */
    if cr == r && cg == g && cb == b {
        return (16 + 36 * qr + 6 * qg + qb) as i32;
    }

    /* Work out the closest grey (average of RGB). */
    let grey_avg: i32 = (((r as i64) + (g as i64) + (b as i64)) / 3) as i32;
    let grey_idx: i32;
    let mut grey = 0;

    if grey_avg > 238 {
        grey_idx = 23;
    } else {
        grey_idx = (grey_avg - 3) / 10;
        grey = 8 + (10 * grey_idx);
    }

    let idx: i32;

    /* Is grey or 6x6x6 color closest? */
    if dist_sq(grey, grey, grey, r as i32, g as i32, b as i32)
        < dist_sq(
            cr as i32, cg as i32, cb as i32, r as i32, g as i32, b as i32,
        )
    {
        idx = 232 + grey_idx;
    } else {
        idx = (16 + 36 * qr + 6 * qg + qb) as i32;
    }

    idx
}

/// Join RGB into a color.
pub fn join_rgb(r: u8, g: u8, b: u8) -> i32 {
    (r as i32) << 16 | (g as i32) << 8 | (b as i32)
}

/// Split color into RGB.
pub fn split_rgb(c: i32, r: &mut u8, g: &mut u8, b: &mut u8) {
    *r = (c >> 16 & 0xff) as u8;
    *g = (c >> 8 & 0xff) as u8;
    *b = (c & 0xff) as u8;
}

/// Convert 256 color to RGB color.
pub fn color_256torgb(c: u8) -> i32 {
    let table = [
        0x000000, 0x800000, 0x008000, 0x808000, 0x000080, 0x800080, 0x008080, 0xc0c0c0, 0x808080,
        0xff0000, 0x00ff00, 0xffff00, 0x0000ff, 0xff00ff, 0x00ffff, 0xffffff, 0x000000, 0x00005f,
        0x000087, 0x0000af, 0x0000d7, 0x0000ff, 0x005f00, 0x005f5f, 0x005f87, 0x005faf, 0x005fd7,
        0x005fff, 0x008700, 0x00875f, 0x008787, 0x0087af, 0x0087d7, 0x0087ff, 0x00af00, 0x00af5f,
        0x00af87, 0x00afaf, 0x00afd7, 0x00afff, 0x00d700, 0x00d75f, 0x00d787, 0x00d7af, 0x00d7d7,
        0x00d7ff, 0x00ff00, 0x00ff5f, 0x00ff87, 0x00ffaf, 0x00ffd7, 0x00ffff, 0x5f0000, 0x5f005f,
        0x5f0087, 0x5f00af, 0x5f00d7, 0x5f00ff, 0x5f5f00, 0x5f5f5f, 0x5f5f87, 0x5f5faf, 0x5f5fd7,
        0x5f5fff, 0x5f8700, 0x5f875f, 0x5f8787, 0x5f87af, 0x5f87d7, 0x5f87ff, 0x5faf00, 0x5faf5f,
        0x5faf87, 0x5fafaf, 0x5fafd7, 0x5fafff, 0x5fd700, 0x5fd75f, 0x5fd787, 0x5fd7af, 0x5fd7d7,
        0x5fd7ff, 0x5fff00, 0x5fff5f, 0x5fff87, 0x5fffaf, 0x5fffd7, 0x5fffff, 0x870000, 0x87005f,
        0x870087, 0x8700af, 0x8700d7, 0x8700ff, 0x875f00, 0x875f5f, 0x875f87, 0x875faf, 0x875fd7,
        0x875fff, 0x878700, 0x87875f, 0x878787, 0x8787af, 0x8787d7, 0x8787ff, 0x87af00, 0x87af5f,
        0x87af87, 0x87afaf, 0x87afd7, 0x87afff, 0x87d700, 0x87d75f, 0x87d787, 0x87d7af, 0x87d7d7,
        0x87d7ff, 0x87ff00, 0x87ff5f, 0x87ff87, 0x87ffaf, 0x87ffd7, 0x87ffff, 0xaf0000, 0xaf005f,
        0xaf0087, 0xaf00af, 0xaf00d7, 0xaf00ff, 0xaf5f00, 0xaf5f5f, 0xaf5f87, 0xaf5faf, 0xaf5fd7,
        0xaf5fff, 0xaf8700, 0xaf875f, 0xaf8787, 0xaf87af, 0xaf87d7, 0xaf87ff, 0xafaf00, 0xafaf5f,
        0xafaf87, 0xafafaf, 0xafafd7, 0xafafff, 0xafd700, 0xafd75f, 0xafd787, 0xafd7af, 0xafd7d7,
        0xafd7ff, 0xafff00, 0xafff5f, 0xafff87, 0xafffaf, 0xafffd7, 0xafffff, 0xd70000, 0xd7005f,
        0xd70087, 0xd700af, 0xd700d7, 0xd700ff, 0xd75f00, 0xd75f5f, 0xd75f87, 0xd75faf, 0xd75fd7,
        0xd75fff, 0xd78700, 0xd7875f, 0xd78787, 0xd787af, 0xd787d7, 0xd787ff, 0xd7af00, 0xd7af5f,
        0xd7af87, 0xd7afaf, 0xd7afd7, 0xd7afff, 0xd7d700, 0xd7d75f, 0xd7d787, 0xd7d7af, 0xd7d7d7,
        0xd7d7ff, 0xd7ff00, 0xd7ff5f, 0xd7ff87, 0xd7ffaf, 0xd7ffd7, 0xd7ffff, 0xff0000, 0xff005f,
        0xff0087, 0xff00af, 0xff00d7, 0xff00ff, 0xff5f00, 0xff5f5f, 0xff5f87, 0xff5faf, 0xff5fd7,
        0xff5fff, 0xff8700, 0xff875f, 0xff8787, 0xff87af, 0xff87d7, 0xff87ff, 0xffaf00, 0xffaf5f,
        0xffaf87, 0xffafaf, 0xffafd7, 0xffafff, 0xffd700, 0xffd75f, 0xffd787, 0xffd7af, 0xffd7d7,
        0xffd7ff, 0xffff00, 0xffff5f, 0xffff87, 0xffffaf, 0xffffd7, 0xffffff, 0x080808, 0x121212,
        0x1c1c1c, 0x262626, 0x303030, 0x3a3a3a, 0x444444, 0x4e4e4e, 0x585858, 0x626262, 0x6c6c6c,
        0x767676, 0x808080, 0x8a8a8a, 0x949494, 0x9e9e9e, 0xa8a8a8, 0xb2b2b2, 0xbcbcbc, 0xc6c6c6,
        0xd0d0d0, 0xdadada, 0xe4e4e4, 0xeeeeee,
    ];

    table[c as usize]
}

/// Convert 256 color to 16 color.
pub fn color_256to16(c: u8) -> i32 {
    let table = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0, 4, 4, 4, 12, 12, 2, 6, 4, 4, 12,
        12, 2, 2, 6, 4, 12, 12, 2, 2, 2, 6, 12, 12, 10, 10, 10, 10, 14, 12, 10, 10, 10, 10, 10, 14,
        1, 5, 4, 4, 12, 12, 3, 8, 4, 4, 12, 12, 2, 2, 6, 4, 12, 12, 2, 2, 2, 6, 12, 12, 10, 10, 10,
        10, 14, 12, 10, 10, 10, 10, 10, 14, 1, 1, 5, 4, 12, 12, 1, 1, 5, 4, 12, 12, 3, 3, 8, 4, 12,
        12, 2, 2, 2, 6, 12, 12, 10, 10, 10, 10, 14, 12, 10, 10, 10, 10, 10, 14, 1, 1, 1, 5, 12, 12,
        1, 1, 1, 5, 12, 12, 1, 1, 1, 5, 12, 12, 3, 3, 3, 7, 12, 12, 10, 10, 10, 10, 14, 12, 10, 10,
        10, 10, 10, 14, 9, 9, 9, 9, 13, 12, 9, 9, 9, 9, 13, 12, 9, 9, 9, 9, 13, 12, 9, 9, 9, 9, 13,
        12, 11, 11, 11, 11, 7, 12, 10, 10, 10, 10, 10, 14, 9, 9, 9, 9, 9, 13, 9, 9, 9, 9, 9, 13, 9,
        9, 9, 9, 9, 13, 9, 9, 9, 9, 9, 13, 9, 9, 9, 9, 9, 13, 11, 11, 11, 11, 11, 15, 0, 0, 0, 0,
        0, 0, 8, 8, 8, 8, 8, 8, 7, 7, 7, 7, 7, 7, 15, 15, 15, 15, 15, 15,
    ];

    table[c as usize]
}

#[allow(non_snake_case)]
fn dist_sq(R: i32, G: i32, B: i32, r: i32, g: i32, b: i32) -> i32 {
    (R - r) * (R - r) + (G - g) * (G - g) + (B - b) * (B - b)
}

fn color_to_6cube(v: u8) -> u8 {
    if v < 48 {
        0
    } else {
        if v < 114 {
            1
        } else {
            (v - 35) / 40
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_color_rgbto256() {
        assert_eq!(16, color_rgbto256(0, 0, 0)); // Black (0 / 16)
        assert_eq!(231, color_rgbto256(255, 255, 255)); // White (15 / 231)
        assert_eq!(196, color_rgbto256(255, 0, 0)); // Red (196)
        assert_eq!(46, color_rgbto256(0, 255, 0)); // Green (46)
        assert_eq!(21, color_rgbto256(0, 0, 255)); // Blue (21)
        assert_eq!(251, color_rgbto256(200, 200, 200)); // Grey (232 - 255)
        assert_eq!(251, color_rgbto256(201, 201, 201)); // Fuzzy match
        assert_eq!(188, color_rgbto256(215, 215, 215)); // Exact match
    }

    #[test]
    fn test_join_rgb() {
        assert_eq!(0x000000, join_rgb(0, 0, 0));
        assert_eq!(0xffffff, join_rgb(255, 255, 255));
        assert_eq!(0xff0000, join_rgb(255, 0, 0));
        assert_eq!(0x00ff00, join_rgb(0, 255, 0));
        assert_eq!(0x0000ff, join_rgb(0, 0, 255));
        assert_eq!(0xc8c8c8, join_rgb(200, 200, 200));
    }

    #[test]
    fn test_split_rgb() {
        let mut r: u8 = 0;
        let mut g: u8 = 0;
        let mut b: u8 = 0;
        split_rgb(0xc8c8c8, &mut r, &mut g, &mut b);
        assert_eq!(200, r);
        assert_eq!(200, g);
        assert_eq!(200, b);
    }

    #[test]
    fn test_color_256torgb() {
        assert_eq!(join_rgb(135, 255, 135), color_256torgb(120));
        assert_eq!(join_rgb(215, 255, 0), color_256torgb(190));
    }

    #[test]
    fn test_color_256to16() {
        assert_eq!(12, color_256to16(21));
        assert_eq!(1, color_256to16(52));
        assert_eq!(9, color_256to16(196));
        assert_eq!(7, color_256to16(244));
        assert_eq!(15, color_256to16(253));
    }
}
