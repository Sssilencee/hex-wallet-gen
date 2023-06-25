use blake2::{
    Blake2b,
    digest::consts::U32
};
use ed25519_dalek::{
    Keypair as Ed25519Keypair,
    PublicKey as Ed25519PublicKey
};
use rand::rngs::OsRng;
use sha3::{
    Digest, 
    Sha3_256
};

use super::ecdsa_keypair::EcdsaKeypair;

type Blake2b32 = Blake2b<U32>;

impl EcdsaKeypair<64, Ed25519PublicKey> {
    pub fn new_ed25519() -> Self {
        let kp = Ed25519Keypair::generate(&mut OsRng::default());
        Self {
            secret_key: kp.to_bytes(),
            public_key: kp.public,
        }
    }

    pub fn get_sol_address(&self) -> String {
        bs58::encode(self.public_key.to_bytes()).into_string()
    }

    pub fn get_aptos_address(&self) -> String {
        let mut hasher = Sha3_256::new();
        hasher.update([
            self.public_key
                .to_bytes()
                .as_slice(), 
            &[0x00]
        ].concat());
        hex::encode(hasher.finalize())
    }

    pub fn get_sui_address(&self) -> String {
        let mut hasher = Blake2b32::new();
        hasher.update([
            &[0x00], 
            self.public_key
                .to_bytes()
                .as_slice()
        ].concat());
        hex::encode(hasher.finalize())
    }
}