pub mod verify_hash;
pub mod core;
pub mod memory_pool;

use sha2::Sha512;
use hmac::{Hmac};

type HmacSha256 = Hmac<Sha512>;

pub struct Hash {
    data: Vec<u8>,
}

impl Hash {
    pub fn new(data: &[u8]) -> Self {
        Self { data: data.to_vec() }
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.data.clone())
    }
}
