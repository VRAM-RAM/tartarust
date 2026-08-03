/// Crate that provides and exposes functions for using `tartarus` hashing algorithm.
/// `generation.rs` contains the `Salt` and the `Pepper` structures and all their methods.
/// `params.rs` contains `TartarusParams`, a structure for managing parameters.
/// `hash.rs` contains `Hash`, a structure that stores the digest and the salt, but also `verify_code()`, `hash()`, and `hash_with_custom_salt()`.
pub mod hash;
pub mod params;
pub mod generation;


