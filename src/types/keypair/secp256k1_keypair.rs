use secp256k1::{
    hashes::{
        Hash, 
        ripemd160::Hash as Ridemp160Hash,
        sha256::Hash as Sha256Hash, 
    },
    PublicKey as Secp256k1PublicKey,
    rand::rngs::OsRng as SecpOsRng,
    Secp256k1, 
};
use std::ops::Deref;

use crate::types::crypto::Crypto;
use super::ecdsa_keypair::EcdsaKeypair;


impl EcdsaKeypair<32, Secp256k1PublicKey> {
    pub fn new_secp256k1() -> Self {
        let secp = Secp256k1::new();
        let (sk, pk) = secp.generate_keypair(&mut SecpOsRng);
        Self {
            secret_key: sk.secret_bytes(),
            public_key: pk,
        }
    }

    pub fn get_evm_address(&self) -> String {
        "0x".to_owned() + hex::encode(
            &self.keccak256_hash(
                self.public_key.serialize_uncompressed()
            )[12..]
        ).deref()
    }

    pub fn get_trx_address(&self) -> String {
        Crypto::b58_check([

            // Tron mainnet address prefix
            &[0x41], 
            &self.keccak256_hash(
                self.public_key.serialize_uncompressed()
            )[12..]
        ].concat())
    }

    pub fn get_btc_based_address(&self, version_prefix: u8) -> String {
        let pk_sha_256 = Sha256Hash::hash(
            &self.public_key.serialize()
        ).to_byte_array();

        let mut pk_ridemp_160 = Ridemp160Hash::hash(&pk_sha_256)
            .to_byte_array()
            .to_vec();

        // Bitcoin mainnet address prefix
        pk_ridemp_160.insert(0, version_prefix);

        Crypto::b58_check(pk_ridemp_160)
    }
}