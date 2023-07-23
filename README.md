# Hex Wallet Gen

Hex Wallet Gen is a native Rust library that provides functionality for generating crypto wallets and encrypting them into Ethereum keystore format. It offers support of a various blockchain platforms such as Ethereum, Solana, Aptos, Sui, Bitcoin, Litecoin, and Tron.

## [dependencies]
- `aes` and `ctr`: These dependencies provide the necessary cipher algorithms for encrypting wallets into Ethereum keystore format.

- `blake2`: This dependency is used for Sui address generation.

- `bs58`: This dependency enables address generation for Solana, Bitcoin, Litecoin, and supports Base58Check encoding.

- `ed25519-dalek`: This library offers key derivation algorithms for Solana, Aptos, and Sui.

- `hex`: This crate provides utilities for hexadecimal encoding and decoding, essential for Ethereum keystore and address handling.

- `rand`: This dependency utilizes OsRng to generate secure random ECDSA keypairs.

- `scrypt`: It provides the Key Derivation Function (KDF) used for Ethereum keystore encryption.

- `secp256k1`: This dependency supports key derivation algorithms for Ethereum, Bitcoin, Litecoin, and Tron.

- `serde` and `serde_json`: These crates facilitate the serialization and deserialization of JSON responses.

- `sha3`: This dependency offers the Keccak256 and Sha3_256 hash functions necessary for Aptos and Ethereum.

## [setup]

- Update the `KEYCHAIN_PASSWORD` in the [keychain.rs](https://github.com/Sssilencee/hex-wallet-gen/blob/main/src/types/keychain.rs) file with your own password.
```Rust
const KEYCHAIN_PASSWORD: &str = "STRONG_PASSWORD";
```

- Build `cdylib` file
```console
cargo build --release
```


## [examples]

Here's a simple example of a library using with CGO:

### [[main.go](https://github.com/Sssilencee/hex-wallet-gen/blob/main/service/main.go)]

```Go
package main

/*
#cgo LDFLAGS: -L ../target/debug -l wallet_gen
#include "../libc/binding.h"
*/
import "C"
import "fmt"

func main() {
	c_ptr := C.generate_keychain(C.Btc)

    // C.GoString( ... ) creates a copy of the original 
    // string, so we need to free the original
	defer C.free_c_char(c_ptr)

	go_str := C.GoString(c_ptr)
	fmt.Println(go_str)
	// ..
}
```

### [[binding.h](https://github.com/Sssilencee/hex-wallet-gen/blob/main/libc/binding.h)]

```C
typedef enum {
    Eth,
    Trx,
    Btc,
    Ltc,
    Sol,
    Apt,
    Sui,
} Network;

extern char* generate_keychain(Network);

extern void free_c_char(char*);
```