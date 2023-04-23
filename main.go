package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: ./lib/libhello.a -ldl
#include "hello.h"
#include <stdlib.h>
*/
import "C"
import "unsafe"

//export EthSayGo
func EthSayGo() {
	println("rust will call go 1 ")
}

//export EthSayGoHello
func EthSayGoHello() *C.char {
	return C.CString("Hello from Go! 1 ")
}

func main() {
	var name = C.CString("world 1 ")
	defer C.free(unsafe.Pointer(name))
	var str = C.hello(name)
	defer C.free(unsafe.Pointer(str))
	println("go call rust result :", C.GoString(str))
}
