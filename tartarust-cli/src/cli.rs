use clap::{Parser, Subcommand};

/// The `Cli` structure, that define the command-line tool. All the commands are in `Commands`.
#[derive(Parser)]
#[command(name = "tartarus")]
#[command(author = "Tartarus' contributors")]
#[command(version = "1.0")]
#[command(about = "Experimental hashing algorithm.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}


/// The `subcommands` of tartarus-cli. We have :
/// 
/// - Hash : hashes a password, with the password and optionally the salt as arguments.
/// 
/// - Verify : takes a password, the stored hash and the stored salt as arguments, and verifies if the password is correct or not.
/// 
/// - Params : Changes the saved parameters (memory usage, number of iterations).
#[derive(Subcommand)]
pub enum Commands {
    /// Hash a password
    Hash {
        /// Password to hash
        #[arg(short = 'p', long = "password")]
        password: String,

        /// The salt the value of the salt will determine which function will be used :
        /// If you don't provide a salt, it will generate one randomly.
        /// If you provide one, the one you provided will be used.
        #[arg(short = 's', long = "salt")]
        salt: Option<String>,
    },

    /// Verify a password against a stored hash
    Verify {
        /// Password to verify
        #[arg(short = 'p', long = "password")]
        password: String,

        /// The salt of the stored hash.
        #[arg(short = 's', long = "salt")]
        salt: String,

        /// The stored `hash`.
        #[arg(short = 'H', long = "hash")]
        hash: String
    },

    /// Show or configure hashing parameters
    Params {
        /// Changes the memory usage (Memory cost in MiB)
        #[arg(short = 'm', long = "memory", value_name = "MiB")]
        memory: Option<u32>,

        /// Changes the number of iterations (Number of memory-hard iterations)
        #[arg(long = "iterations", visible_alias = "it", value_name = "ITER")]
        iterations: Option<u32>,
    }
}