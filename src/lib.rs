mod types;

use ed25519_dalek::PublicKey as Ed25519PublicKey;
use secp256k1::PublicKey as Secp256k1PublicKey;
use types::{
    keypair::ecdsa_keypair::{EcdsaKeypair, Keypair},
    keychain::{Keychain},
    network::Network,
};
use std::ffi::{c_char, CString};


#[no_mangle]
pub extern "C" fn generate_keychain(network: Network) -> *mut c_char {
    let keychain = match network {
        Network::Evm |
        Network::Trx | 
        Network::Btc |
        Network::Ltc => {
            let mut kp = Keypair::Secp256k1(EcdsaKeypair::new_secp256k1());
            Keychain::new_from_keypair::<32, Secp256k1PublicKey>(&mut kp, network).unwrap()
        },

        Network::Apt | 
        Network::Sol |
        Network::Sui => {
            let mut kp = Keypair::Ed25519(EcdsaKeypair::new_ed25519());
            Keychain::new_from_keypair::<64, Ed25519PublicKey>(&mut kp, network).unwrap()
        }
    };

    CString::new(
        serde_json::to_string(&keychain).unwrap()
    )
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn free_c_char(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    let _ = CString::from_raw(ptr);
}