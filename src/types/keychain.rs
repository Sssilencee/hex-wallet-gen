use serde::Serialize;

use super::{
    crypto::Crypto,
    keypair::ecdsa_keypair::Keypair,
    network::{Network, NetworkMismatchError},
};


#[derive(Serialize)]
pub struct Keychain<'a> {
    pub address: String,
    pub crypto: Crypto<'a>,
}

impl <'a> Keychain<'a> {
    pub fn new_from_keypair<const N: usize, T>(
        keypair: &mut Keypair,
        network: Network,
    ) -> Result<Self, NetworkMismatchError> {
        let (address, secret_key) = match keypair 
        {
            Keypair::Secp256k1(kp) => {(
                match network {
                    Network::Evm => kp.get_evm_address(),
                    Network::Trx => kp.get_trx_address(),
                    Network::Btc => kp.get_btc_based_address(0x00),
                    Network::Ltc => kp.get_btc_based_address(0x30),

                    Network::Apt | 
                    Network::Sol |
                    Network::Sui => 
                        return Err(NetworkMismatchError {
                            network: network,
                            keypair: "Secp256k1"
                        })
                }, 
                kp.secret_key.as_mut()
            )},
            Keypair::Ed25519(kp) => {(
                match network {
                    Network::Sol => kp.get_sol_address(),
                    Network::Apt => kp.get_aptos_address(),
                    Network::Sui => kp.get_sui_address(),

                    Network::Evm |
                    Network::Trx |
                    Network::Btc |
                    Network::Ltc => 
                        return Err(NetworkMismatchError {
                            network: network,
                            keypair: "Ed25519"
                        })
                }, 
                kp.secret_key.as_mut()
            )}
        };

        Ok(Self {
            address: address,
            crypto: Crypto::new_aes_128_ctr(
                secret_key, 
                "PASSWORD".into()
            )
        })
    }
}