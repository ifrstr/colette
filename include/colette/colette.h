#ifndef _COLETTE_H_
#define _COLETTE_H_

static int colette_dist_sq(int R, int G, int B, int r, int g, int b) {
  return (R - r) * (R - r) + (G - g) * (G - g) + (B - b) * (B - b);
}

static int colette_color_to_6cube(int v) {
  return v < 48 ? 0 : v < 114 ? 1 : (v - 35) / 40;
}

/*
 * Convert an RGB triplet to the xterm(1) 256 color palette.
 *
 * xterm provides a 6x6x6 color cube (16 - 231) and 24 greys (232 - 255). We
 * map our RGB color to the closest in the cube, also work out the closest
 * grey, and use the nearest of the two.
 *
 * Note that the xterm has much lower resolution for darker colors (they are
 * not evenly spread out), so our 6 levels are not evenly spread: 0x0, 0x5f
 * (95), 0x87 (135), 0xaf (175), 0xd7 (215) and 0xff (255). Greys are more
 * evenly spread (8, 18, 28 ... 238).
 */
int colette_rgbto256(unsigned char r, unsigned char g, unsigned char b) {
  static const int q2c[6] = {0x00, 0x5f, 0x87, 0xaf, 0xd7, 0xff};
  int qr, qg, qb, cr, cg, cb, idx, grey_avg, grey_idx, grey;

  /* Map RGB to 6x6x6 cube. */
  qr = colette_color_to_6cube(r);
  cr = q2c[qr];
  qg = colette_color_to_6cube(g);
  cg = q2c[qg];
  qb = colette_color_to_6cube(b);
  cb = q2c[qb];

  /* If we have hit the color exactly, return early. */
  if (cr == r && cg == g && cb == b)
    return 16 + 36 * qr + 6 * qg + qb;

  /* Work out the closest grey (average of RGB). */
  grey_avg = (r + g + b) / 3;
  if (grey_avg > 238)
    grey_idx = 23;
  else
    grey_idx = (grey_avg - 3) / 10;
  grey = 8 + (10 * grey_idx);

  /* Is grey or 6x6x6 color closest? */
  idx = colette_dist_sq(grey, grey, grey, r, g, b) <
                colette_dist_sq(cr, cg, cb, r, g, b)
            ? 232 + grey_idx
            : 16 + 36 * qr + 6 * qg + qb;
  return idx;
}

/* Join RGB into a color. */
int colette_join_rgb(unsigned char r, unsigned char g, unsigned char b) {
  return (int)r << 16 | (int)g << 8 | (int)b;
}

/* Split color into RGB. */
void colette_split_rgb(int c, unsigned char *r, unsigned char *g,
                     unsigned char *b) {
  *r = c >> 16 & 0xff;
  *g = c >> 8 & 0xff;
  *b = c & 0xff;
}

