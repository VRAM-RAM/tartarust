use hmac::digest::InvalidLength;

#[derive(Debug)]
pub enum TartarusError {
    MacCreationError(hmac::digest::InvalidLength),
    HexDecodingError(hex::FromHexError),
    SaltLengthIsWrong,
}

impl From<InvalidLength> for TartarusError {
    fn from(value: InvalidLength) -> Self {
        Self::MacCreationError(value)
    }
}

impl From<hex::FromHexError> for TartarusError {
    fn from(value: hex::FromHexError) -> Self {
        Self::HexDecodingError(value)
    }
}

impl std::fmt::Display for TartarusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TartarusError::MacCreationError(e) => format!("Mac creation error : {:?}", e),
            TartarusError::HexDecodingError(e) => format!("Hex decoding error : {:?}", e),
            TartarusError::SaltLengthIsWrong => "Salt length is wrong. It should be 32 when encoded in hex, and 16 when encoded in bytes".to_string(),
        };

        write!(f, "`{str}`")
    }
}