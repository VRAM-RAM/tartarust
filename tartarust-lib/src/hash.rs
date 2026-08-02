use crate::{generation::Salt, params::TartarusParams};
use tartarust_core::{errors::TartarusError, hash::{core::tartarus, verify_hash}};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Hash {
    digest: Vec<u8>,
    salt: Salt,
}

impl Hash {
    pub fn new(digest: &[u8], salt: Salt) -> Self {
        Self { digest: digest.to_vec(), salt: salt }
    }

    pub fn verify(&self, data: &[u8], parameters: &TartarusParams) -> Result<bool, TartarusError> {
        let digest = tartarus(data, self.salt.as_ref(), parameters.pepper.as_ref(), parameters.memory, parameters.iterations)?;
        Ok(verify_hash::verify_hash(&self.digest, &digest))
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