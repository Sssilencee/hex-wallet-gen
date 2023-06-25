use aes::cipher::{
    KeyIvInit, 
    StreamCipher
};
use rand::Rng;
use scrypt::{
    password_hash::{
        rand_core::OsRng,
        SaltString,
    },
    Params,
};
use secp256k1::hashes::{
    sha256::Hash as Sha256Hash, 
    Hash
};
use serde::Serialize;
use sha3::{Digest, Keccak256};


const SCRYPT_DK_LEN: usize = 64;
const SCRYPT_N: u8 = 13;
const SCRYPT_R: u32 = 8;
const SCRYPT_P: u32 = 8;

#[derive(Serialize)]
pub struct CipherParams {
    pub iv: String,

    #[serde(skip_serializing)]
    pub iv_bytes: [u8; 16]
}

impl CipherParams {
    pub fn new() -> Self {
        let iv_bytes: [u8; 16] = rand::thread_rng().gen();
        Self {
            iv: hex::encode(iv_bytes),
            iv_bytes: iv_bytes
        }
    }
}

#[derive(Serialize)]
pub struct KdfParams {
    pub dklen: usize,
    pub n: u8,
    pub r: u32,
    pub p: u32,
    pub salt: String,

    #[serde(skip_serializing)]
    pub salt_bytes: [u8; 16]
}

impl KdfParams {

    pub fn default() -> Self {

        let salt = SaltString::generate(&mut OsRng);
        let mut salt_bytes = [0u8; 16];
        salt.decode_b64(&mut salt_bytes).unwrap();

        Self { 
            dklen: SCRYPT_DK_LEN, 
            n: SCRYPT_N, 
            r: SCRYPT_R, 
            p: SCRYPT_P, 
            salt: hex::encode(salt.as_str()),
            salt_bytes: salt_bytes
        }
    }

    pub fn as_scrypt_params(&self) -> Params {
        Params::new(
            self.n, 
            self.r, 
            self.p, 
            self.dklen
        ).unwrap()
    }
}

#[derive(Serialize)]
pub struct Crypto <'a> {
    pub cipher: &'a str,
    pub cipherparams: CipherParams,
    pub ciphertext: String,
    pub kdf: &'a str,
    pub kdfparams: KdfParams,
    pub mac: String
}

type Aes128Ctr64LE = ctr::Ctr64LE<aes::Aes128>;

impl <'a> Crypto <'a> {
    pub fn new_aes_128_ctr(
        secret_key: &mut [u8],
        password: String
    ) -> Self {
        let kdfparams = KdfParams::default();

        // Hashes password bytes with a Scrypt function
        let mut password_hash = [0u8; 16];
        scrypt::scrypt(
            password.as_bytes(), 
            &kdfparams.salt_bytes, 
            &kdfparams.as_scrypt_params(), 
            &mut password_hash
        ).unwrap();

        // Encrypts private key with an AES-158-CTR and initial vector
        let cipherparams = CipherParams::new();
        let mut cipher = Aes128Ctr64LE::new(
            &password_hash.into(), 
            &cipherparams.iv_bytes.into()
        );
        cipher.apply_keystream(secret_key);

        // Hashes message authentication code with a Keccak256 function 
        // of the second-leftmost 8 bytes of the derived key together
        // with the full ciphertext
        let mut hasher = Keccak256::new();
        hasher.update([
            &password_hash[8..], 
            secret_key
        ].concat());

        Self { 
            cipher: "aes-128-ctr", 
            cipherparams: cipherparams, 
            ciphertext: hex::encode(secret_key), 
            kdf: "scrypt", 
            kdfparams: kdfparams, 
            mac: hex::encode(hasher.finalize())
        }
    }

    pub fn b58_check(mut bytes: Vec<u8>) -> String {
        let chk = &Sha256Hash::hash(
            &Sha256Hash::hash(&bytes)
                .to_byte_array()
        )[..4];
        bytes.extend(chk.iter());
        bs58::encode(bytes).into_string()
    }
}