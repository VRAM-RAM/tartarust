use rand::{RngCore, rngs::OsRng};
use serde::{Deserialize, Serialize};
use tartarust_core::errors::TartarusError;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// The Salt (an array of 16 u8s)
#[derive(Zeroize, ZeroizeOnDrop, Clone, Serialize, Deserialize, Debug)]
pub struct Salt([u8; 16]);

impl Salt {
    pub fn empty() -> Self {
        Salt([0u8; 16])
    }

    /// Generate a random salt.
    pub fn generate() -> Self {
        let mut salt = [0u8; 16];
        OsRng.fill_bytes(&mut salt);
        Salt(salt)
    }

    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        Salt(bytes)
    }

    /// Encodes the salt into hexadecimal.
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }

    /// Decodes the salt from hexadecimal to `Salt`.
    pub fn from_hex(hex_salt: String) -> Result<Self, TartarusError> {
        let salt = hex::decode(hex_salt)?;
        if salt.len() != 16 {
            return Err(TartarusError::SaltLengthIsWrong);
        }

        Ok(salt.into())
    }
}

impl From<Vec<u8>> for Salt {
    fn from(value: Vec<u8>) -> Self {
        let mut salt = [0u8; 16];
        for index in 0..16 {
            salt[index] = value[index];
        }

        Salt(salt)
    }
}

impl AsRef<[u8]> for Salt {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// The `Pepper` structure.
#[derive(Zeroize, ZeroizeOnDrop, Clone)]
pub struct Pepper(Vec<u8>);

impl Pepper {
    /// Generate a random `Pepper` of size 32.
    pub fn generate() -> Self {
        let mut pepper = [0u8; 32];
        OsRng.fill_bytes(&mut pepper);
        Pepper(pepper.to_vec())
    }

    /// Generate a random `Pepper`, but with a custom length.
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