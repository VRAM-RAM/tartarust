# Tartarust-core

Tartarus is an experimental hashing algorithm, originally written in C, and built to be memory-hard.

## Warning

The `Tartarus` scheme is experimental, has never been nor audited nor studied by experts, and both `C` and `Rust` implementations are also unaudited. For a production use, consider using *Argon2id*.

## About

This crate exposes the primitives for `Tartarus`, such as `round functions`, `Memory Pool`, `hashing`, etc. For more informations, please read the doc.

## Example

```rust
use tartarust_core::{errors::TartarusError, hash::{core::tartarus, verify_hash}};

pub fn hash(data: &[u8]) -> Result<Vec<u8>, TartarusError> {
    let mut pepper = [0u8; 32];
    OsRng.fill_bytes(&mut pepper);

    let mut salt = [0u8; 32];
    OsRng.fill_bytes(&mut salt);

    let digest = tartarus(data, , pepper, 12, 3)?;
    Ok(digest)
}
```

## License 

Tartarus is licensed under MIT License.

## Repository 

For the `Tartarus` repository, click [here](https://github.com/KenzoPortela/tartarus-hasher).