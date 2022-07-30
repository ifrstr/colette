#include <gtest/gtest.h>

extern "C" {
#include "colette/colette.h"
}

TEST(Colette, RgbTo256) {
  EXPECT_EQ(16, colette_rgbto256(0, 0, 0));        // Black (0 / 16)
  EXPECT_EQ(231, colette_rgbto256(255, 255, 255)); // White (15 / 231)
  EXPECT_EQ(196, colette_rgbto256(255, 0, 0));     // Red (196)
  EXPECT_EQ(46, colette_rgbto256(0, 255, 0));      // Green (46)
  EXPECT_EQ(21, colette_rgbto256(0, 0, 255));      // Blue (21)
  EXPECT_EQ(251, colette_rgbto256(200, 200, 200)); // Grey (232 - 255)
  EXPECT_EQ(251, colette_rgbto256(201, 201, 201)); // Fuzzy match
}

TEST(Colette, JoinRgb) {
  EXPECT_EQ(0x000000, colette_join_rgb(0, 0, 0));
  EXPECT_EQ(0xffffff, colette_join_rgb(255, 255, 255));
  EXPECT_EQ(0xff0000, colette_join_rgb(255, 0, 0));
  EXPECT_EQ(0x00ff00, colette_join_rgb(0, 255, 0));
  EXPECT_EQ(0x0000ff, colette_join_rgb(0, 0, 255));
  EXPECT_EQ(0xc8c8c8, colette_join_rgb(200, 200, 200));
}

TEST(Colette, SplitRgb) {
  unsigned char r, g, b;
  colette_split_rgb(0xc8c8c8, &r, &g, &b);
  EXPECT_EQ(200, r);
  EXPECT_EQ(200, g);
  EXPECT_EQ(200, b);
}

TEST(Colette, 256ToRgb) {
  EXPECT_EQ(colette_join_rgb(135, 255, 135), colette_256torgb(120));
  EXPECT_EQ(colette_join_rgb(215, 255, 0), colette_256torgb(190));
}

TEST(Colette, 256To16) {
  EXPECT_EQ(12, colette_256to16(21));
  EXPECT_EQ(1, colette_256to16(52));
  EXPECT_EQ(9, colette_256to16(196));
  EXPECT_EQ(7, colette_256to16(244));
  EXPECT_EQ(15, colette_256to16(253));
}
