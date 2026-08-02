pub mod verify_hash;
pub mod core;
pub mod memory_pool;

use sha2::Sha512;
use hmac::{Hmac};

type HmacSha256 = Hmac<Sha512>;


