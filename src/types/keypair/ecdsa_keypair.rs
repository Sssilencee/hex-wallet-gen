use ed25519_dalek::{
    PublicKey as Ed25519PublicKey
};
use secp256k1::{
    PublicKey as Secp256k1PublicKey,
};
use sha3::{
    Digest, 
    Keccak256, 
};


pub enum Keypair {
    Secp256k1(EcdsaKeypair<32, Secp256k1PublicKey>),
    Ed25519(EcdsaKeypair<64, Ed25519PublicKey>)
}

pub struct EcdsaKeypair <const N: usize, T> {
    pub secret_key: [u8; N],
    pub public_key: T,
}

impl<const N: usize, T> EcdsaKeypair<N, T> {
    pub fn keccak256_hash<A>(&self, data: A) -> Vec<u8>
        where A: AsRef<[u8]>
    {
        let mut hasher = Keccak256::new();
        hasher.update(data);
        Vec::from(hasher.finalize().as_slice())
    }
}