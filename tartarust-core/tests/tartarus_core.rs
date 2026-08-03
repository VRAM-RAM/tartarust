use tartarust_core::hash::core::tartarus;
use tartarust_core::hash::verify_hash::verify_hash;

const DATA: &[u8] = b"Correct Horse Battery Staple";
const SALT: &[u8] = b"0123456789abcdef";
const PEPPER: &[u8] = b"test_pepper_32_bytes_0123456789abcdef";

fn hash_with(memory: u32, iterations: u32) -> Vec<u8> {
    tartarus(DATA, SALT, PEPPER, memory, iterations).unwrap()
}

fn decode_hex(hex_str: &str) -> Vec<u8> {
    hex_str
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let hi = (pair[0] as char).to_digit(16).unwrap();
            let lo = (pair[1] as char).to_digit(16).unwrap();
            ((hi << 4) | lo) as u8
        })
        .collect()
}

#[test]
fn matches_c_reference_known_answer() {
    let expected =
        "4d12b22400fe9f07918c5ece2d91ee2a7a7a89d32e6942a7314c33c0ad9bc3f6834e269a42c7cf434164df5d0ece01f4ac071117e07af0bbe62c317ea110787e";
    let actual = decode_hex(expected);
    assert_eq!(hash_with(4, 1), actual);
}

#[test]
fn output_is_exactly_64_bytes() {
    assert_eq!(hash_with(4, 1).len(), 64);
}

#[test]
fn hash_is_deterministic() {
    assert_eq!(hash_with(4, 1), hash_with(4, 1));
}

#[test]
fn different_data_yields_different_hash() {
    let base = hash_with(4, 1);
    let other = tartarus(b"a different password", SALT, PEPPER, 4, 1).unwrap();
    assert_ne!(base, other);
}

#[test]
fn different_salt_yields_different_hash() {
    let base = hash_with(4, 1);
    let other = tartarus(DATA, b"deadbeefdeadbeef", PEPPER, 4, 1).unwrap();
    assert_ne!(base, other);
}

#[test]
fn different_pepper_yields_different_hash() {
    let base = hash_with(4, 1);
    let other = tartarus(DATA, SALT, b"a_different_pepper_value", 4, 1).unwrap();
    assert_ne!(base, other);
}

#[test]
fn different_memory_yields_different_hash() {
    let base = hash_with(4, 1);
    let other = hash_with(8, 1);
    assert_ne!(base, other);
}

#[test]
fn different_iterations_yields_different_hash() {
    let base = hash_with(4, 1);
    let other = hash_with(4, 2);
    assert_ne!(base, other);
}

#[test]
fn verify_hash_accepts_matching_hashes() {
    let h = hash_with(4, 1);
    assert!(verify_hash(&h, &h));
}

#[test]
fn verify_hash_rejects_different_hashes() {
    let h = hash_with(4, 1);
    let mut tampered = h.clone();
    tampered[0] ^= 1;
    assert!(!verify_hash(&h, &tampered));
}

#[test]
fn verify_hash_rejects_wrong_length() {
    let h = hash_with(4, 1);
    assert!(!verify_hash(&h, &[]));
    assert!(!verify_hash(&h, &[0u8; 63]));
    assert!(!verify_hash(&h, &[0u8; 65]));
}
