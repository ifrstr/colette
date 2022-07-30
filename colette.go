// Convert between truecolor, xterm(1) 256 color and 16 color.
// Single-header library.
// Arranged from [tmux](https://github.com/tmux/tmux).
package colette

//#cgo CFLAGS: -I${SRCDIR}/include
//#include "colette/colette.h"
import "C"

import "unsafe"

// RgbTo256 converts an RGB triplet to the xterm(1) 256 color palette.
//
// xterm provides a 6x6x6 color cube (16 - 231) and 24 greys (232 - 255). We
// map our RGB color to the closest in the cube, also work out the closest
// grey, and use the nearest of the two.
//
// Note that the xterm has much lower resolution for darker colors (they are
// not evenly spread out), so our 6 levels are not evenly spread: 0x0, 0x5f
// (95), 0x87 (135), 0xaf (175), 0xd7 (215) and 0xff (255). Greys are more
// evenly spread (8, 18, 28 ... 238).
func RgbTo256(r byte, g byte, b byte) int {
	return int(C.colette_rgbto256(C.uchar(r), C.uchar(g), C.uchar(b)))
}

// JoinRgb joins RGB into a color.
func JoinRgb(r byte, g byte, b byte) int {
	return int(C.colette_join_rgb(C.uchar(r), C.uchar(g), C.uchar(b)))
}

// SplitRgb converts 256 color to RGB color.
func SplitRgb(c int, r *byte, g *byte, b *byte) {
	C.colette_split_rgb(
		C.int(c),
		(*C.uchar)(unsafe.Pointer(r)),
		(*C.uchar)(unsafe.Pointer(g)),
		(*C.uchar)(unsafe.Pointer(b)),
	)
}

// Color256ToRgb converts 256 color to RGB color.
func Color256ToRgb(c int) int {
	return int(C.colette_256torgb(C.int(c)))
}

// Color256To16 converts 256 color to 16 color.
func Color256To16(c int) int {
	return int(C.colette_256to16(C.int(c)))
}
