package colette

//#cgo CFLAGS: -I${SRCDIR}/include/colette
//#include "colette.h"
import "C"

func RgbTo256(r rune, g rune, b rune) int {
	return C.colette_rgbto256(r, g, b)
}

func JoinRgb(r rune, g rune, b rune) int {
	return C.colette_join_rgb(r, g, b)
}

func SplitRgb(c int, r *rune, g *rune, b *rune) {
	C.colette_split_rgb(c, r, g, b)
}

func Color256ToRgb(c int) int {
	return C.colette_256torgb(c)
}

func Color256To16(c int) int {
	return C.colette_256to16(c)
}
