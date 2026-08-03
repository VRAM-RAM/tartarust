use crate::{generation::Salt, params::TartarusParams};
use tartarust_core::{errors::TartarusError, hash::{core::tartarus, verify_hash}};
use serde::{Deserialize, Serialize};
use hex::{encode};

#[derive(Serialize, Deserialize, Debug)]
/// The structure that contains the result of hashing : the digest and the salt.
pub struct Hash {
    digest: Vec<u8>,
    salt: Salt,
}

impl Hash {
    /// Creates a new Hash (basic & idiomatic `new()` method)
    pub fn new(digest: &[u8], salt: Salt) -> Self {
        Self { digest: digest.to_vec(), salt: salt }
    }

    /// Returns a reference of the digest.
    pub fn digest(&self) -> &[u8] {
        &self.digest
    }

    /// Returns a reference of the salt.
    pub fn salt(&self) -> &Salt {
        &self.salt
    }

    /// Computes, given the stored `salt` and the data to verify, the hash of these data, then calls `verify_hash` 
    /// to compare the stored digest and the computed one.
    pub fn verify(&self, data: &[u8], parameters: &TartarusParams) -> Result<bool, TartarusError> {
        let digest = tartarus(data, self.salt.as_ref(), parameters.pepper.as_ref(), parameters.memory, parameters.iterations)?;
        Ok(verify_hash::verify_hash(&self.digest, &digest))
    }

    /// Returns both the digest and the salt (a tuple), encoded in hexadecimal.
    pub fn encode_hex(&self) -> (String, String) {
        let hex_digest = encode(&self.digest);
        let hex_salt = encode(&self.salt);

        (hex_digest, hex_salt)
    }
}

/// Returns the `Hash` of the data (digest + salt). The salt is generated randomly in the function.
pub fn hash(data: &[u8], parameters: &TartarusParams) -> Result<Hash, TartarusError> {
    let salt = Salt::generate();
    let digest = tartarus(data, salt.as_ref(), parameters.pepper.as_ref(), parameters.memory, parameters.iterations)?;
    Ok(Hash::new(&digest, salt))
}

/// Returns the `Hash` of the data (digest + salt). The salt is provided by the user.
pub fn hash_with_custom_salt(data: &[u8], salt: Salt, parameters: &TartarusParams) -> Result<Hash, TartarusError> {
    let digest = tartarus(data, salt.as_ref(), parameters.pepper.as_ref(), parameters.memory, parameters.iterations)?;
    Ok(Hash::new(&digest, salt))
}