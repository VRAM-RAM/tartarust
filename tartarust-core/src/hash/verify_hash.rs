use subtle::ConstantTimeEq;


pub fn verify_hash(stored_hash: &[u8], computed_hash: &[u8]) -> bool {
    if stored_hash.len() != 64 || computed_hash.len() != 64 {
        return false
    }
    return stored_hash.ct_eq(&computed_hash).into();
}