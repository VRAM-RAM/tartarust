use tartarust_lib::generation::{Pepper, Salt};
use tartarust_lib::hash::{hash, hash_with_custom_salt};
use tartarust_lib::params::TartarusParams;

fn params(memory: u32, iterations: u32) -> TartarusParams {
    TartarusParams::new(memory, iterations, Pepper::from("test_pepper_32_bytes_0123456789abcdef"))
}

fn salt_from_bytes(bytes: [u8; 32]) -> Salt {
    Salt::from_bytes(bytes)
}

const DATA: &[u8] = b"Correct Horse Battery Staple";

#[test]
fn hash_and_verify_round_trip() {
    let p = params(4, 1);
    let hashed = hash(DATA, &p).unwrap();
    assert!(hashed.verify(DATA, &p).unwrap());
}

#[test]
fn verify_rejects_wrong_password() {
    let p = params(4, 1);
    let hashed = hash(DATA, &p).unwrap();
    assert!(!hashed.verify(b"not the password", &p).unwrap());
}

#[test]
fn verify_rejects_wrong_memory() {
    let p = params(4, 1);
    let hashed = hash(DATA, &p).unwrap();
    assert!(!hashed.verify(DATA, &params(8, 1)).unwrap());
}

#[test]
fn verify_rejects_wrong_iterations() {
    let p = params(4, 1);
    let hashed = hash(DATA, &p).unwrap();
    assert!(!hashed.verify(DATA, &params(4, 2)).unwrap());
}

#[test]
fn hash_uses_random_salts() {
    let p = params(4, 1);
    let first = hash(DATA, &p).unwrap();
    let second = hash(DATA, &p).unwrap();
    assert_ne!(first.digest(), second.digest());
    assert_ne!(first.salt().as_ref(), second.salt().as_ref());
}

#[test]
fn custom_salt_is_deterministic() {
    let p = params(4, 1);
    let fixed_salt = salt_from_bytes([7u8; 32]);
    let first = hash_with_custom_salt(DATA, fixed_salt.clone(), &p).unwrap();
    let second = hash_with_custom_salt(DATA, fixed_salt, &p).unwrap();
    assert_eq!(first.digest(), second.digest());
    assert!(first.verify(DATA, &p).unwrap());
}

#[test]
fn different_custom_salts_yield_different_hashes() {
    let p = params(4, 1);
    let hashed_a = hash_with_custom_salt(DATA, salt_from_bytes([1u8; 32]), &p).unwrap();
    let hashed_b = hash_with_custom_salt(DATA, salt_from_bytes([2u8; 32]), &p).unwrap();
    assert_ne!(hashed_a.digest(), hashed_b.digest());
    assert!(hashed_a.verify(DATA, &p).unwrap());
    assert!(hashed_b.verify(DATA, &p).unwrap());
}

#[test]
fn pepper_conversions() {
    let from_str = Pepper::from("my pepper");
    assert_eq!(from_str.as_ref(), b"my pepper");

    let from_string = Pepper::from(String::from("my pepper"));
    assert_eq!(from_string.as_ref(), b"my pepper");

    let from_vec = Pepper::from(vec![1u8, 2, 3]);
    assert_eq!(from_vec.as_ref(), &[1u8, 2, 3]);

    let from_slice = Pepper::from(&[4u8, 5, 6][..]);
    assert_eq!(from_slice.as_ref(), &[4u8, 5, 6]);

    assert_eq!(Pepper::generate().as_ref().len(), 32);
    assert_eq!(Pepper::generate_with_len(64).as_ref().len(), 64);
}

#[test]
fn salt_is_32_bytes() {
    assert_eq!(Salt::generate().as_ref().len(), 32);
}
