package main

/*
#cgo LDFLAGS: -L ../target/debug -l wallet_gen
#include "../libc/binding.h"
*/
import "C"
import "fmt"

func main() {
	fmt.Printf("%s\n", C.GoString(C.generate_keychain(C.Eth)))
	fmt.Printf("%s\n", C.GoString(C.generate_keychain(C.Trx)))
	fmt.Printf("%s\n", C.GoString(C.generate_keychain(C.Btc)))
	fmt.Printf("%s\n", C.GoString(C.generate_keychain(C.Ltc)))
	fmt.Printf("%s\n", C.GoString(C.generate_keychain(C.Sol)))
	fmt.Printf("%s\n", C.GoString(C.generate_keychain(C.Apt)))
	fmt.Printf("%s\n", C.GoString(C.generate_keychain(C.Sui)))
}
