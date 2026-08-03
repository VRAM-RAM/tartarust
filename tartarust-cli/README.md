# Tartarust-cli

Tartarus is an experimental hashing algorithm, originally written in C, and built to be memory-hard.

## Warning

The `Tartarus` scheme is experimental, has never been nor audited nor studied by experts, and both `C` and `Rust` implementations are also unaudited. For a production use, consider using *Argon2id*.

## About

This binary provides the cli tool for using `tartarus`. It is based on `tartarust-core` and `tartarust-lib`.

## Installation 

You can either install it with `cargo`, or install it manually :

1. Clone the repository.

2. Run `cargo build --release`

3. Run `cargo install --path .`

4. Use it ! (`tartarus-cli -h` for help)

## Commands 

```text
Usage: tartarust-cli [COMMAND]

Commands:
  hash
  verify
  params
  help    
```

### Hash

```text
Usage: tartarust-cli hash [OPTIONS]

Options:
  -p, --password <PASSWORD>
          Password to hash

  -s, --salt <SALT>
          The `salt` the value of the salt will determine which function will be used :
          
          `salt == None -> hash();`
          
          `salt == Some(String) == hash_with_custom_salt();`

          So, if you don't provide salt, one will be generated.

  -h, --help 
        Print help
```

### Verify password

```text
Usage: tartarust-cli verify --password <PASSWORD> --salt <SALT> --hash <HASH>

Options:
  -p, --password <PASSWORD>  Password to verify
  -s, --salt <SALT>          The `salt` of the stored hash
  -H, --hash <HASH>          The stored `hash`
  -h, --help                 Print help
```

### Params

```text
Usage: tartarust-cli params [OPTIONS]

Options:
  -m, --memory <MiB>       Changes the memory usage (in MiB)
      --iterations <ITER>  Changes the number of iterations [alias: --it]
  -h, --help               Print help

  If you don't provide option, it will just show the current parameters.
```

## License 

Tartarus is licensed under MIT License.

## Repository 

For the `Tartarus` repository, click [here](https://github.com/KenzoPortela/tartarus-hasher).