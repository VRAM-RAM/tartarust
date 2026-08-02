# Tartarust-lib

Tartarus is an experimental hashing algorithm, originally written in C, and built to be memory-hard.

## Warning

The `Tartarus` scheme is experimental, has never been nor audited nor studied by experts, and both `C` and `Rust` implementations are also unaudited. For a production use, consider using *Argon2id*.

## About

This crate exposes all the functions, structures for using `Tartarus`, such as `Salt`, `Hash`, `Pepper`, `hash()`, `hash_with_custom_salt()`... for more informations, please refeer to the doc.

## Example

```rust
use tartarust_lib::generation::{Pepper, Salt};
use tartarust_lib::hash::{hash, hash_with_custom_salt};
use tartarust_lib::params::TartarusParams;

fn my_func() {
    let data = b"Correct Horse Battery Staple"; 

    let parameters = TartarusParams::new(12, 3, Pepper::from("test_pepper_32_bytes_0123456789abcdef"));

    let hashed = hash(data, &p).unwrap(); 

    assert!(hashed.verify(data, &p).unwrap());
}
```

## License 

Tartarus is licensed under MIT License.

## Repository 

For the `Tartarus` repository, click [here](https://github.com/KenzoPortela/tartarus-hasher).