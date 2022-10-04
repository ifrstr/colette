package colette

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestRgbTo256(t *testing.T) {
	assert.Equal(t, 16, RgbTo256(0, 0, 0), "Black (0 / 16)")
	assert.Equal(t, 231, RgbTo256(255, 255, 255), "White (15 / 231)")
	assert.Equal(t, 196, RgbTo256(255, 0, 0), "Red (196)")
	assert.Equal(t, 46, RgbTo256(0, 255, 0), "Green (46)")
	assert.Equal(t, 21, RgbTo256(0, 0, 255), "Blue (21)")
	assert.Equal(t, 251, RgbTo256(200, 200, 200), "Grey (232 - 255)")
	assert.Equal(t, 251, RgbTo256(201, 201, 201), "Fuzzy match")
	assert.Equal(t, 188, RgbTo256(215, 215, 215), "Exact match")
}

func TestJoinRgb(t *testing.T) {
	assert.Equal(t, 0x000000, JoinRgb(0, 0, 0), "Black")
	assert.Equal(t, 0xffffff, JoinRgb(255, 255, 255), "White")
	assert.Equal(t, 0xff0000, JoinRgb(255, 0, 0), "Red")
	assert.Equal(t, 0x00ff00, JoinRgb(0, 255, 0), "Green")
	assert.Equal(t, 0x0000ff, JoinRgb(0, 0, 255), "Blue")
	assert.Equal(t, 0xc8c8c8, JoinRgb(200, 200, 200), "Grey")
}

func TestSplitRgb(t *testing.T) {
	r, g, b := SplitRgb(0xc8c8c8)
	assert.Equal(t, byte(200), r, "R")
	assert.Equal(t, byte(200), g, "G")
	assert.Equal(t, byte(200), b, "B")
}

func TestColor256ToRgb(t *testing.T) {
	assert.Equal(t, JoinRgb(135, 255, 135), Color256ToRgb(120), "Custom Color 1")
	assert.Equal(t, JoinRgb(215, 255, 0), Color256ToRgb(190), "Custom Color 2")
}

func TestColor256To16(t *testing.T) {
	assert.Equal(t, byte(12), Color256To16(21), "Pure Blue")
	assert.Equal(t, byte(1), Color256To16(52), "Deep Red")
	assert.Equal(t, byte(9), Color256To16(196), "Pure Red")
	assert.Equal(t, byte(7), Color256To16(244), "Normal Grey")
	assert.Equal(t, byte(15), Color256To16(253), "Light Grey")
}