/* Convert 256 color to RGB color. */
int colette_256torgb(unsigned char c) {
  static const int table[256] = {
      0x000000, 0x800000, 0x008000, 0x808000,
      0x000080, 0x800080, 0x008080, 0xc0c0c0,
      0x808080, 0xff0000, 0x00ff00, 0xffff00,
      0x0000ff, 0xff00ff, 0x00ffff, 0xffffff,
      0x000000, 0x00005f, 0x000087, 0x0000af,
      0x0000d7, 0x0000ff, 0x005f00, 0x005f5f,
      0x005f87, 0x005faf, 0x005fd7, 0x005fff,
      0x008700, 0x00875f, 0x008787, 0x0087af,
      0x0087d7, 0x0087ff, 0x00af00, 0x00af5f,
      0x00af87, 0x00afaf, 0x00afd7, 0x00afff,
      0x00d700, 0x00d75f, 0x00d787, 0x00d7af,
      0x00d7d7, 0x00d7ff, 0x00ff00, 0x00ff5f,
      0x00ff87, 0x00ffaf, 0x00ffd7, 0x00ffff,
      0x5f0000, 0x5f005f, 0x5f0087, 0x5f00af,
      0x5f00d7, 0x5f00ff, 0x5f5f00, 0x5f5f5f,
      0x5f5f87, 0x5f5faf, 0x5f5fd7, 0x5f5fff,
      0x5f8700, 0x5f875f, 0x5f8787, 0x5f87af,
      0x5f87d7, 0x5f87ff, 0x5faf00, 0x5faf5f,
      0x5faf87, 0x5fafaf, 0x5fafd7, 0x5fafff,
      0x5fd700, 0x5fd75f, 0x5fd787, 0x5fd7af,
      0x5fd7d7, 0x5fd7ff, 0x5fff00, 0x5fff5f,
      0x5fff87, 0x5fffaf, 0x5fffd7, 0x5fffff,
      0x870000, 0x87005f, 0x870087, 0x8700af,
      0x8700d7, 0x8700ff, 0x875f00, 0x875f5f,
      0x875f87, 0x875faf, 0x875fd7, 0x875fff,
      0x878700, 0x87875f, 0x878787, 0x8787af,
      0x8787d7, 0x8787ff, 0x87af00, 0x87af5f,
      0x87af87, 0x87afaf, 0x87afd7, 0x87afff,
      0x87d700, 0x87d75f, 0x87d787, 0x87d7af,
      0x87d7d7, 0x87d7ff, 0x87ff00, 0x87ff5f,
      0x87ff87, 0x87ffaf, 0x87ffd7, 0x87ffff,
      0xaf0000, 0xaf005f, 0xaf0087, 0xaf00af,
      0xaf00d7, 0xaf00ff, 0xaf5f00, 0xaf5f5f,
      0xaf5f87, 0xaf5faf, 0xaf5fd7, 0xaf5fff,
      0xaf8700, 0xaf875f, 0xaf8787, 0xaf87af,
      0xaf87d7, 0xaf87ff, 0xafaf00, 0xafaf5f,
      0xafaf87, 0xafafaf, 0xafafd7, 0xafafff,
      0xafd700, 0xafd75f, 0xafd787, 0xafd7af,
      0xafd7d7, 0xafd7ff, 0xafff00, 0xafff5f,
      0xafff87, 0xafffaf, 0xafffd7, 0xafffff,
      0xd70000, 0xd7005f, 0xd70087, 0xd700af,
      0xd700d7, 0xd700ff, 0xd75f00, 0xd75f5f,
      0xd75f87, 0xd75faf, 0xd75fd7, 0xd75fff,
      0xd78700, 0xd7875f, 0xd78787, 0xd787af,
      0xd787d7, 0xd787ff, 0xd7af00, 0xd7af5f,
      0xd7af87, 0xd7afaf, 0xd7afd7, 0xd7afff,
      0xd7d700, 0xd7d75f, 0xd7d787, 0xd7d7af,
      0xd7d7d7, 0xd7d7ff, 0xd7ff00, 0xd7ff5f,
      0xd7ff87, 0xd7ffaf, 0xd7ffd7, 0xd7ffff,
      0xff0000, 0xff005f, 0xff0087, 0xff00af,
      0xff00d7, 0xff00ff, 0xff5f00, 0xff5f5f,
      0xff5f87, 0xff5faf, 0xff5fd7, 0xff5fff,
      0xff8700, 0xff875f, 0xff8787, 0xff87af,
      0xff87d7, 0xff87ff, 0xffaf00, 0xffaf5f,
      0xffaf87, 0xffafaf, 0xffafd7, 0xffafff,
      0xffd700, 0xffd75f, 0xffd787, 0xffd7af,
      0xffd7d7, 0xffd7ff, 0xffff00, 0xffff5f,
      0xffff87, 0xffffaf, 0xffffd7, 0xffffff,
      0x080808, 0x121212, 0x1c1c1c, 0x262626,
      0x303030, 0x3a3a3a, 0x444444, 0x4e4e4e,
      0x585858, 0x626262, 0x6c6c6c, 0x767676,
      0x808080, 0x8a8a8a, 0x949494, 0x9e9e9e,
      0xa8a8a8, 0xb2b2b2, 0xbcbcbc, 0xc6c6c6,
      0xd0d0d0, 0xdadada, 0xe4e4e4, 0xeeeeee};

  return table[c];
}

/* Convert 256 color to 16 color. */
int colette_256to16(unsigned char c) {
  static const char table[256] = {
       0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15,
       0,  4,  4,  4, 12, 12,  2,  6,  4,  4, 12, 12,  2,  2,  6,  4,
      12, 12,  2,  2,  2,  6, 12, 12, 10, 10, 10, 10, 14, 12, 10, 10,
      10, 10, 10, 14,  1,  5,  4,  4, 12, 12,  3,  8,  4,  4, 12, 12,
       2,  2,  6,  4, 12, 12,  2,  2,  2,  6, 12, 12, 10, 10, 10, 10,
      14, 12, 10, 10, 10, 10, 10, 14,  1,  1,  5,  4, 12, 12,  1,  1,
       5,  4, 12, 12,  3,  3,  8,  4, 12, 12,  2,  2,  2,  6, 12, 12,
      10, 10, 10, 10, 14, 12, 10, 10, 10, 10, 10, 14,  1,  1,  1,  5,
      12, 12,  1,  1,  1,  5, 12, 12,  1,  1,  1,  5, 12, 12,  3,  3,
       3,  7, 12, 12, 10, 10, 10, 10, 14, 12, 10, 10, 10, 10, 10, 14,
       9,  9,  9,  9, 13, 12,  9,  9,  9,  9, 13, 12,  9,  9,  9,  9,
      13, 12,  9,  9,  9,  9, 13, 12, 11, 11, 11, 11,  7, 12, 10, 10,
      10, 10, 10, 14,  9,  9,  9,  9,  9, 13,  9,  9,  9,  9,  9, 13,
       9,  9,  9,  9,  9, 13,  9,  9,  9,  9,  9, 13,  9,  9,  9,  9,
       9, 13, 11, 11, 11, 11, 11, 15,  0,  0,  0,  0,  0,  0,  8,  8,
       8,  8,  8,  8,  7,  7,  7,  7,  7,  7, 15, 15, 15, 15, 15, 15};

  return table[c];
}

#endif /* _COLETTE_H_ */
