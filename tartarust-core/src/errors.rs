use hmac::digest::InvalidLength;

pub enum TartarusError {
    MacCreationError(hmac::digest::InvalidLength),
}

impl From<InvalidLength> for TartarusError {
    fn from(value: InvalidLength) -> Self {
        Self::MacCreationError(value)
    }
}