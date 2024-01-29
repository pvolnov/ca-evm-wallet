use std::convert::TryInto;

use sha3::{Digest, Keccak256};
use rlp::{RlpStream, Encodable};



pub struct EthAddress([u8; 20]);

impl EthAddress {
    pub fn from(hex_address: &str) -> Result<Self, &'static str> {
        if hex_address.len() != 40 {
            return Err("Invalid length for Ethereum address");
        }

        let decoded = hex::decode(hex_address).map_err(|_| "Invalid Hex")?;
        let address = decoded.try_into().unwrap();
        Ok(EthAddress(address))
    }
}

impl Encodable for EthAddress {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.encoder().encode_value(&self.0);
    }
}

pub struct EthereumTransaction {
    nonce: u64,
    gas_price: u64,
    gas_limit: u64,
    to: EthAddress,
    value: u64,
    data: Vec<u8>,
}

impl EthereumTransaction {
    pub fn new(nonce: u64, gas_price: u64, gas_limit: u64, to: EthAddress, value: u64, data: Vec<u8>) -> Self {
        EthereumTransaction {
            nonce,
            gas_price,
            gas_limit,
            to,
            value,
            data,
        }
    }
    

    pub fn rlp_encode(&self) -> Vec<u8> {
        let mut stream = RlpStream::new();
        stream.begin_list(6);
        stream.append(&self.nonce);
        stream.append(&self.gas_price);
        stream.append(&self.gas_limit);
        stream.append(&self.to);
        stream.append(&self.value);
        stream.append(&self.data);
        stream.out()
    }

    pub fn calculate_hash(&self) -> Vec<u8> {
        let encoded = self.rlp_encode();
        Keccak256::digest(&encoded).to_vec()
    }
}