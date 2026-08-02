use crate::{generation::Salt, params::TartarusParams};
use tartarust_core::{errors::TartarusError, hash::{core::tartarus, verify_hash}};
use serde::{Deserialize, Serialize};
use hex::{encode};

#[derive(Serialize, Deserialize, Debug)]
pub struct Hash {
    digest: Vec<u8>,
    salt: Salt,
}

impl Hash {
    pub fn new(digest: &[u8], salt: Salt) -> Self {
        Self { digest: digest.to_vec(), salt: salt }
    }

    pub fn digest(&self) -> &[u8] {
        &self.digest
    }

    pub fn salt(&self) -> &Salt {
        &self.salt
    }

    pub fn verify(&self, data: &[u8], parameters: &TartarusParams) -> Result<bool, TartarusError> {
        let digest = tartarus(data, self.salt.as_ref(), parameters.pepper.as_ref(), parameters.memory, parameters.iterations)?;
        Ok(verify_hash::verify_hash(&self.digest, &digest))
    }

    pub fn encode_hex(&self) -> (String, String) {
        let hex_digest = encode(&self.digest);
        let hex_salt = encode(&self.salt);

        (hex_digest, hex_salt)
    }
}


pub fn hash(data: &[u8], parameters: &TartarusParams) -> Result<Hash, TartarusError> {
    let salt = Salt::generate();
    let digest = tartarus(data, salt.as_ref(), parameters.pepper.as_ref(), parameters.memory, parameters.iterations)?;
    Ok(Hash::new(&digest, salt))
}

pub fn hash_with_custom_salt(data: &[u8], salt: Salt, parameters: &TartarusParams) -> Result<Hash, TartarusError> {
    let digest = tartarus(data, salt.as_ref(), parameters.pepper.as_ref(), parameters.memory, parameters.iterations)?;
    Ok(Hash::new(&digest, salt))
}