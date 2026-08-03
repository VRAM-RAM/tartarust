# Tartarus

> A memory-hard password hashing algorithm and its Rust implementation.

Tartarus is an experimental KDF (Key Derivation Function) based on a memory-hard design intended to increase the computational cost of offline password cracking.

This repository contains :

+ The tartarus algorithm reference implementation (in C)
+ Tartarust, a Rust implementation of Tartarus splitted in three crates.

> [!WARNING] Tartarus is experimental. It isn't audited. For production use, consider using Argon2id.


## Why tartarus ?

General-purpose cryptographic hash functions such as SHA-256 and SHA-512 are intentionally designed to be fast.
This makes them excellent for integrity checking, digital signatures and HMACs, but unsuitable for storing passwords directly.
Password hashing algorithms intentionally trade performance for security by increasing the cost of each password guess.

Tartarus follows this philosophy by combining:

+ configurable memory usage
+ multiple memory-hard passes
+ pseudo-random memory accesses
+ diffusion through repeated block mixing

The objective is to increase the cost of large-scale offline attacks while remaining practical for legitimate users.

## Design

Tartarus is inspired by primitives schemes such as *Argon2id* and *ChaCha20*. The current implementation contains :

- HMAC-SHA512 for initial state derivation
- ChaCha-inspired ARX block mixing
- Configurable memory cost
- Configurable iteration count
- Constant-time verification
- Random salt / pepper generation


## Project structure

```
.
├── tartarus-c       # Original C implementation & original Cli
├── tartarust-core   # Core primitives in Rust
├── tartarust-lib    # Public Rust API 
└── tartarust-cli    # Command line interface (in Rust)
```


## Installation

### Build the C CLI from source

Prerequisites

You need a C compiler (gcc) and the OpenSSL development libraries installed on your system.

+ Linux : `sudo apt install gcc libssl-dev`

+ MacOS : On MacOs, you'll have to run `brew install gcc openssl`.

+ Windows: Use MSYS2 to install `mingw-w64-ucrt-x86_64-gcc` and `mingw-w64-ucrt-x86_64-openssl`.

#### Compilation

Clone the repository and compile the `tartarus_c/tartarus_cli.c` file:

```bash
gcc tartarus_cli.c -o tartarus_cli -O3 -lcrypto -Wno-deprecated-declarations
```

Be aware ! On MacOs, the command is not the same :
```bash
gcc tartarus_cli.c \
    -o tartarus_cli \
    -O3 \
    -I$(brew --prefix openssl)/include \
    -L$(brew --prefix openssl)/lib \
    -lcrypto \
    -Wno-deprecated-declarations
```

(Note: `-Wno-deprecated-declarations` is used to silence OpenSSL 3.0 transition warnings regarding HMAC structures, ensuring a clean compilation).


### Build the Rust CLI from source 

```bash
git clone https://github.com/KenzoPortela/tartarus-hasher
cd tartarus-hasher && cd tartarust-cli
cargo build --release
cargo install --path .
```


# CLI

## Rust CLI :

Generate a password hash:

```bash
tartarust-cli hash --password "correct horse battery staple"
```

Verify a password:

```bash
tartarust-cli verify -p "correct horse battery staple" -s <salt> -H <hash>
```

The CLI expects the pepper to be provided through the `TARTARUS_PEPPER` environment variable.

Linux/macOS

```bash
export TARTARUS_PEPPER="your-secret-pepper"
```

Windows (PowerShell)

```powershell
$env:TARTARUS_PEPPER="your-secret-pepper"
```

For more informations about the Rust Cli, please refeer to the [readme](./tartarust-cli/README.md).

## C CLI :

Hash a password :

```bash
./tartarus_cli hash "MySuperSecretPassword123!"
```

Verify a password :

```bash
./tartarus_cli verify "MySuperSecretPassword123!" "7a8b9c0d..." "629c2635..."
```

## Rust Library

`tartarust-lib` provides functions for using **Tartarus** in your Rust code. For example :

```rust
use tartarust_lib::hash::{hash, hash_with_custom_salt};
use tartarust_lib::params::TartarusParams;

let params = TartarusParams::recommended();

let hash = hash(
    b"correct horse battery staple",
    params,
)?;
```

Verification:

```rust
assert!(hash.verify(
    b"correct horse battery staple",
    params,
)?);
```

For more informations, please refeer to the [readme](./tartarust-lib/README.md).

## License

**Tartarus** is licensed under [MIT](./LICENSE).