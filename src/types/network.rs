use serde::Serialize;
use std::{
    fmt::{
        Display as ErrorDisplay,
        Formatter, 
        Result as ErrorResult,
    }
};


#[repr(C)]
#[derive(Debug, Serialize)]
pub enum Network {
    Evm,
    Trx,
    Btc,
    Ltc,
    Sol,
    Apt,
    Sui,
}

#[derive(Debug, Serialize)]
pub struct NetworkMismatchError<'a> {
    pub network: Network,
    pub keypair: &'a str,
}

impl<'a> ErrorDisplay for NetworkMismatchError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> ErrorResult {
        write!(f, "Network mismatch: {}", serde_json::to_string(self).unwrap())
    }
}
