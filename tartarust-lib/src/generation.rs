use rand::{RngCore, rngs::OsRng};
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Zeroize, ZeroizeOnDrop, Clone, Serialize, Deserialize)]
pub struct Salt([u8; 32]);

impl Salt {
    pub fn generate() -> Self {
        let mut salt = [0u8; 32];
        OsRng.fill_bytes(&mut salt);
        Salt(salt)
    }
}

impl AsRef<[u8]> for Salt {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}


#[derive(Zeroize, ZeroizeOnDrop, Clone)]
pub struct Pepper(Vec<u8>);

impl Pepper {
    pub fn generate() -> Self {
        let mut pepper = [0u8; 32];
        OsRng.fill_bytes(&mut pepper);
        Pepper(pepper.to_vec())
    }

    pub fn generate_with_len(len: usize) -> Self {
        let mut pepper = vec![0u8; len];
        OsRng.fill_bytes(&mut pepper);
        Pepper(pepper)
    }
}

impl AsRef<[u8]> for Pepper {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<String> for Pepper {
    fn from(value: String) -> Self {
        Self(value.into_bytes().to_vec())
    }
}

impl From<&str> for Pepper {
    fn from(value: &str) -> Self {
        Self(value.as_bytes().to_vec())
    }
}

impl From<Vec<u8>> for Pepper {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl From<&[u8]> for Pepper {
    fn from(value: &[u8]) -> Self {
        Self(value.to_vec())
    }
}