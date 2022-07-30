package colette

//#cgo CFLAGS: -I${SRCDIR}/include
//#include "colette/colette.h"
import "C"

import "unsafe"

func RgbTo256(r rune, g rune, b rune) int {
	return int(C.colette_rgbto256(C.uchar(r), C.uchar(g), C.uchar(b)))
}

func JoinRgb(r rune, g rune, b rune) int {
	return int(C.colette_join_rgb(C.uchar(r), C.uchar(g), C.uchar(b)))
}

func SplitRgb(c int, r *rune, g *rune, b *rune) {
	C.colette_split_rgb(
		C.int(c),
		(*C.uchar)(unsafe.Pointer(r)),
		(*C.uchar)(unsafe.Pointer(g)),
		(*C.uchar)(unsafe.Pointer(b)),
	)
}

func Color256ToRgb(c int) int {
	return int(C.colette_256torgb(C.int(c)))
}

func Color256To16(c int) int {
	return int(C.colette_256to16(C.int(c)))
}
