package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L./lib -lhello
#include "./hello/hello.h"
#include <stdlib.h>
*/
import "C"
import "unsafe"

func main() {
	var name = C.CString("world")
	defer C.free(unsafe.Pointer(name))
	var str = C.hello(name)
	defer C.free(unsafe.Pointer(str))
	println("go say :", C.GoString(str))
}
