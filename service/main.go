package main

/*
#cgo LDFLAGS: -L ../target/debug -l wallet_gen
#include "../libc/binding.h"
*/
import "C"
import (
	"fmt"
)

func main() {
	c_ptr := C.generate_keychain(C.Btc)

	// C.GoString( ... ) creates a copy of the original
	// string, so we need to free the original
	defer C.free_c_char(c_ptr)

	go_str := C.GoString(c_ptr)
	fmt.Println(go_str)
}
