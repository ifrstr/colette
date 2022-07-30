package colette

//#cgo CFLAGS: -I${SRCDIR}/include/colette
//#include "colette.h"
import "C"

func RgbTo256(r rune, g rune, b rune) int {
	return int(C.colette_rgbto256(C.uchar(r), C.uchar(g), C.uchar(b)))
}

func JoinRgb(r rune, g rune, b rune) int {
	return int(C.colette_join_rgb(C.uchar(r), C.uchar(g), C.uchar(b)))
}

func SplitRgb(c int, r *rune, g *rune, b *rune) {
	C.colette_split_rgb(C.int(c), C.uchar(r), C.uchar(g), C.uchar(b))
}

func Color256ToRgb(c int) int {
	return int(C.colette_256torgb(C.int(c)))
}

func Color256To16(c int) int {
	return int(C.colette_256to16(C.int(c)))
}
